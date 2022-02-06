//! # Source
//! Processed Vulkan elements to prepare code generation

mod alias;
mod basetypes;
mod bitflags;
mod bitmasks;
mod commands;
mod consts;
mod enums;
mod extensions;
mod funcpointers;
mod handles;
mod opaque;
mod structs;
mod tag;
mod unions;
mod vendors;

use std::borrow::Cow;

use ahash::AHashSet;
use convert_case::{Case, Casing};
use smallvec::SmallVec;
use tracing::{debug, error, info, span, warn, Level};
use vk_parse::{
    CommandDefinition, Commands, CommentedChildren, EnumSpec, Enums, EnumsChild, ExtensionChild, Feature,
    InterfaceItem, Registry, RegistryChild, Type, TypeCodeMarkup, TypeMember, TypeSpec, TypesChild, VendorId,
};

use crate::{
    expr::Expr,
    name::{const_name, funcpointer_name, tag_of_type, type_name},
    origin::Origin,
    symbols::SymbolTable,
    ty::{Mutability, Ty},
};

pub use self::{
    alias::Alias,
    basetypes::Basetype,
    bitflags::{Bit, BitFlags},
    bitmasks::Bitmask,
    commands::{BufferLevelFlags, Command, CommandAlias, Function, FunctionArgument, QueueFlags, RenderpassFlags},
    consts::{Const, ConstAlias},
    enums::Enum,
    extensions::{DeprecationStatus, Extension, ExtensionType},
    funcpointers::{FunctionPointer, FunctionPointerArgument},
    handles::Handle,
    opaque::OpaqueType,
    structs::{Field, Struct},
    tag::Tag,
    unions::Union,
    vendors::Vendor,
};

/// The set of elements defined in the Vulkan specifications, pre-processed
/// for easier code generation.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Source<'a> {
    /// The vendors defined in the Vulkan specification
    pub vendors: SymbolTable<'a, Vendor<'a>>,

    /// The extensions defined in the Vulkan specification
    pub extensions: SymbolTable<'a, Extension<'a>>,

    /// The tags defined in the Vulkan specification
    pub tags: SymbolTable<'a, Tag<'a>>,

    /// The opaque types defined in the Vulkan specification
    pub opaque_types: SymbolTable<'a, OpaqueType<'a>>,

    /// The type aliases defined in the Vulkan specification
    pub aliases: SymbolTable<'a, Alias<'a>>,

    /// The structs defined in the Vulkan specification
    pub structs: SymbolTable<'a, Struct<'a>>,

    /// The unions defined in the Vulkan specification
    pub unions: SymbolTable<'a, Union<'a>>,

    /// The handles defined in the Vulkan specification
    pub handles: SymbolTable<'a, Handle<'a>>,

    /// The function pointers defined in the Vulkan specification
    pub funcpointers: SymbolTable<'a, FunctionPointer<'a>>,

    /// The base types defined in the Vulkan specification
    pub basetypes: SymbolTable<'a, Basetype<'a>>,

    /// The bitmasks defined in the Vulkan specification
    pub bitmasks: SymbolTable<'a, Bitmask<'a>>,

    /// The constants defined in the Vulkan specification
    pub constants: SymbolTable<'a, Const<'a>>,

    /// The constant aliases defined in the Vulkan specification
    pub constant_aliases: SymbolTable<'a, ConstAlias<'a>>,

    /// The bitflags defined in the Vulkan specification
    pub bitflags: SymbolTable<'a, BitFlags<'a>>,

    /// The enums defined in the Vulkan specification
    pub enums: SymbolTable<'a, Enum<'a>>,

    /// The command/function aliases defined in the Vulkan specification
    pub command_aliases: SymbolTable<'a, CommandAlias<'a>>,

    /// The functions defined in the Vulkan specification
    pub functions: SymbolTable<'a, Function<'a>>,

    /// The commands defined in the Vulkan specification
    pub commands: SymbolTable<'a, Command<'a>>,

    /// The origins (versions & extensions) defined in the Vulkan specification
    pub origins: SymbolTable<'a, Origin<'a>>,
}

