#![feature(box_patterns, box_syntax, io_error_other, string_remove_matches)]

mod expr;
mod externally_synced;
mod name;
mod processor;
mod string;
mod ty;

use expr::parse_expr;
pub use magritte_types::*;

use std::{
    borrow::Cow,
    error::Error,
    hint::unreachable_unchecked,
    io::{self, Cursor},
    ops::{Not, Deref},
};

use externally_synced::externally_synced_new;
use heck::ToSnakeCase;
use name::{bit_name, enum_name, funcpointer_name, function_name, tag_of_type, type_name, const_name};
use processor::process_type;
use ty::{ty_new, ty_with_name};
use vk_parse::{
    EnumSpec, EnumsChild, FatalError, InterfaceItem, RegistryChild, TypeCodeMarkup, TypeMember, TypeMemberDefinition,
    TypeMemberMarkup, TypeSpec, TypesChild, ExtensionChild,
};

pub const EXTENSION_BLOCK_LIST: &[&str] = &["VK_EXT_video", "VK_QCOM", "VK_QNX", "VK_GGP"];

pub fn parse(vulkan: &str) -> Result<Source<'_>, Box<dyn Error>> {
    let cursor = Cursor::new(vulkan);
    let registry = match vk_parse::parse_stream(cursor) {
        Ok((registry, _)) => registry,
        Err(FatalError::IoError(e)) => Err(e)?,
        Err(FatalError::MissingRegistryElement) => Err(io::Error::other("missing registry element"))?,
        Err(e) => Err(io::Error::other(format!("error: {e:?}")))?,
    };

    let mut out = Source::default();

    out.origins_mut().push(Origin::Opaque);

    for child in registry.0 {
        match child {
            RegistryChild::VendorIds(vendor_ids) => {
                vendor_ids.children.into_iter().for_each(|v| out.visit_vendor_id(v));
            },
            RegistryChild::Tags(tags) => tags.children.into_iter().for_each(|t| out.visit_tag(t)),
            RegistryChild::Types(types) => {
                types
                    .children
                    .into_iter()
                    .filter_map(|t| match t {
                        TypesChild::Type(t) => Some(t),
                        _ => None,
                    })
                    .for_each(|t| process_type(&mut out, t));
            },
            RegistryChild::Enums(enums) => match enums.kind.as_deref() {
                Some("bitmask") => out.visit_flagbits(enums),
                Some("enum") => out.visit_enum(enums),
                _ => (),
            },
            RegistryChild::Commands(commands) => commands.children.into_iter().for_each(|child| match child {
                vk_parse::Command::Alias { name, alias } => out.visit_command_alias(name, alias),
                vk_parse::Command::Definition(def) => out.visit_command(def),
                other => unreachable!("unknown command definition: {other:?}"),
            }),
            RegistryChild::Feature(feature) => {
                let version = Origin::from_core(&feature.number);

                out.origins_mut().push(version.clone());

                feature.children.into_iter().for_each(|child| {
                    if let vk_parse::ExtensionChild::Require { items, .. } = child {
                        items
                            .into_iter()
                            .for_each(|item| out.assign_origin(version.clone(), item));
                    }
                });
            },
            RegistryChild::Extensions(extensions) => extensions.children.into_iter().for_each(|ext| out.visit_extension(ext)),

            // TODO: deal with formats, SPIR-V (for magritte-shaders and magritte-rg later) and platforms
            RegistryChild::SpirvExtensions(_)
            | RegistryChild::SpirvCapabilities(_)
            | RegistryChild::Platforms(_)
            | RegistryChild::Formats(_) => (),

            RegistryChild::Comment(_) => (),
            other => Err(io::Error::other(format!("unknown child: {other:?}")))?,
        }
        out.update_symbol_table();
    }

    out.handles.push(Handle {
        original_name: Cow::Borrowed("VkImage"),
        rename: Some(Cow::Borrowed("VkSwapchainImage")),
        name: "SwapchainImage".to_string(),
        parent: Some(Cow::Borrowed("VkSwapchainKHR")),
        dispatchable: false,
        origin: out.extensions.get_by_name("VK_KHR_swapchain").unwrap().origin.clone(),
        functions: SymbolTable::default(),
        destroyer: None,
    });

    out.global.push((
        Cow::Borrowed("VkSwapchainImage"),
        "SwapchainImage".to_string(),
        SourceType::Handle,
        out.handles.len() - 1,
    ));

    *out
        .functions
        .get_by_name_mut("vkGetSwapchainImagesKHR")
        .unwrap()
        .arguments_mut()
        .last_mut()
        .unwrap()
        .ty_mut()
        .as_slice_mut()
        .1
        .as_named_mut() = Cow::Borrowed("VkSwapchainImage");

    out.handles.push(Handle {
        original_name: Cow::Borrowed("VkImageView"),
        rename: Some(Cow::Borrowed("VkSwapchainImageView")),
        name: "SwapchainImageView".to_string(),
        parent: Some(Cow::Borrowed("VkSwapchainImage")),
        dispatchable: false,
        origin: out.extensions.get_by_name("VK_KHR_swapchain").unwrap().origin.clone(),
        functions: SymbolTable::default(),
        destroyer: Some(Cow::Borrowed("vkDestroyImageView")),
    });

    out.global.push((
        Cow::Borrowed("VkSwapchainImageView"),
        "SwapchainImageView".to_string(),
        SourceType::Handle,
        out.handles.len() - 1,
    ));

    let mut func = out.functions.get_by_name("vkCreateImageView").unwrap().clone();
    func.rename = Some(Cow::Borrowed("vkCreateSwapchainImageView"));
    func.name = "create_swapchain_image_view".to_string();
    func.origin = out.extensions.get_by_name("VK_KHR_swapchain").unwrap().origin.clone();
    *func
        .arguments_mut()
        .last_mut()
        .unwrap()
        .ty_mut()
        .as_ptr_mut()
        .1
        .as_named_mut() = Cow::Borrowed("VkSwapchainImageView");

    out.functions.push(func);

    out.global.push((
        Cow::Borrowed("vkCreateSwapchainImageView"),
        "create_swapchain_image_view".to_string(),
        SourceType::Function,
        out.functions.len() - 1,
    ));

    for function in out.functions.iter().chain(out.commands.iter().map(Deref::deref)) {
        let first_arg = &function.arguments()[0];

        match first_arg.ty() {
            Ty::Named(name) => {
                if let Some(handle) = out.handles.get_by_name_mut(name) {
                    handle.add_function(<Function as SymbolName>::name(function));
                } else {
                    out.loader_functions
                        .push(<Function as SymbolName>::name(function));
                }
            },
            _ => {
                out.loader_functions
                    .push(<Function as SymbolName>::name(function));
            },
        }

        const STARTS: &[&str] = &["vkFree", "vkDestroy", "vkRelease"];
        if !STARTS.iter().any(|start| function.original_name().starts_with(*start)) {
            continue;
        }

        let len = function.arguments().len();
        let before_last_arg = function.arguments().get(len.saturating_sub(2)).unwrap();
        let last_arg = function.arguments().last().unwrap();

        match last_arg.ty() {
            Ty::Pointer(Mutability::Const, box Ty::Named(Cow::Borrowed("VkAllocationCallbacks"))) => {
                match before_last_arg.ty() {
                    Ty::Named(name)
                    | Ty::Slice(_, box Ty::Named(name), _)
                    | Ty::Pointer(Mutability::Const, box Ty::Named(name)) => {
                        if let Some(handle) = out.handles.get_by_name_mut(name) {
                            if handle.destroyer().is_some() {
                                continue;
                            }

                            handle.set_destroyer(function.original_name.clone());
                        }
                    },
                    _ => {},
                }
            },
            Ty::Named(name)
            | Ty::Slice(_, box Ty::Named(name), _)
            | Ty::Pointer(Mutability::Const, box Ty::Named(name)) => {
                if let Some(handle) = out.handles.get_by_name_mut(name) {
                    if handle.destroyer().is_some() {
                        continue;
                    }

                    handle.set_destroyer(function.original_name.clone());
                }
            },
            _ => {},
        }
    }

    for alias in &out.command_aliases {
        // ignore disabled aliases
        if alias.origin().is_disabled() {
            continue;
        }

        if let Some(function) = out.functions.get_by_name_mut(alias.of()) {
            function.aliases_mut().push(alias.original_name.clone());
        } else if let Some(command) = out.commands.get_by_name_mut(alias.of()) {
            command.aliases_mut().push(alias.original_name.clone());
        }
    }

    for i in 0..out.structs.len() {
        if out.structs[i].origin().is_disabled() {
            continue;
        }

        for j in 0..out.structs[i].extends().len() {
            let extended = out.structs[i].extends()[j].clone();
            let extender = out.structs[i].original_name.clone();

            out.structs.get_by_name_mut(&extended).unwrap().add_extended(extender);
        }
    }

    Ok(out)
}

