#![feature(box_patterns)]

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

mod alias;
mod basetype;
mod bitflag;
mod bitmask;
mod command;
mod r#const;
mod r#enum;
mod expr;
mod extension;
mod funcpointer;
mod handle;
mod opaque;
mod origin;
mod reference;
mod r#struct;
mod symbols;
mod tag;
mod ty;
mod r#union;
mod vendor;

pub use alias::*;
pub use basetype::*;
pub use bitflag::*;
pub use bitmask::*;
pub use command::*;
pub use expr::*;
pub use extension::*;
pub use funcpointer::*;
pub use handle::*;
pub use opaque::*;
pub use origin::*;
pub use r#const::*;
pub use r#enum::*;
pub use r#struct::*;
pub use r#union::*;
pub use reference::*;
pub use symbols::*;
pub use tag::*;
pub use ty::*;
pub use vendor::*;

/// The set of elements defined in the Vulkan specifications, pre-processed
/// for easier code generation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Source<'a> {
    /// Global symbol table
    #[serde(borrow = "'a")]
    pub global: SymbolTable<'a, (Cow<'a, str>, String, SourceType, usize)>,

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
    pub bitflags: SymbolTable<'a, Bitflag<'a>>,

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

    /// Functions defined on the base loader
    pub loader_functions: SymbolTable<'a, Cow<'a, str>>,
}

impl<'a> Source<'a> {
    /// Finds a value defined in the Vulkan spefification and returns it if it exists.
    #[inline]
    pub fn find(&self, name: &str) -> Option<Ref<'a, '_>> {
        if let Some((_, _, ty, idx)) = self.global.get_by_either(name) {
            match ty {
                SourceType::Vendor => self.vendors.get(*idx).map(Ref::Vendor),
                SourceType::Extension => self.extensions.get(*idx).map(Ref::Extension),
                SourceType::Tag => self.tags.get(*idx).map(Ref::Tag),
                SourceType::Opaque => self.opaque_types.get(*idx).map(Ref::OpaqueType),
                SourceType::Alias => self.aliases.get(*idx).map(Ref::Alias),
                SourceType::Struct => self.structs.get(*idx).map(Ref::Struct),
                SourceType::Union => self.unions.get(*idx).map(Ref::Union),
                SourceType::Handle => self.handles.get(*idx).map(Ref::Handle),
                SourceType::FunctionPointer => self.funcpointers.get(*idx).map(Ref::FunctionPointer),
                SourceType::BaseType => self.basetypes.get(*idx).map(Ref::Basetype),
                SourceType::Bitmask => self.bitmasks.get(*idx).map(Ref::Bitmask),
                SourceType::Constant => self.constants.get(*idx).map(Ref::Const),
                SourceType::ConstantAlias => self.constant_aliases.get(*idx).map(Ref::ConstAlias),
                SourceType::Bitflag => self.bitflags.get(*idx).map(Ref::Bitflag),
                SourceType::Enum => self.enums.get(*idx).map(Ref::Enum),
                SourceType::CommandAlias => self.command_aliases.get(*idx).map(Ref::CommandAlias),
                SourceType::Function => self.functions.get(*idx).map(Ref::Function),
                SourceType::Command => self.commands.get(*idx).map(|f| Ref::Function(f)),
            }
        } else if let Some(v) = self.origins.get_by_either(name) {
            Some(Ref::Origin(v))
        } else {
            None
        }
    }