impl<'a> Source<'a> {
    /// Processes a registry into a set of symbol tables.
    #[inline]
    pub fn new(registry: &'a Registry) -> Self {
        let mut this = Self::default();

        for child in &registry.0 {
            match child {
                RegistryChild::VendorIds(vendor) => this.vendors(vendor),
                RegistryChild::Platforms(_) => {},
                RegistryChild::Tags(tags) => this.tags(tags),
                RegistryChild::Types(types) => this.types(types),
                RegistryChild::Enums(enums) => this.enums(enums),
                RegistryChild::Commands(commands) => this.commands(commands),
                RegistryChild::Feature(feature) => this.assign_origin_version(feature),
                RegistryChild::Extensions(extensions) => this.extensions(extensions),

                // TODO: deal with formats
                RegistryChild::Formats(_) => {},

                // TODO: use SPIR-V stuff to generate code for `magritte-shader`
                RegistryChild::SpirvExtensions(_) => {},
                RegistryChild::SpirvCapabilities(_) => {},
                RegistryChild::Comment(comment) => debug!(?comment, "comment"),
                child => error!(?child, "unknown registry child"),
            }
        }

        this
    }

    fn extension(&mut self, extension: &'a vk_parse::Extension) {
        let disabled = extension
            .supported
            .as_ref()
            .map(|s| s == "disabled")
            .unwrap_or_default();

        let name = Cow::Borrowed(&extension.name as &str);
        let id = extension.number.expect("an extension should have an id");

        let span = span!(Level::INFO, "extension", ?name, ?disabled, ?id);
        let _guard = span.enter();

        let ty = if let Some(ext_type) = extension.ext_type.as_ref() {
            ExtensionType::from(ext_type)
        } else {
            ExtensionType::Instance
        };

        info!(?ty, "parsed extension type");

        let min_core = extension
            .requires_core
            .as_ref()
            .map(|s| s as &str)
            .map_or(Origin::Vulkan1_0, Origin::from_core);
        info!(?min_core, "parsed minimum core version");

        let required = extension
            .requires
            .as_ref()
            .map_or_else(SmallVec::new, |s| s.split(',').map(Cow::Borrowed).collect());
        info!(?required, "parsed required extension");

        let deprecation_status = DeprecationStatus::new(
            extension.promotedto.as_ref(),
            extension.deprecatedby.as_ref(),
            extension.obsoletedby.as_ref(),
        );

        self.extensions.push(Extension::new(
            name,
            disabled,
            id,
            ty,
            min_core,
            required,
            deprecation_status,
        ));

        self.assign_origin_extension(extension);
    }

    fn extensions(&mut self, extensions: &'a CommentedChildren<vk_parse::Extension>) {
        extensions.children.iter().for_each(|ext| self.extension(ext));
    }

    fn assign_origin_extension(&mut self, extension: &'a vk_parse::Extension) {
        let disabled = extension
            .supported
            .as_ref()
            .map(|s| s == "disabled")
            .unwrap_or_default();
        let origin = Origin::Extension(Cow::Borrowed(&extension.name), extension.number.unwrap(), disabled);

        self.origins.push(origin.clone());

        extension.children.iter().for_each(|child| match child {
            ExtensionChild::Require { items, .. } => items
                .iter()
                .for_each(|item| self.assign_origin_item(origin.clone(), item)),
            _ => {},
        })
    }

