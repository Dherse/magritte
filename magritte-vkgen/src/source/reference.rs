//! # Reference
//! References for elements of a source

use super::{
    Alias, Basetype, BitFlag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function,
    FunctionPointer, Handle, OpaqueType, Origin, Struct, Tag, Union, Vendor,
};

/// A reference to a Vulkan element
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Ref<'a: 'b, 'b> {
    /// A vendor defined in the Vulkan specification
    Vendor(&'b Vendor<'a>),

    /// An extension defined in the Vulkan specification
    Extension(&'b Extension<'a>),

    /// A tag defined in the Vulkan specification
    Tag(&'b Tag<'a>),

    /// An opaque type defined in the Vulkan specification
    OpaqueType(&'b OpaqueType<'a>),

    /// An alias defined in the Vulkan specification
    Alias(&'b Alias<'a>),

    /// A struct defined in the Vulkan specification
    Struct(&'b Struct<'a>),

    /// A union defined in the Vulkan specification
    Union(&'b Union<'a>),

    /// A handle defined in the Vulkan specification
    Handle(&'b Handle<'a>),

    /// A function pointer defined in the Vulkan specification
    FunctionPointer(&'b FunctionPointer<'a>),

    /// A base type defined in the Vulkan specification
    Basetype(&'b Basetype<'a>),

    /// A bitmask defined in the Vulkan specification
    Bitmask(&'b Bitmask<'a>),

    /// A constant defined in the Vulkan specification
    Const(&'b Const<'a>),

    /// A constant alias defined in the Vulkan specification
    ConstAlias(&'b ConstAlias<'a>),

    /// A bitflag defined in the Vulkan specification
    BitFlag(&'b BitFlag<'a>),

    /// An enum defined in the Vulkan specification
    Enum(&'b Enum<'a>),

    /// A command alias defined in the Vulkan specification
    CommandAlias(&'b CommandAlias<'a>),

    /// A function defined in the Vulkan specification
    Function(&'b Function<'a>),

    /// An origin defined in the Vulkan specification
    Origin(&'b Origin<'a>),
}

impl<'a: 'b, 'b> Ref<'a, 'b> {
    /// Gets a reference to the origin
    #[inline]
    pub const fn origin(&self) -> &'b Origin<'a> {
        match self {
            Self::Vendor(_) => &Origin::Core,
            Self::Extension(r) => r.origin(),
            Self::Tag(_) => &Origin::Core,
            Self::OpaqueType(r) => r.origin(),
            Self::Alias(r) => r.origin(),
            Self::Struct(r) => r.origin(),
            Self::Union(r) => r.origin(),
            Self::Handle(r) => r.origin(),
            Self::FunctionPointer(r) => r.origin(),
            Self::Basetype(r) => r.origin(),
            Self::Bitmask(r) => r.origin(),
            Self::Const(r) => r.origin(),
            Self::ConstAlias(r) => r.origin(),
            Self::BitFlag(r) => r.origin(),
            Self::Enum(r) => r.origin(),
            Self::CommandAlias(r) => r.origin(),
            Self::Function(r) => r.origin(),
            Self::Origin(r) => *r,
        }
    }

    /// Turns a reference into a type reference, returns `None` if it is not a type
    #[inline]
    pub const fn as_type_ref(&self) -> Option<TypeRef<'a, 'b>> {
        match self {
            Ref::OpaqueType(r) => Some(TypeRef::OpaqueType(*r)),
            Ref::Alias(r) => Some(TypeRef::Alias(*r)),
            Ref::Struct(r) => Some(TypeRef::Struct(*r)),
            Ref::Union(r) => Some(TypeRef::Union(*r)),
            Ref::Handle(r) => Some(TypeRef::Handle(*r)),
            Ref::FunctionPointer(r) => Some(TypeRef::FunctionPointer(*r)),
            Ref::Basetype(r) => Some(TypeRef::Basetype(*r)),
            Ref::Bitmask(r) => Some(TypeRef::Bitmask(*r)),
            Ref::BitFlag(r) => Some(TypeRef::BitFlag(*r)),
            Ref::Enum(r) => Some(TypeRef::Enum(*r)),
            Ref::Vendor(_) | Ref::Extension(_) | Ref::Tag(_) | Ref::Const(_) | Ref::ConstAlias(_) | Ref::CommandAlias(_) | Ref::Function(_) | Ref::Origin(_) => None,
        }
    }

    /// Returns true if this reference is an alias
    pub const fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_) | Self::ConstAlias(_) | Self::CommandAlias(_))
    }
}

/// A reference to a Vulkan type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeRef<'a: 'b, 'b> {
    /// An opaque type defined in the Vulkan specification
    OpaqueType(&'b OpaqueType<'a>),

    /// An alias defined in the Vulkan specification
    Alias(&'b Alias<'a>),

    /// A struct defined in the Vulkan specification
    Struct(&'b Struct<'a>),

    /// A union defined in the Vulkan specification
    Union(&'b Union<'a>),

    /// A handle defined in the Vulkan specification
    Handle(&'b Handle<'a>),

    /// A function pointer defined in the Vulkan specification
    FunctionPointer(&'b FunctionPointer<'a>),

    /// A base type defined in the Vulkan specification
    Basetype(&'b Basetype<'a>),

    /// A bitmask defined in the Vulkan specification
    Bitmask(&'b Bitmask<'a>),

    /// A bitflag defined in the Vulkan specification
    BitFlag(&'b BitFlag<'a>),

    /// An enum defined in the Vulkan specification
    Enum(&'b Enum<'a>),
}

impl<'a: 'b, 'b> TypeRef<'a, 'b> {
    /// Gets a reference to the origin
    #[inline]
    pub const fn origin(&self) -> &'b Origin<'a> {
        match self {
            Self::OpaqueType(r) => r.origin(),
            Self::Alias(r) => r.origin(),
            Self::Struct(r) => r.origin(),
            Self::Union(r) => r.origin(),
            Self::Handle(r) => r.origin(),
            Self::FunctionPointer(r) => r.origin(),
            Self::Basetype(r) => r.origin(),
            Self::Bitmask(r) => r.origin(),
            Self::BitFlag(r) => r.origin(),
            Self::Enum(r) => r.origin(),
        }
    }

    /// Turns a type ref into a reference
    #[inline]
    pub const fn as_ref(&self) -> Ref<'a, 'b> {
        match self {
            Self::OpaqueType(r) => Ref::OpaqueType(*r),
            Self::Alias(r) => Ref::Alias(*r),
            Self::Struct(r) => Ref::Struct(*r),
            Self::Union(r) => Ref::Union(*r),
            Self::Handle(r) => Ref::Handle(*r),
            Self::FunctionPointer(r) => Ref::FunctionPointer(*r),
            Self::Basetype(r) => Ref::Basetype(*r),
            Self::Bitmask(r) => Ref::Bitmask(*r),
            Self::BitFlag(r) => Ref::BitFlag(*r),
            Self::Enum(r) => Ref::Enum(*r),
        }
    }

    /// Returns true if this reference is an alias
    pub const fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_))
    }
}