    /// Finds a value defined in the Vulkan spefification and returns a mutable reference it if it
    /// exists.
    #[inline]
    pub fn find_mut(&mut self, name: &str) -> Option<RefMut<'a, '_>> {
        if let Some((_, _, ty, idx)) = self.global.get_by_either(name) {
            match ty {
                SourceType::Vendor => self.vendors.get_mut(*idx).map(RefMut::Vendor),
                SourceType::Extension => self.extensions.get_mut(*idx).map(RefMut::Extension),
                SourceType::Tag => self.tags.get_mut(*idx).map(RefMut::Tag),
                SourceType::Opaque => self.opaque_types.get_mut(*idx).map(RefMut::OpaqueType),
                SourceType::Alias => self.aliases.get_mut(*idx).map(RefMut::Alias),
                SourceType::Struct => self.structs.get_mut(*idx).map(RefMut::Struct),
                SourceType::Union => self.unions.get_mut(*idx).map(RefMut::Union),
                SourceType::Handle => self.handles.get_mut(*idx).map(RefMut::Handle),
                SourceType::FunctionPointer => self.funcpointers.get_mut(*idx).map(RefMut::FunctionPointer),
                SourceType::BaseType => self.basetypes.get_mut(*idx).map(RefMut::Basetype),
                SourceType::Bitmask => self.bitmasks.get_mut(*idx).map(RefMut::Bitmask),
                SourceType::Constant => self.constants.get_mut(*idx).map(RefMut::Const),
                SourceType::ConstantAlias => self.constant_aliases.get_mut(*idx).map(RefMut::ConstAlias),
                SourceType::Bitflag => self.bitflags.get_mut(*idx).map(RefMut::Bitflag),
                SourceType::Enum => self.enums.get_mut(*idx).map(RefMut::Enum),
                SourceType::CommandAlias => self.command_aliases.get_mut(*idx).map(RefMut::CommandAlias),
                SourceType::Function => self.functions.get_mut(*idx).map(RefMut::Function),
                SourceType::Command => self.commands.get_mut(*idx).map(|f| RefMut::Function(f)),
            }
        } else if let Some(v) = self.origins.get_mut_by_either(name) {
            Some(RefMut::Origin(v))
        } else {
            None
        }
    }

    /// Finds a value defined in the Vulkan spefification and returns it if it exists.
    /// Only applies for types
    pub fn find_type(&self, name: &str) -> Option<TypeRef<'a, '_>> {
        self.find(name).and_then(Ref::as_type_ref)
    }

    /// Resolves a chain of aliases to the original reference.
    ///
    /// This **never** returns [`Ref::Alias`], [`Ref::ConstAlias`], [`Ref::CommandAlias`]
    #[inline]
    pub fn resolve(&self, name: &str) -> Option<Ref<'a, '_>> {
        let ref_ = self.find(name)?;

        match ref_ {
            Ref::Alias(alias) => self.resolve(alias.of()),
            Ref::ConstAlias(alias) => self.resolve(alias.of()),
            Ref::CommandAlias(alias) => self.resolve(alias.of()),
            _ => Some(ref_),
        }
    }

    /// Resolves a chain of aliases to the original type.
    ///
    /// This **never** returns [`TypeRef::Alias`]
    #[inline]
    pub fn resolve_type(&self, name: &str) -> Option<TypeRef<'a, '_>> {
        let ref_ = self.find(name)?;
        let ty = ref_.as_type_ref()?;

        match ty {
            TypeRef::Alias(alias) => self.resolve_type(alias.of()),
            _ => Some(ty),
        }
    }