    fn assign_origin_item(&mut self, origin: Origin<'a>, item: &'a InterfaceItem) {
        match item {
            InterfaceItem::Type { name, .. } => {
                if let Some(item) = self.structs.get_by_name_mut(name) {
                    info!(?name, "assigned struct");
                    item.set_origin(origin);
                } else if let Some(item) = self.unions.get_by_name_mut(name) {
                    info!(?name, "assigned union");
                    item.set_origin(origin);
                } else if let Some(item) = self.funcpointers.get_by_name_mut(name) {
                    info!(?name, "assigned function pointer");
                    item.set_origin(origin);
                } else if let Some(item) = self.aliases.get_by_name_mut(name) {
                    info!(?name, "assigned type alias");
                    item.set_origin(origin);
                } else if let Some(item) = self.handles.get_by_name_mut(name) {
                    info!(?name, "assigned handle");
                    item.set_origin(origin);
                } else if let Some(item) = self.bitmasks.get_by_name_mut(name) {
                    info!(?name, "assigned bitmask");
                    item.set_origin(origin);
                } else if let Some(item) = self.enums.get_by_name_mut(name) {
                    info!(?name, "assigned enum");

                    item.variants_mut()
                        .iter_mut()
                        .filter(|v| v.origin().is_unknown())
                        .for_each(|v| v.set_origin(origin.clone()));

                    item.aliases_mut()
                        .iter_mut()
                        .filter(|a| a.origin().is_unknown())
                        .for_each(|a| a.set_origin(origin.clone()));

                    item.set_origin(origin);
                } else if let Some(item) = self.bitflags.get_by_name_mut(name) {
                    info!(?name, "assigned bitflags");

                    item.bits_mut()
                        .iter_mut()
                        .filter(|b| b.origin().is_unknown())
                        .for_each(|b| b.set_origin(origin.clone()));

                    item.aliases_mut()
                        .iter_mut()
                        .filter(|a| a.origin().is_unknown())
                        .for_each(|a| a.set_origin(origin.clone()));

                    item.set_origin(origin);
                } else if let Some(item) = self.basetypes.get_by_name_mut(name) {
                    info!(?name, "assigned base type");
                    item.set_origin(origin);
                } else if let Some(item) = self.opaque_types.get_by_name_mut(name) {
                    info!(?name, "assigned opaque type");
                    item.set_origin(origin);
                }
            },
            InterfaceItem::Enum(en) => {
                let name = &en.name;

                match &en.spec {
                    EnumSpec::Alias { alias, extends } => {
                        if let Some(extends) = extends {
                            if let Some(item) = self.bitflags.get_by_name_mut(extends) {
                                info!(?name, ?alias, "added bit to bitflag");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.aliases_mut().push(Alias::new(original_name, name, alias, origin));
                            } else if let Some(item) = self.enums.get_by_name_mut(extends) {
                                info!(?name, ?alias, "added variant to enum");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.aliases_mut().push(Alias::new(original_name, name, alias, origin));
                            } else {
                                error!(?extends, "unknown bitflags/enum");
                            }
                        } else {
                            info!(?name, ?alias, "added constant alias");

                            let original_name = name;
                            let name = const_name(original_name, None);
                            self.constant_aliases
                                .push(ConstAlias::new(original_name, name, alias, origin));
                        }
                    },
                    EnumSpec::Offset {
                        offset,
                        extends,
                        extnumber,
                        dir,
                    } => {
                        let base = extnumber.unwrap_or(origin.id()) - 1;
                        let negative = !dir;

                        let value = if negative { -1 } else { 1 } * (1000000000 + base * 1000 + offset);
                        if let Some(item) = self.bitflags.get_by_name_mut(extends) {
                            info!(?name, "added bit to bitflag");

                            let tag = tag_of_type(item.original_name(), &self.tags[..]);

                            let original_name = name;
                            let name = const_name(original_name, tag);

                            item.bits_mut().push(Bit::new(original_name, name, value, origin));
                        } else if let Some(item) = self.enums.get_by_name_mut(extends) {
                            info!(?name, "added variant to enum");

                            let tag = tag_of_type(item.original_name(), &self.tags[..]);

                            let original_name = name;
                            let name = const_name(original_name, tag);

                            item.variants_mut().push(Bit::new(original_name, name, value, origin));
                        } else {
                            error!(?extends, "unknown bitflags/enum");
                        }
                    },
                    EnumSpec::Bitpos { bitpos, extends } => {
                        let value = 1 << *bitpos;

                        if let Some(extends) = extends {
                            if let Some(item) = self.bitflags.get_by_name_mut(extends) {
                                info!(?name, "added bit to bitflag");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.bits_mut().push(Bit::new(original_name, name, value, origin));
                            } else if let Some(item) = self.enums.get_by_name_mut(extends) {
                                info!(?name, "added variant to enum");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.variants_mut().push(Bit::new(original_name, name, value, origin));
                            } else {
                                error!(?extends, "unknown bitflags/enum");
                            }
                        } else {
                            info!(?name, "added constant alias");

                            let original_name = name;
                            let name = const_name(original_name, None);
                            self.constants.push(Const::new(
                                original_name,
                                name,
                                Ty::u32(),
                                Expr::ConstantInt(value),
                                origin,
                            ));
                        }
                    },
                    EnumSpec::Value { value, extends } => {
                        let value = Expr::new(value);

                        if let Some(extends) = extends {
                            if let Some(item) = self.bitflags.get_by_name_mut(extends) {
                                info!(?name, "added bit to bitflag");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.bits_mut()
                                    .push(Bit::new(original_name, name, value.compute(), origin));
                            } else if let Some(item) = self.enums.get_by_name_mut(extends) {
                                info!(?name, "added variant to enum");

                                let tag = tag_of_type(item.original_name(), &self.tags[..]);

                                let original_name = name;
                                let name = const_name(original_name, tag);

                                item.variants_mut()
                                    .push(Bit::new(original_name, name, value.compute(), origin));
                            } else {
                                error!(?extends, "unknown bitflags/enum");
                            }
                        } else {
                            info!(?name, "added constant alias");

                            let original_name = name;
                            let name = const_name(original_name, None);
                            self.constants.push(Const::new(
                                original_name,
                                name,
                                match &value {
                                    Expr::String(_) => Ty::NullTerminatedString(Mutability::Const),
                                    Expr::ConstantInt(_) => Ty::u32(),
                                    Expr::Constant(name) => {
                                        let other = self.constants.get_by_name(name).expect("unknown other");

                                        other.ty().clone()
                                    },
                                    expr => unreachable!("unsupported constan expr: {:?}", expr),
                                },
                                value,
                                origin,
                            ));
                        }
                    },
                    EnumSpec::None => {
                        if let Some(item) = self.constants.get_by_name_mut(name) {
                            info!(?name, "assigned constant");
                            item.set_origin(origin);
                        } else if let Some(item) = self.constant_aliases.get_by_name_mut(name) {
                            info!(?name, "assigned constant alias");
                            item.set_origin(origin);
                        } else if let Some(item) = self.bitflags.get_by_name_mut(name) {
                            info!(?name, "assigned bitflags");
                            item.set_origin(origin);
                        } else if let Some(item) = self.enums.get_by_name_mut(name) {
                            info!(?name, "assigned enum");
                            item.set_origin(origin);
                        }
                    },
                    other => unreachable!("unknown enum spec: {:?}", other),
                }
            },
            InterfaceItem::Command { name, .. } => {
                if let Some(item) = self.command_aliases.get_by_name_mut(name) {
                    info!(?name, "assigned function/command alias");
                    item.set_origin(origin);
                } else if let Some(item) = self.functions.get_by_name_mut(name) {
                    info!(?name, "assigned function");
                    item.set_origin(origin);
                } else if let Some(item) = self.commands.get_by_name_mut(name) {
                    info!(?name, "assigned command");
                    item.set_origin(origin);
                }
            },
            InterfaceItem::Comment(_) => {},
            other => unreachable!("unsupported interface item: {:?}", other),
        }
    }