pub trait Visitor<'a> {
    fn update_symbol_table(&mut self);

    fn visit_vendor_id(&mut self, vendor: vk_parse::VendorId);

    fn visit_tag(&mut self, tag: vk_parse::Tag);

    fn visit_opaque(&mut self, type_: vk_parse::Type);

    fn visit_alias(&mut self, type_: vk_parse::Type);

    fn visit_struct(&mut self, type_: vk_parse::Type);

    fn visit_union(&mut self, type_: vk_parse::Type);

    fn visit_handle(&mut self, type_: vk_parse::Type);

    fn visit_handle_no_name(&mut self, type_: vk_parse::Type);

    fn function_pointer(&mut self, type_: vk_parse::Type);

    fn visit_base_type(&mut self, type_: vk_parse::Type);

    fn visit_bitmask(&mut self, type_: vk_parse::Type);

    fn visit_enum(&mut self, enums: vk_parse::Enums);

    fn visit_flagbits(&mut self, enums: vk_parse::Enums);

    fn visit_command_alias(&mut self, original_name: String, alias: String);

    fn visit_command(&mut self, command: vk_parse::CommandDefinition);

    fn assign_origin(&mut self, origin: Origin<'static>, item: vk_parse::InterfaceItem);

    fn visit_extension(&mut self, extension: vk_parse::Extension);
}