    pub fn global(&self) -> &SymbolTable<'a, (Cow<'a, str>, String, SourceType, usize)> {
        &self.global
    }

    pub fn vendors(&self) -> &SymbolTable<'a, Vendor<'a>> {
        &self.vendors
    }

    pub fn extensions(&self) -> &SymbolTable<'a, Extension<'a>> {
        &self.extensions
    }

    pub fn tags(&self) -> &SymbolTable<'a, Tag<'a>> {
        &self.tags
    }

    pub fn opaque_types(&self) -> &SymbolTable<'a, OpaqueType<'a>> {
        &self.opaque_types
    }

    pub fn aliases(&self) -> &SymbolTable<'a, Alias<'a>> {
        &self.aliases
    }

    pub fn structs(&self) -> &SymbolTable<'a, Struct<'a>> {
        &self.structs
    }

    pub fn unions(&self) -> &SymbolTable<'a, Union<'a>> {
        &self.unions
    }

    pub fn handles(&self) -> &SymbolTable<'a, Handle<'a>> {
        &self.handles
    }

    pub fn funcpointers(&self) -> &SymbolTable<'a, FunctionPointer<'a>> {
        &self.funcpointers
    }

    pub fn basetypes(&self) -> &SymbolTable<'a, Basetype<'a>> {
        &self.basetypes
    }

    pub fn bitmasks(&self) -> &SymbolTable<'a, Bitmask<'a>> {
        &self.bitmasks
    }

    pub fn constants(&self) -> &SymbolTable<'a, Const<'a>> {
        &self.constants
    }

    pub fn constant_aliases(&self) -> &SymbolTable<'a, ConstAlias<'a>> {
        &self.constant_aliases
    }

    pub fn bitflags(&self) -> &SymbolTable<'a, Bitflag<'a>> {
        &self.bitflags
    }

    pub fn enums(&self) -> &SymbolTable<'a, Enum<'a>> {
        &self.enums
    }

    pub fn command_aliases(&self) -> &SymbolTable<'a, CommandAlias<'a>> {
        &self.command_aliases
    }

    pub fn functions(&self) -> &SymbolTable<'a, Function<'a>> {
        &self.functions
    }

    pub fn commands(&self) -> &SymbolTable<'a, Command<'a>> {
        &self.commands
    }

    pub fn origins(&self) -> &SymbolTable<'a, Origin<'a>> {
        &self.origins
    }

    pub fn loader_functions(&self) -> &SymbolTable<'a, Cow<'a, str>> {
        &self.loader_functions
    }

    pub fn set_vendors(&mut self, vendors: SymbolTable<'a, Vendor<'a>>) {
        self.vendors = vendors;
    }

    pub fn vendors_mut(&mut self) -> &mut SymbolTable<'a, Vendor<'a>> {
        &mut self.vendors
    }

    pub fn extensions_mut(&mut self) -> &mut SymbolTable<'a, Extension<'a>> {
        &mut self.extensions
    }

    pub fn tags_mut(&mut self) -> &mut SymbolTable<'a, Tag<'a>> {
        &mut self.tags
    }

    pub fn global_mut(&mut self) -> &mut SymbolTable<'a, (Cow<'a, str>, String, SourceType, usize)> {
        &mut self.global
    }

    pub fn opaque_types_mut(&mut self) -> &mut SymbolTable<'a, OpaqueType<'a>> {
        &mut self.opaque_types
    }

    pub fn aliases_mut(&mut self) -> &mut SymbolTable<'a, Alias<'a>> {
        &mut self.aliases
    }

    pub fn structs_mut(&mut self) -> &mut SymbolTable<'a, Struct<'a>> {
        &mut self.structs
    }

    pub fn unions_mut(&mut self) -> &mut SymbolTable<'a, Union<'a>> {
        &mut self.unions
    }

    pub fn handles_mut(&mut self) -> &mut SymbolTable<'a, Handle<'a>> {
        &mut self.handles
    }

    pub fn funcpointers_mut(&mut self) -> &mut SymbolTable<'a, FunctionPointer<'a>> {
        &mut self.funcpointers
    }

    pub fn basetypes_mut(&mut self) -> &mut SymbolTable<'a, Basetype<'a>> {
        &mut self.basetypes
    }

    pub fn bitmasks_mut(&mut self) -> &mut SymbolTable<'a, Bitmask<'a>> {
        &mut self.bitmasks
    }

    pub fn constants_mut(&mut self) -> &mut SymbolTable<'a, Const<'a>> {
        &mut self.constants
    }

    pub fn constant_aliases_mut(&mut self) -> &mut SymbolTable<'a, ConstAlias<'a>> {
        &mut self.constant_aliases
    }

    pub fn bitflags_mut(&mut self) -> &mut SymbolTable<'a, Bitflag<'a>> {
        &mut self.bitflags
    }

    pub fn enums_mut(&mut self) -> &mut SymbolTable<'a, Enum<'a>> {
        &mut self.enums
    }

    pub fn command_aliases_mut(&mut self) -> &mut SymbolTable<'a, CommandAlias<'a>> {
        &mut self.command_aliases
    }

    pub fn functions_mut(&mut self) -> &mut SymbolTable<'a, Function<'a>> {
        &mut self.functions
    }

    pub fn commands_mut(&mut self) -> &mut SymbolTable<'a, Command<'a>> {
        &mut self.commands
    }

    pub fn origins_mut(&mut self) -> &mut SymbolTable<'a, Origin<'a>> {
        &mut self.origins
    }

    pub fn loader_functions_mut(&mut self) -> &mut SymbolTable<'a, Cow<'a, str>> {
        &mut self.loader_functions
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SourceType {
    Vendor,
    Extension,
    Tag,
    Opaque,
    Alias,
    Struct,
    Union,
    Handle,
    FunctionPointer,
    BaseType,
    Bitmask,
    Constant,
    ConstantAlias,
    Bitflag,
    Enum,
    CommandAlias,
    Function,
    Command,
}

impl<'a> SymbolName<'a> for (Cow<'a, str>, String, SourceType, usize) {
    fn name(&self) -> Cow<'a, str> {
        self.0.clone()
    }

    fn pretty_name(&self) -> String {
        self.1.clone()
    }
}

/// An object that has queryable children
pub trait Queryable<'a> {
    /// Find the owned value
    fn find<'b>(&'b self, source: &'b Source<'a>, name: &str) -> Option<&'b str>;
}

impl<'a> Queryable<'a> for () {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}