    fn assign_origin_version(&mut self, feature: &'a Feature) {
        let version = Origin::from_core(&feature.number);

        let span = span!(Level::INFO, "version", number = ?feature.number);
        let _guard = span.enter();

        self.origins.push(version.clone());

        feature.children.iter().for_each(|child| match child {
            ExtensionChild::Require { items, .. } => items
                .iter()
                .for_each(|item| self.assign_origin_item(version.clone(), item)),
            _ => {},
        });
    }

    fn command(&mut self, def_: &'a CommandDefinition) {
        let original_name = &def_.proto.name;

        let span = span!(Level::INFO, "command", ?original_name);
        let _guard = span.enter();

        let name = funcpointer_name(&original_name, &self.tags[..]);
        info!(?name, "generated rustified name");

        let (_, return_type) = Ty::new(def_.proto.type_name.as_ref().expect("no return type for command"), "");
        info!(?return_type, "parsed return type");

        let success_codes = def_
            .successcodes
            .as_ref()
            .map(|c| c.split(',').map(Cow::Borrowed).collect())
            .unwrap_or_else(SmallVec::<[_; 1]>::new);

        info!(?success_codes, "parsed the success codes");

        let error_codes = def_
            .errorcodes
            .as_ref()
            .map(|c| c.split(',').map(Cow::Borrowed).collect())
            .unwrap_or_else(SmallVec::<[_; 1]>::new);

        info!(?error_codes, "parsed the error codes");

        let code = def_
            .code
            .split("(")
            .nth(1)
            .expect("command without parameters")
            .split(",");

        let arguments = def_
            .params
            .iter()
            .zip(code)
            .map(|(param, code)| FunctionArgument::new(param, Ty::new(code, "").1))
            .collect::<SymbolTable<_>>();

        info!(?arguments, "parsed arguments");

        let function = Function::new_no_origin(original_name, name, return_type, success_codes, error_codes, arguments);

        if let Some(buffer_level) = &def_.cmdbufferlevel {
            info!("function is a command");

            let buffer_level = buffer_level
                .split(',')
                .map(|s| match s {
                    "primary" => BufferLevelFlags::PRIMARY,
                    "secondary" => BufferLevelFlags::SECONDARY,
                    "both" => BufferLevelFlags::BOTH,
                    e => unreachable!("Unknown buffer level: {}", e),
                })
                .fold(BufferLevelFlags::empty(), |a, b| a | b);
            info!(?buffer_level, "parsed buffer level");

            let renderpass = def_
                .renderpass
                .as_ref()
                .map(|s| {
                    s.split(',')
                        .map(|s| match s {
                            "outside" => RenderpassFlags::OUTSIDE,
                            "inside" => RenderpassFlags::INSIDE,
                            "both" => RenderpassFlags::BOTH,
                            e => unreachable!("Unknown renderpass: {}", e),
                        })
                        .fold(RenderpassFlags::empty(), |a, b| a | b)
                })
                .unwrap_or_else(|| RenderpassFlags::empty());
            info!(?renderpass, "parsed renderpass requierments");

            let queues = def_
                .queues
                .as_ref()
                .map(|s| {
                    s.split(',')
                        .map(|s| match s {
                            "transfer" => QueueFlags::TRANSFER,
                            "graphics" => QueueFlags::GRAPHICS,
                            "compute" => QueueFlags::COMPUTE,
                            "decode" => QueueFlags::DECODE,
                            "encode" => QueueFlags::ENCODE,
                            e => unreachable!("Unknown queue: {}", e),
                        })
                        .fold(QueueFlags::empty(), |a, b| a | b)
                })
                .unwrap_or_else(|| QueueFlags::empty());
            info!(?queues, "parsed queue types");

            self.commands
                .push(Command::new(function, renderpass, buffer_level, queues));
        } else {
            self.functions.push(function);
        }
    }