macro_rules! symbol {
    ($this:ident, $name:ident: $ty:ident) => {
        $this.global.extend($this.$name.iter().enumerate().filter(|(_, v)| v.origin().is_unknown()).map(|(idx, value)| {
            (
                SymbolName::name(value),
                SymbolName::pretty_name(value),
                SourceType::$ty,
                idx,
            )
        }));
    };
    ($this:ident, !$name:ident: $ty:ident) => {
        $this.global.extend($this.$name.iter().enumerate().map(|(idx, value)| {
            (
                SymbolName::name(value),
                SymbolName::pretty_name(value),
                SourceType::$ty,
                idx,
            )
        }));
    };

    ($this:ident, $($name:ident: $ty:ident),*, $(!$name2:ident: $ty2:ident),* $(,)*) => {
        $(
            symbol!{ $this, $name: $ty }
        )*
        $(
            symbol!{ $this, !$name2: $ty2 }
        )*
    };
}

impl<'a> Visitor<'a> for Source<'a> {
    fn update_symbol_table(&mut self) {
        symbol! {
            self,
            commands: Command,
            functions: Function,
            command_aliases: CommandAlias,
            enums: Enum,
            bitflags: Bitflag,
            constant_aliases: ConstantAlias,
            constants: Constant,
            bitmasks: Bitmask,
            basetypes: BaseType,
            funcpointers: FunctionPointer,
            unions: Union,
            handles: Handle,
            structs: Struct,
            aliases: Alias,
            opaque_types: Opaque,
            extensions: Extension,
            !tags: Tag,
            !vendors: Vendor,
        }
    }

    fn visit_vendor_id(&mut self, vendor: vk_parse::VendorId) {
        self.vendors_mut().push(Vendor::new(vendor.name, vendor.id));
    }

    fn visit_tag(&mut self, tag: vk_parse::Tag) {
        self.tags_mut().push(Tag::new(tag.name));
    }

    fn visit_opaque(&mut self, type_: vk_parse::Type) {
        let requires = type_.requires.expect("not an opaque type");
        if requires.ends_with(".h") {
            let name = type_.name.expect("missing a name");

            self.opaque_types_mut()
                .push(OpaqueType::new(name, requires, Origin::Opaque));
        }
    }

    fn visit_alias(&mut self, type_: vk_parse::Type) {
        if let (Some(name), Some(alias)) = (type_.name, type_.alias) {
            let original_name = name;
            let name = type_name(&original_name, self.tags().as_slice());

            self.aliases_mut()
                .push(Alias::new_no_origin(original_name, name, alias));
        }
    }

    fn visit_struct(&mut self, type_: vk_parse::Type) {
        let original_name = type_.name.expect("struct without a name");
        let name = type_name(&original_name, self.tags().as_slice());

        let extends = type_.structextends.as_ref().map_or_else(Vec::new, |s| {
            s.split(',').map(str::to_string).map(Cow::Owned).collect::<Vec<_>>()
        });

        let always_returned = type_
            .returnedonly
            .as_ref()
            .map_or(false, |s| s.eq_ignore_ascii_case("true"));

        let fields = match type_.spec {
            TypeSpec::Members(members) => members
                .into_iter()
                .filter_map(|m| match m {
                    TypeMember::Definition(def) => Some(def),
                    _ => None,
                })
                .map(parse_field)
                .collect::<SymbolTable<_>>(),
            other => unreachable!("unexpected struct specification: {other:?}"),
        };

        self.structs_mut().push(Struct::new_no_origin(
            original_name,
            name,
            extends,
            always_returned,
            fields,
        ));
    }

    fn visit_union(&mut self, type_: vk_parse::Type) {
        let original_name = type_.name.expect("struct without a name");
        let name = type_name(&original_name, self.tags().as_slice());

        let fields = match type_.spec {
            TypeSpec::Members(members) => members
                .into_iter()
                .filter_map(|m| match m {
                    TypeMember::Definition(def) => Some(def),
                    _ => None,
                })
                .map(parse_field)
                .collect::<SymbolTable<_>>(),
            other => unreachable!("unexpected struct specification: {other:?}"),
        };

        self.unions_mut()
            .push(Union::new_no_origin(original_name, name, fields));
    }

    fn visit_handle(&mut self, type_: vk_parse::Type) {
        let original_name = type_.name.expect("handle without a name");
        let name = type_name(&original_name, self.tags().as_slice());

        let parent = type_.parent.map(Cow::Owned);
        let dispatchable = match &type_.spec {
            TypeSpec::Code(code) => !code.code.contains("VK_DEFINE_NON_DISPATCHABLE_HANDLE"),
            other => unreachable!("does not make sense for a handle: {:?}", other),
        };

        self.handles_mut()
            .push(Handle::new_no_origin(original_name, name, dispatchable, parent));
    }

    fn visit_handle_no_name(&mut self, type_: vk_parse::Type) {
        let original_name = if let TypeSpec::Code(code) = type_.spec.clone() {
            code.markup
                .into_iter()
                .find_map(|s| {
                    if let TypeCodeMarkup::Name(name) = s {
                        Some(name)
                    } else {
                        None
                    }
                })
                .expect("no name in code")
        } else {
            unreachable!("a handle must have either a name or a code snippet");
        };

        let name = type_name(&original_name, self.tags().as_slice());

        let parent = type_.parent.map(Cow::Owned);
        let dispatchable = match type_.spec {
            TypeSpec::Code(code) => !code.code.contains("VK_DEFINE_NON_DISPATCHABLE_HANDLE"),
            other => unreachable!("does not make sense for a handle: {:?}", other),
        };

        self.handles_mut()
            .push(Handle::new_no_origin(original_name, name, dispatchable, parent));
    }

    fn function_pointer(&mut self, type_: vk_parse::Type) {
        // extracts the definition (code) and the name
        let (code, original_name) = if let TypeSpec::Code(code) = type_.spec {
            (
                code.code.trim().to_string(),
                code.markup
                    .into_iter()
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
            unreachable!("a handle must have either a name or a code snippet");
        };

        let name = funcpointer_name(&original_name, &self.tags().as_slice());

        let (_, ret) = ty_with_name(code.split(' ').nth(1).expect("no return type"), "");
        let return_type = ret.is_void().not().then(|| ret);

        let mut arguments: SymbolTable<_> = code
            .split('(')
            .nth(2)
            .expect("no arguments for function pointer")
            .split(',')
            .map(|elements| ty_with_name(elements, ""))
            .map(|(original_name, ty)| {
                let name = original_name.to_snake_case();

                FunctionPointerArgument::new(Cow::Owned(original_name), name, ty)
            })
            .collect();

        // in the C-specification, functions that do not take arguments contain a `c_void` argument,
        // this filter those out.
        if arguments.len() == 1 && arguments[0].ty().is_void() {
            arguments.clear();
        }

        self.funcpointers_mut().push(FunctionPointer::new_no_origin(
            original_name,
            name,
            return_type,
            arguments,
        ));
    }

    fn visit_base_type(&mut self, type_: vk_parse::Type) {
        let (original_name, ty) = if let TypeSpec::Code(code) = type_.spec {
            (
                code.markup
                    .clone()
                    .into_iter()
                    .find_map(|s| {
                        if let TypeCodeMarkup::Name(name) = s {
                            Some(name)
                        } else {
                            None
                        }
                    })
                    .expect("no name in code"),
                code.markup
                    .into_iter()
                    .find_map(|s| {
                        if let TypeCodeMarkup::Type(ty) = s {
                            Some(ty_with_name(&ty, ""))
                        } else {
                            None
                        }
                    })
                    .map_or_else(Ty::void, |(_, ty)| ty),
            )
        } else {
            panic!("a base type must have a code snippet");
        };

        let name = type_name(&original_name, self.tags().as_slice());

        self.basetypes_mut()
            .push(Basetype::new_no_origin(original_name, name, ty));
    }

    fn visit_bitmask(&mut self, type_: vk_parse::Type) {
        let original_name = if let TypeSpec::Code(code) = type_.spec {
            code.markup
                .into_iter()
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

        let name = type_name(&original_name, self.tags().as_slice());
        let bits = type_.bitvalues.or(type_.requires);

        self.bitmasks_mut()
            .push(Bitmask::new_no_origin(original_name, name, bits));
    }

    fn visit_enum(&mut self, enums: vk_parse::Enums) {
        let original_name = enums.name.expect("flag bits missing name");
        let tag = tag_of_type(&original_name, self.tags().as_slice());
        let name = type_name(&original_name, self.tags().as_slice());

        let mut variants = SymbolTable::new();
        let mut aliases = SymbolTable::new();

        for child in enums.children {
            match child {
                EnumsChild::Enum(enum_) => {
                    let bit_original_name = enum_.name;
                    let bit_name = enum_name(&original_name, tag, Some(&name));
                    match enum_.spec {
                        EnumSpec::Alias { alias, .. } => {
                            aliases.push(Alias::new_no_origin(bit_original_name, bit_name, alias));
                        },
                        EnumSpec::Bitpos { bitpos, .. } => {
                            variants.push(Bit::new_no_origin(bit_original_name, bit_name, 1 << bitpos));
                        },
                        EnumSpec::Value { value, .. } => {
                            let expr = parse_expr(&value).expect("failed to parse expression").1;
                            variants.push(Bit::new_no_origin(bit_original_name, bit_name, expr.compute()));
                        },
                        other => unreachable!("unexpected enum value: {other:?}"),
                    }
                },
                EnumsChild::Unused(_) | EnumsChild::Comment(_) => (),
                other => unreachable!("unexpected enums value: {other:?}"),
            }
        }

        self.enums_mut()
            .push(Enum::new_no_origin(original_name, name, variants, aliases));
    }

    fn visit_flagbits(&mut self, enums: vk_parse::Enums) {
        let original_name = enums.name.expect("flag bits missing name");
        let tag = tag_of_type(&original_name, self.tags().as_slice());
        let name = type_name(&original_name, self.tags().as_slice());

        let width = enums
            .bitwidth
            .unwrap_or(32)
            .checked_div(8)
            .expect("not a multiple of 8 bits");

        let mut bits = SymbolTable::new();
        let mut aliases = SymbolTable::new();

        for child in enums.children {
            match child {
                EnumsChild::Enum(enum_) => {
                    let bit_original_name = enum_.name;
                    let bit_name = bit_name(&original_name, tag, Some(&name));
                    match enum_.spec {
                        EnumSpec::Alias { alias, .. } => {
                            aliases.push(Alias::new_no_origin(bit_original_name, bit_name, alias));
                        },
                        EnumSpec::Bitpos { bitpos, .. } => {
                            bits.push(Bit::new_no_origin(bit_original_name, bit_name, 1 << bitpos));
                        },
                        EnumSpec::Value { value, .. } => {
                            let expr = parse_expr(&value).expect("failed to parse expression").1;
                            bits.push(Bit::new_no_origin(bit_original_name, bit_name, expr.compute()));
                        },
                        other => unreachable!("unexpected enum value: {other:?}"),
                    }
                },
                EnumsChild::Unused(_) | EnumsChild::Comment(_) => (),
                other => unreachable!("unexpected enums value: {other:?}"),
            }
        }

        self.bitflags_mut().push(Bitflag::new_no_origin(
            original_name,
            name,
            (width & 0xFF) as u8,
            bits,
            aliases,
        ));
    }

    fn visit_command_alias(&mut self, original_name: String, alias: String) {
        let name = function_name(&original_name, self.tags().as_slice());

        self.command_aliases_mut()
            .push(CommandAlias::new_no_origin(original_name, name, alias));
    }

    fn visit_command(&mut self, command: vk_parse::CommandDefinition) {
        let original_name = command.proto.name;
        let name = function_name(&original_name, self.tags().as_slice());

        let (_, return_type) = ty_with_name(command.proto.type_name.as_deref().expect("no return type"), "");

        let success_codes = command.successcodes.map_or_else(Vec::<Cow<str>>::new, |c| {
            c.split(',').map(ToString::to_string).map(Cow::Owned).collect()
        });

        let error_codes = command.errorcodes.map_or_else(Vec::<Cow<str>>::new, |c| {
            c.split(',').map(ToString::to_string).map(Cow::Owned).collect()
        });

        let code = command
            .code
            .split('(')
            .nth(1)
            .expect("command without parameters")
            .split(',');

        let arguments = command
            .params
            .into_iter()
            .zip(code)
            .map(|(param, code)| new_function_argument(param, code))
            .collect::<SymbolTable<_>>();

        let function = Function::new_no_origin(original_name, name, return_type, success_codes, error_codes, arguments);

        if let Some(buffer_level) = command.cmdbufferlevel {
            let buffer_level = buffer_level
                .split(',')
                .map(|s| match s {
                    "primary" => BufferLevelFlags::PRIMARY,
                    "secondary" => BufferLevelFlags::SECONDARY,
                    "both" => BufferLevelFlags::BOTH,
                    e => unreachable!("Unknown buffer level: {}", e),
                })
                .fold(BufferLevelFlags::empty(), |a, b| a | b);

            let renderpass = command.renderpass.map_or_else(RenderpassFlags::empty, |s| {
                s.split(',')
                    .map(|s| match s {
                        "outside" => RenderpassFlags::OUTSIDE,
                        "inside" => RenderpassFlags::INSIDE,
                        "both" => RenderpassFlags::BOTH,
                        e => unreachable!("Unknown renderpass: {}", e),
                    })
                    .fold(RenderpassFlags::empty(), |a, b| a | b)
            });

            let queues = command.queues.map_or_else(QueueFlags::empty, |s| {
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
            });

            self.commands_mut()
                .push(Command::new(function, renderpass, buffer_level, queues));
        } else {
            self.functions_mut().push(function);
        }
    }

    fn assign_origin(&mut self, origin: Origin<'static>, item: vk_parse::InterfaceItem) {
        self.origins_mut().push(origin.clone());

        match item {
            InterfaceItem::Type { name, .. } | InterfaceItem::Command { name, .. } => {
                if let Some(ref_mut) = self.find_mut(&name) {
                    ref_mut.assign_origin(origin);
                }
            },
            InterfaceItem::Enum(enum_) => {
                let original_name = enum_.name;
                match enum_.spec {
                    EnumSpec::Alias { alias, extends } => {
                        if let Some(extends) = extends {
                            if let Some(item) = self.bitflags.get_by_name_mut(&extends) {
                                let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                                let name = bit_name(&original_name, tag, Some(item.name()));
                                let alias = Alias::new(original_name, name, alias, origin);

                                item.aliases_mut().push(alias);
                            } else if let Some(item) = self.enums.get_by_name_mut(&extends) {
                                let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                                let name = enum_name(&original_name, tag, Some(item.name()));
                                let alias = Alias::new(original_name, name, alias, origin);
                                
                                item.aliases_mut().push(alias);
                            } else {
                                unreachable!("unknown bitflag/enum: {extends}");
                            }
                        } else {
                            let name = const_name(&original_name, None);
                            self.constant_aliases_mut()
                                .push(ConstAlias::new(original_name, name, alias, origin));
                        }
                    },
                    EnumSpec::Offset { offset, extends, extnumber, dir } => {
                        let base = extnumber.unwrap_or(origin.id()) - 1;
                        let negative = !dir;
                        let value = if negative { -1 } else { 1 } * (1_000_000_000 + base * 1_000 + offset);
                        if let Some(item) = self.bitflags.get_by_name_mut(&extends) {
                            let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                            let name = bit_name(&original_name, tag, Some(item.name()));
                            let bit = Bit::new(original_name, name, value, origin);

                            item.bits_mut().push(bit);
                        } else if let Some(item) = self.enums.get_by_name_mut(&extends) {
                            let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                            let name = enum_name(&original_name, tag, Some(item.name()));
                            let variant = Bit::new(original_name, name, value, origin);
                            
                            item.variants_mut().push(variant);
                        } else {
                            unreachable!("unknown bitflag/enum: {extends}");
                        }
                    },
                    EnumSpec::Bitpos { bitpos, extends } => if let Some(extends) = extends {
                        let value = 1 << bitpos;
                        if let Some(item) = self.bitflags.get_by_name_mut(&extends) {
                            let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                            let name = bit_name(&original_name, tag, Some(item.name()));
                            let bit = Bit::new(original_name, name, value, origin);

                            item.bits_mut().push(bit);
                        } else if let Some(item) = self.enums.get_by_name_mut(&extends) {
                            let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                            let name = enum_name(&original_name, tag, Some(item.name()));
                            let variant = Bit::new(original_name, name, value, origin);
                            
                            item.variants_mut().push(variant);
                        } else {
                            unreachable!("unknown bitflag/enum: {extends}");
                        }
                    } else {
                        let value = 1 << bitpos;
                        let name = const_name(&original_name, None);
                        self.constants_mut()
                            .push(Const::new(original_name, name, Ty::u32(), Expr::ConstantInt(value), origin));
                    },
                    EnumSpec::Value { value, extends } => {
                        let value = parse_expr(&value).expect("failed to parse expr").1;

                        if let Some(extends) = extends {
                            if let Some(item) = self.bitflags.get_by_name_mut(&extends) {
                                let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                                let name = bit_name(&original_name, tag, Some(item.name()));
                                let bit = Bit::new(original_name, name, value.compute(), origin);
    
                                item.bits_mut().push(bit);
                            } else if let Some(item) = self.enums.get_by_name_mut(&extends) {
                                let tag = tag_of_type(item.original_name(), self.tags.as_slice());
                                let name = enum_name(&original_name, tag, Some(item.name()));
                                let variant = Bit::new(original_name, name, value.compute(), origin);
                                
                                item.variants_mut().push(variant);
                            } else {
                                unreachable!("unknown bitflag/enum: {extends}");
                            }
                        } else {
                            let name = const_name(&original_name, None);
                            self.constants.push(Const::new(
                                original_name,
                                name,
                                match &value {
                                    Expr::String(_) => Ty::NullTerminatedString(Mutability::Const),
                                    Expr::ConstantInt(_) => Ty::u32(),
                                    Expr::Constant(name) => {
                                        let other = self.constants.get_by_name(name).expect("unknown other");

                                        other.ty().clone().as_static()
                                    },
                                    expr => unreachable!("unsupported constan expr: {:?}", expr),
                                },
                                value,
                                origin,
                            ));
                        }
                    },
                    EnumSpec::None => {
                        if let Some(ref_mut) = self.find_mut(&original_name) {
                            ref_mut.assign_origin(origin);
                        }
                    },
                    other => unreachable!("unknown enum spec: {other:?}")
                }
            },
            InterfaceItem::Comment(_) => (),
            other => unreachable!("unexpected interface item: {other:?}"),
        }
    }

    fn visit_extension(&mut self, extension: vk_parse::Extension) {
        let mut disabled = extension
            .supported
            .as_ref()
            .map(|s| s == "disabled")
            .unwrap_or_default();

        if EXTENSION_BLOCK_LIST.iter().any(|s| extension.name.starts_with(s)) {
            disabled = true;
        }

        let name = Cow::Owned(extension.name);
        let id = extension.number.expect("an extension should have an id");

        let ty = if let Some(ext_type) = extension.ext_type.as_ref() {
            ExtensionType::from(ext_type)
        } else {
            ExtensionType::Instance
        };

        let min_core = extension
            .requires_core
            .as_ref()
            .map(|s| s as &str)
            .map_or(Origin::Vulkan1_0, Origin::from_core);


        let required = extension
            .requires
            .as_ref()
            .map_or_else(Vec::new, |s| s.split(',').map(ToString::to_string).map(Cow::Owned).collect());


        let deprecation_status = DeprecationStatus::new(
            extension.promotedto.as_ref(),
            extension.deprecatedby.as_ref(),
            extension.obsoletedby.as_ref(),
        );

        let ext = Extension::new(
            name,
            disabled,
            id,
            ty,
            min_core,
            required,
            deprecation_status,
        );

        let origin = ext.origin().clone();

        self.extensions.push(ext);

        extension.children.into_iter().for_each(|child| {
            if let ExtensionChild::Require { items, .. } = child {
                items
                    .into_iter()
                    .for_each(|item| self.assign_origin(origin.clone(), item));
            }
        });
    }
}

fn parse_field(member: TypeMemberDefinition) -> Field<'static> {
    let mut name = None;

    for item in member.markup {
        match item {
            TypeMemberMarkup::Name(value) => name = Some(Cow::Owned(value)),
            TypeMemberMarkup::Enum(_) | TypeMemberMarkup::Type(_) | TypeMemberMarkup::Comment(_) => (),
            _ => unreachable!("Unknown type markup: {:?}", item),
        }
    }

    let mut ty = ty_new(
        &member.code,
        name.as_ref().unwrap(),
        member.altlen.as_ref().or(member.len.as_ref()).map_or("", |s| s as &str),
    )
    .simplify();

    let original_name = name.expect("missing name");
    let mut name = original_name.to_snake_case();

    if name == "type" {
        name = "type_".to_string();
    } else if name == "p_next" && matches!(ty, Ty::Pointer(_, box Ty::Native(Native::Void))) {
        let mutability = match &ty {
            Ty::Pointer(mut_, _) => mut_,
            _ => unsafe { unreachable_unchecked() },
        };

        ty = Ty::Pointer(
            *mutability,
            if mutability.is_mut() {
                box Ty::Named(Cow::Borrowed("VkBaseOutStructure"))
            } else {
                box Ty::Named(Cow::Borrowed("VkBaseInStructure"))
            },
        );
    } else if name.starts_with("p_") {
        name = name.trim_start_matches("p_").to_string();
    }

    Field {
        original_name,
        name,
        ty,
        selector: member.selector.map(Cow::Owned),
        selection: member.selection.map(Cow::Owned),
        optional: match member
            .optional
            .as_ref()
            .map_or((false, false), |s| (s.contains("true"), s.contains("false")))
        {
            (true, false) => Optionality::Yes,
            (true, true) => Optionality::Sometimes,
            (false, _) => Optionality::No,
        },
        externally_synchronized: externally_synced_new(member.externsync),
        must_be_valid: member
            .noautovalidity
            .as_ref()
            .map_or(false, |s| s.eq_ignore_ascii_case("true")),
        value: member.values.map(Cow::Owned),
    }
}

fn new_function_argument(param: vk_parse::CommandParam, code: &str) -> FunctionArgument<'static> {
    let original_name = param.definition.name;

    let mut name = original_name.to_snake_case();
    if name == "type" {
        name = "type_".to_string();
    }

    let len = param.altlen.or(param.len);

    let optionality = match param
        .optional
        .as_ref()
        .map_or((false, false), |s| (s.contains("true"), s.contains("false")))
    {
        (true, false) => Optionality::Yes,
        (true, true) => Optionality::Sometimes,
        (false, _) => Optionality::No,
    };

    let no_auto_validity = param.noautovalidity.as_ref().map_or(false, |v| v == "true");

    let externally_synced = externally_synced_new(param.externsync);

    let (_, ty) = ty_with_name(code, len.as_deref().unwrap_or_else(|| ""));

    FunctionArgument {
        original_name: Cow::Owned(original_name),
        name,
        len: len.map(Cow::Owned),
        optionality,
        no_auto_validity,
        externally_synced,
        ty,
    }
}