    fn commands(&mut self, commands: &'a Commands) {
        commands.children.iter().for_each(|child| match child {
            vk_parse::Command::Alias {
                name: original_name,
                alias,
            } => {
                let span = span!(Level::INFO, "command alias", ?original_name, ?alias);
                let _guard = span.enter();

                let name = funcpointer_name(original_name, &self.tags[..]);
                info!(?name, "generated rustified name");

                self.command_aliases
                    .push(CommandAlias::new_no_origin(original_name, name, alias));
            },
            vk_parse::Command::Definition(def_) => self.command(def_),
            other => unreachable!("unsupported command type: {:?}", other),
        })
    }

    fn enum_(&mut self, enums: &'a Enums) {
        let original_name = enums.name.as_ref().expect("found nameless flag bits");

        let span = span!(Level::INFO, "flag bits", ?original_name);
        let _guard = span.enter();

        let tag = tag_of_type(original_name, &self.tags[..]);
        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "generated rustified name");

        let mut variants = SymbolTable::new();
        let mut aliases = SymbolTable::new();

        // compute the values of all non-alias bits
        enums.children.iter().for_each(|child| match child {
            EnumsChild::Enum(enum_) => match &enum_.spec {
                EnumSpec::Bitpos { bitpos, .. } => {
                    let span = span!(Level::INFO, "variant", original_name = ?enum_.name, ?bitpos);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    variants.push(Bit::new_no_origin(original_name, name, 1 << *bitpos));
                },
                EnumSpec::Alias { alias, .. } => {
                    let span = span!(Level::INFO, "variant alias", original_name = ?enum_.name, ?alias);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    aliases.push(Alias::new_no_origin(original_name, name, alias));
                },
                EnumSpec::Value { value, .. } => {
                    let span = span!(Level::INFO, "variant", original_name = ?enum_.name, ?value);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    let expr = Expr::new(value);
                    info!(?expr, "computed the value's expression");

                    let value = expr.compute();
                    info!(?value, "computed the value");

                    variants.push(Bit::new_no_origin(original_name, name, value));
                },
                _ => unreachable!("unexpected enum value: {:?}", enum_),
            },
            EnumsChild::Unused(_) => (),
            EnumsChild::Comment(_) => (),
            other => panic!("unknown child type: {:?}", other),
        });

        self.enums
            .push(Enum::new_no_origin(original_name, name, variants, aliases));
    }

    fn flagbits(&mut self, enums: &'a Enums) {
        let original_name = enums.name.as_ref().expect("found nameless flag bits");

        let span = span!(Level::INFO, "flag bits", ?original_name);
        let _guard = span.enter();

        let tag = tag_of_type(original_name, &self.tags[..]);
        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "generated rustified name");

        let width = enums
            .bitwidth
            .unwrap_or(32)
            .checked_div(8)
            .expect("not a multiple of 8 bits");
        info!(?width, "computed the bit width");

        let mut bits = SymbolTable::new();
        let mut aliases = SymbolTable::new();

        // compute the values of all non-alias bits
        enums.children.iter().for_each(|child| match child {
            EnumsChild::Enum(enum_) => match &enum_.spec {
                EnumSpec::Bitpos { bitpos, .. } => {
                    let span = span!(Level::INFO, "bit", original_name = ?enum_.name, ?bitpos);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    bits.push(Bit::new_no_origin(original_name, name, 1 << *bitpos));
                },
                EnumSpec::Alias { alias, .. } => {
                    let span = span!(Level::INFO, "bit alias", original_name = ?enum_.name, ?alias);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    aliases.push(Alias::new_no_origin(original_name, name, alias));
                },
                EnumSpec::Value { value, .. } => {
                    let span = span!(Level::INFO, "bit", original_name = ?enum_.name, ?value);
                    let _guard = span.enter();

                    let original_name = &enum_.name;
                    let name = const_name(original_name, tag);
                    info!(?name, "computed rustified name");

                    let expr = Expr::new(value);
                    info!(?expr, "computed the value's expression");

                    let value = expr.compute();
                    info!(?value, "computed the value");

                    bits.push(Bit::new_no_origin(original_name, name, value));
                },
                _ => unreachable!("unexpected enum value: {:?}", enum_),
            },
            EnumsChild::Unused(_) => (),
            EnumsChild::Comment(_) => (),
            other => panic!("unknown child type: {:?}", other),
        });

        self.bitflags
            .push(BitFlags::new_no_origin(original_name, name, width as _, bits, aliases));
    }

    fn consts(&mut self, enums: &'a Enums) {
        let name = enums.name.as_ref().expect("found nameless enums");

        // special case for dealing with API constants
        let create_fn = if name == "API Constants" {
            |original_name, name, ty, expr| Const::new(original_name, name, ty, expr, Origin::Core)
        } else {
            |original_name, name, ty, expr| Const::new_no_origin(original_name, name, ty, expr)
        };

        enums.children.iter().for_each(|child| match child {
            EnumsChild::Enum(enum_) => match &enum_.spec {
                EnumSpec::Alias { alias, .. } => {
                    let original_name = &enum_.name;

                    let span = span!(Level::INFO, "const alias", ?original_name);
                    let _guard = span.enter();

                    info!(of = ?alias, "processed alias of");

                    let name = const_name(original_name, None);
                    info!(?name, "generated rustified name");

                    self.constant_aliases.push(ConstAlias::new_no_origin(
                        original_name,
                        const_name(original_name, None),
                        alias,
                    ));
                },
                EnumSpec::Value { value, .. } => {
                    let original_name = &enum_.name;

                    let span = span!(Level::INFO, "const", ?original_name);
                    let _guard = span.enter();

                    let name = const_name(original_name, None);
                    info!(?name, "generated rustified name");

                    let ty = Ty::new(enum_.type_suffix.as_ref().unwrap(), "").1;
                    info!(?ty, "parsed type");

                    let expr = Expr::new(value);
                    info!(?expr, "parsed value");

                    self.constants
                        .push(create_fn(original_name, const_name(original_name, None), ty, expr));
                },
                other => unreachable!("const spec: {:?}", other),
            },
            EnumsChild::Unused(_) => (),
            EnumsChild::Comment(_) => (),
            other => panic!("unknown child type: {:?}", other),
        });
    }

    fn enums(&mut self, enums: &'a Enums) {
        let span = span!(Level::INFO, "enums", original_name = ?enums.name);
        let _guard = span.enter();

        // determines the type of enums/consts to generate
        match enums.kind.as_ref().map(|s| s as &str) {
            Some("bitmask") => self.flagbits(enums),
            Some("enum") => self.enum_(enums),
            None => self.consts(enums),
            Some(kind) => panic!("unknown enums kind: `{}`", kind),
        }
    }

    fn type_(&mut self, ty: &'a Type) {
        // treats the special case of opaque types
        if let Some(requires) = &ty.requires {
            if requires.ends_with(".h") {
                let original_name = ty.name.as_ref().expect("missing name for opaque type");
                let name = type_name(original_name, &self.tags[..]);

                let span = span!(Level::INFO, "opaque", ?original_name, ?requires);
                let _guard = span.enter();

                info!("opaque type");

                self.opaque_types
                    .push(OpaqueType::new_no_origin(original_name, name, requires));

                return;
            }
        }

        // process all aliases at once
        if let (Some(original_name), Some(alias)) = (&ty.name, &ty.alias) {
            let span = span!(Level::INFO, "alias", ?original_name, of = ?alias);
            let _guard = span.enter();

            info!("alias");

            self.aliases.push(Alias::new_no_origin(
                original_name,
                type_name(original_name, &self.tags[..]),
                alias,
            ));

            return;
        }

        match ty.category.as_ref().map(|s| s as &str) {
            Some("struct") if ty.name.is_some() => self.struct_(ty),
            Some("union") if ty.name.is_some() => self.union_(ty),
            Some("handle") if ty.name.is_some() => self.handle(ty),
            Some("handle") => self.handle_no_name(ty),
            Some("funcpointer") => self.function_pointer(ty),
            Some("basetype") => self.base_type(ty),
            Some("bitmask") => self.bitmask(ty),
            Some("enum") if ty.name.is_some() => {
                debug!(name = ?ty.name.as_ref().unwrap(), "ignored enum declared in `types`")
            },
            Some("define") => debug!(name = ?ty.name, "ignored a define"),
            Some(category) => error!(
                "unknown category: `{}` (has a name? {})",
                category,
                if ty.name.is_some() { "yes" } else { "no" }
            ),
            None => warn!("no category found for type: {:?}", ty),
        }
    }

    fn bitmask(&mut self, ty: &'a Type) {
        let original_name = if let TypeSpec::Code(code) = &ty.spec {
            code.markup
                .iter()
                .find_map(|s| {
                    if let TypeCodeMarkup::Name(name) = s {
                        Some(name)
                    } else {
                        None
                    }
                })
                .expect("no name in code")
        } else {
            panic!("a bitmask must have either a code snippet");
        };

        let span = span!(Level::INFO, "bitmask", ?original_name, ?ty);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        let bits = ty.bitvalues.as_ref().or(ty.requires.as_ref()).map(|s| s as &str);

        self.bitmasks.push(Bitmask::new_no_origin(original_name, name, bits));
    }

    fn base_type(&mut self, ty: &'a Type) {
        let (original_name, ty) = if let TypeSpec::Code(code) = &ty.spec {
            (
                code.markup
                    .iter()
                    .find_map(|s| {
                        if let TypeCodeMarkup::Name(name) = s {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .expect("no name in code"),
                code.markup
                    .iter()
                    .find_map(|s| {
                        if let TypeCodeMarkup::Type(ty) = s {
                            Some(Ty::new(ty, ""))
                        } else {
                            None
                        }
                    })
                    .map(|(_, ty)| ty)
                    .unwrap_or_else(Ty::void),
            )
        } else {
            panic!("a base type must have a code snippet");
        };

        let span = span!(Level::INFO, "basetype", ?original_name, ?ty);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        self.basetypes.push(Basetype::new_no_origin(original_name, name, ty));
    }

    fn function_pointer(&mut self, ty: &'a Type) {
        // extracts the definition (code) and the name
        let (code, original_name) = if let TypeSpec::Code(code) = &ty.spec {
            (
                code.code.trim(),
                code.markup
                    .iter()
                    .find_map(|s| {
                        if let TypeCodeMarkup::Name(name) = s {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .expect("no name in code"),
            )
        } else {
            panic!("a handle must have either a name or a code snippet");
        };

        let span = span!(Level::INFO, "nameless handle", ?original_name);
        let _guard = span.enter();

        let name = funcpointer_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        let (_, ret) = Ty::new(code.split(' ').nth(1).expect("no return type"), "");
        info!(?ret, "return type parsed");

        let arguments: SymbolTable<_> = code
            .split('(')
            .nth(2)
            .expect("no arguments for function pointer")
            .split(',')
            .map(|elements| Ty::new(elements, ""))
            .map(|(original_name, ty)| {
                let name = original_name.to_case(Case::Snake);
                info!(?original_name, ?ty, ?name, "processed argument");

                FunctionPointerArgument::new(original_name, name, ty)
            })
            .collect();

        info!("arguments parsed");

        self.funcpointers
            .push(FunctionPointer::new_no_origin(original_name, name, arguments));
    }

    fn handle_no_name(&mut self, ty: &'a Type) {
        let original_name = if let TypeSpec::Code(code) = &ty.spec {
            code.markup
                .iter()
                .find_map(|s| {
                    if let TypeCodeMarkup::Name(name) = s {
                        Some(name)
                    } else {
                        None
                    }
                })
                .expect("no name in code")
        } else {
            panic!("a handle must have either a name or a code snippet");
        };

        let span = span!(Level::INFO, "nameless handle", ?original_name);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        let object_ty = ty
            .objtypeenum
            .as_ref()
            .map(|s| s as &str)
            .map(Cow::Borrowed)
            .expect("handle without an object type");
        info!(?object_ty, "object type");

        let parent = ty.parent.as_ref().map(|s| s as &str).map(Cow::Borrowed);
        if let Some(parent) = &parent {
            info!(?parent, "parent");
        }

        self.handles.push(Handle::new_no_origin(original_name, name, parent));
    }

    fn handle(&mut self, ty: &'a Type) {
        let original_name = ty.name.as_ref().expect("missing handle name");

        let span = span!(Level::INFO, "handle", ?original_name);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        let object_ty = ty
            .objtypeenum
            .as_ref()
            .map(|s| s as &str)
            .map(Cow::Borrowed)
            .expect("handle without an object type");
        info!(?object_ty, "object type");

        let parent = ty.parent.as_ref().map(|s| s as &str).map(Cow::Borrowed);
        if let Some(parent) = &parent {
            info!(?parent, "parent");
        }

        self.handles.push(Handle::new_no_origin(original_name, name, parent));
    }

    fn union_(&mut self, ty: &'a Type) {
        let original_name = ty.name.as_ref().expect("missing union name");

        let span = span!(Level::INFO, "union", ?original_name);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        // list the fields of this union
        let fields = match &ty.spec {
            TypeSpec::Members(members) => members
                .iter()
                .filter_map(|m| match m {
                    TypeMember::Definition(def) => Some(def),
                    TypeMember::Comment(_) => None,
                    _ => unreachable!("unknwon type members: {:?}", m),
                })
                .map(Field::from_member)
                .collect::<SymbolTable<_>>(),
            other => unreachable!("unexpected type specification for a struct: {:?}", other),
        };

        info!("field list built");

        self.unions.push(Union::new_no_origin(original_name, name, fields));
    }

    fn struct_(&mut self, ty: &'a Type) {
        let original_name = ty.name.as_ref().expect("missing struct name");

        let span = span!(Level::INFO, "struct", ?original_name);
        let _guard = span.enter();

        let name = type_name(original_name, &self.tags[..]);
        info!(?name, "rustified name built");

        // list the structures this structure extends (pointer chains)
        let extends = ty
            .structextends
            .as_ref()
            .map_or_else(AHashSet::new, |s| s.split(',').map(Cow::Borrowed).collect());

        info!(?extends, "extends");

        let fields = match &ty.spec {
            TypeSpec::Members(members) => members
                .iter()
                .filter_map(|m| match m {
                    TypeMember::Definition(def) => Some(def),
                    TypeMember::Comment(_) => None,
                    _ => unreachable!("unknwon type members: {:?}", m),
                })
                .map(Field::from_member)
                .collect::<SymbolTable<_>>(),
            other => unreachable!("unexpected type specification for a struct: {:?}", other),
        };

        info!("field list built");

        let always_returned = ty
            .returnedonly
            .as_ref()
            .map_or(false, |s| s.eq_ignore_ascii_case("true"));

        info!(?always_returned, "always returned");

        self.structs.push(Struct::new_no_origin(
            original_name,
            name,
            extends,
            always_returned,
            fields,
        ));
    }

    #[inline]
    fn types(&mut self, types: &'a CommentedChildren<TypesChild>) {
        types.children.iter().for_each(|child| match child {
            TypesChild::Type(ty) => self.type_(ty),
            TypesChild::Comment(_) => (),
            ty => panic!("unknown child type: {:?}", ty),
        });
    }

    #[inline]
    fn vendors(&mut self, vendors: &'a CommentedChildren<VendorId>) {
        self.vendors.extend(vendors.children.iter().map(From::from));
    }

    #[inline]
    fn tags(&mut self, tags: &'a CommentedChildren<vk_parse::Tag>) {
        self.tags.extend(tags.children.iter().map(From::from))
    }
}