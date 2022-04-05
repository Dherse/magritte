//! # Reference
//! References for elements of a source

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Path, PathSegment};
use tracing::warn;

use crate::{doc::Queryable, imports::Imports};

use super::{
    Alias, Basetype, BitFlag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function, FunctionPointer,
    Handle, OpaqueType, Origin, Source, Struct, Tag, Union, Vendor,
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
            Self::Vendor(_) | Self::Tag(_) => &Origin::Core,
            Self::Extension(r) => r.origin(),
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
            Ref::Vendor(_)
            | Ref::Extension(_)
            | Ref::Tag(_)
            | Ref::Const(_)
            | Ref::ConstAlias(_)
            | Ref::CommandAlias(_)
            | Ref::Function(_)
            | Ref::Origin(_) => None,
        }
    }

    /// Returns true if this reference is an alias
    pub const fn is_alias(&self) -> bool {
        matches!(self, Self::Alias(_) | Self::ConstAlias(_) | Self::CommandAlias(_))
    }

    /// Gets the name of the reference
    #[inline]
    pub fn name(&self) -> &'b str {
        match self {
            Self::Vendor(_) => panic!("a vendor cannot be made into a name"),
            Self::Extension(r) => r.name(),
            Self::Tag(_) => panic!("a tag cannot be made into a name"),
            Self::OpaqueType(r) => r.original_name(),
            Self::Alias(r) => r.name(),
            Self::Struct(r) => r.name(),
            Self::Union(r) => r.name(),
            Self::Handle(r) => r.name(),
            Self::FunctionPointer(r) => r.name(),
            Self::Basetype(r) => r.name(),
            Self::Bitmask(r) => r.name(),
            Self::Const(r) => r.name(),
            Self::ConstAlias(r) => r.name(),
            Self::BitFlag(r) => r.name(),
            Self::Enum(r) => r.name(),
            Self::CommandAlias(r) => r.name(),
            Self::Function(r) => r.name(),
            Self::Origin(_) => panic!("an origin cannot be made into a name"),
        }
    }

    /// Gets the name of the reference as an identifier
    #[inline]
    pub fn as_ident(&self) -> Ident {
        match self {
            Self::Vendor(_) => panic!("a vendor cannot be made into an identifier"),
            Self::Extension(r) => r.as_ident(),
            Self::Tag(_) => panic!("a tag cannot be made into an identifier"),
            Self::OpaqueType(r) => r.as_ident(),
            Self::Alias(r) => r.as_ident(),
            Self::Struct(r) => r.as_ident(),
            Self::Union(r) => r.as_ident(),
            Self::Handle(r) => r.as_ident(),
            Self::FunctionPointer(r) => r.as_ident(),
            Self::Basetype(r) => r.as_ident(),
            Self::Bitmask(r) => r.as_ident(),
            Self::Const(r) => r.as_ident(),
            Self::ConstAlias(r) => r.as_ident(),
            Self::BitFlag(r) => r.as_ident(),
            Self::Enum(r) => r.as_ident(),
            Self::CommandAlias(r) => r.as_ident(),
            Self::Function(r) => r.as_ident(),
            Self::Origin(_) => panic!("an origin cannot be made into an identifier"),
        }
    }

    /// Returns the reference as a path
    pub fn as_path(&self) -> Path {
        let mut path = self.origin().as_path();

        if !matches!(self, Self::Extension(_) | Self::Origin(_)) {
            path.segments.push(PathSegment::from(self.as_ident()));
        }

        path
    }

    /// Tries to destructure `self` into a function
    #[inline]
    pub fn as_function(&self) -> Option<&'b Function<'a>> {
        if let Self::Function(fn_) = *self {
            Some(fn_)
        } else {
            None
        }
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
    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_enum(self) -> Option<&'b Enum<'a>> {
        match self {
            TypeRef::Enum(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_bitflag(self) -> Option<&'b BitFlag<'a>> {
        match self {
            TypeRef::BitFlag(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_bitmask(self) -> Option<&'b Bitmask<'a>> {
        match self {
            TypeRef::Bitmask(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_basetype(self) -> Option<&'b Basetype<'a>> {
        match self {
            TypeRef::Basetype(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_function_pointer(self) -> Option<&'b FunctionPointer<'a>> {
        match self {
            TypeRef::FunctionPointer(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_handle(self) -> Option<&'b Handle<'a>> {
        match self {
            TypeRef::Handle(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_union(self) -> Option<&'b Union<'a>> {
        match self {
            TypeRef::Union(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_struct(self) -> Option<&'b Struct<'a>> {
        match self {
            TypeRef::Struct(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_alias(self) -> Option<&'b Alias<'a>> {
        match self {
            TypeRef::Alias(ref_) => Some(ref_),
            _ => None,
        }
    }

    /// Tries to convert the reference into its underlying reference
    #[inline]
    pub const fn as_opaque_type(self) -> Option<&'b OpaqueType<'a>> {
        match self {
            TypeRef::OpaqueType(ref_) => Some(ref_),
            _ => None,
        }
    }

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

    /// Finds the child of an element
    #[inline]

    pub fn find(&self, source: &'b Source<'a>, name: &str) -> Option<&'b str> {
        match self {
            TypeRef::OpaqueType(_) | TypeRef::Handle(_) | TypeRef::Basetype(_) => None,
            TypeRef::Alias(alias) => source
                .find(alias.of())
                .expect("alias missing")
                .as_type_ref()
                .expect("alias is not a type")
                .find(source, name),
            TypeRef::Struct(struct_) => struct_.find(source, name),
            TypeRef::Union(union_) => union_.find(source, name),
            TypeRef::FunctionPointer(function_pointer) => function_pointer.find(source, name),
            TypeRef::Bitmask(mask) => mask
                .bits()
                .and_then(|bit| source.find(bit))
                .and_then(|ref_| ref_.as_type_ref())
                .expect("alias is not a type")
                .find(source, name),
            TypeRef::BitFlag(flags) => flags.find(source, name),
            TypeRef::Enum(enum_) => enum_.find(source, name),
        }
    }

    /// Gets a reference to the name
    #[inline]
    pub fn name(&self) -> &'b str {
        match self {
            Self::OpaqueType(r) => r.original_name(),
            Self::Alias(r) => r.name(),
            Self::Struct(r) => r.name(),
            Self::Union(r) => r.name(),
            Self::Handle(r) => r.name(),
            Self::FunctionPointer(r) => r.name(),
            Self::Basetype(r) => r.name(),
            Self::Bitmask(r) => r.name(),
            Self::BitFlag(r) => r.name(),
            Self::Enum(r) => r.name(),
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

    /// Imports the type
    pub fn import(&self, imports: &Imports) {
        imports.push_origin(self.origin(), self.name());
    }

    /// Gets the type path of this type
    pub fn as_path(&self) -> Path {
        let mut path = self.origin().as_path();

        path.segments.push(PathSegment::from(self.as_ident()));

        path
    }

    /// Gets the identifier associated with this type
    pub fn as_ident(&self) -> Ident {
        match self {
            TypeRef::OpaqueType(r) => r.as_ident(),
            TypeRef::Alias(r) => r.as_ident(),
            TypeRef::Struct(r) => r.as_ident(),
            TypeRef::Union(r) => r.as_ident(),
            TypeRef::Handle(r) => r.as_ident(),
            TypeRef::FunctionPointer(r) => r.as_ident(),
            TypeRef::Basetype(r) => r.as_ident(),
            TypeRef::Bitmask(r) => r.as_ident(),
            TypeRef::BitFlag(r) => r.as_ident(),
            TypeRef::Enum(r) => r.as_ident(),
        }
    }

    /// Does the type have a lifetime
    pub fn has_lifetime(&self, source: &Source<'a>, pointer_has_lifetime: bool) -> bool {
        match self {
            TypeRef::Alias(alias) => source
                .resolve_type(alias.of())
                .expect("unknown alias")
                .has_lifetime(source, pointer_has_lifetime),
            TypeRef::Struct(struct_) => struct_.has_lifetime(source),
            TypeRef::Union(union_) => union_.has_lifetime(source),

            // in the case of function pointers, there is never a lifetime because they are defined internally
            TypeRef::FunctionPointer(_) => false,
            TypeRef::OpaqueType(_)
            | TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_) => false,
        }
    }

    /// Checks if the type is copy
    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => false,
            TypeRef::Alias(alias) => source.resolve_type(alias.of()).expect("unknown alias").is_copy(source),
            TypeRef::Struct(struct_) => struct_.is_copy(source),
            TypeRef::Union(union_) => union_.is_copy(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_)
            | TypeRef::FunctionPointer(_) => true,
        }
    }

    /// Checks if the type is debug
    pub fn is_debug(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::Alias(alias) => source.resolve_type(alias.of()).expect("unknown alias").is_debug(source),
            TypeRef::Struct(struct_) => struct_.is_debug(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_) => true,
            TypeRef::OpaqueType(_) | TypeRef::Union(_) | TypeRef::FunctionPointer(_) => false,
        }
    }

    /// Checks if the type is eq/ord
    pub fn is_partial_eq(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => false,
            TypeRef::Alias(alias) => source
                .resolve_type(alias.of())
                .expect("unknown alias")
                .is_partial_eq(source),
            TypeRef::Struct(struct_) => struct_.is_partial_eq(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_) => true,

            // unions are not eq because we cannot know the variant, would require bitwise comparison
            TypeRef::Union(_) | TypeRef::FunctionPointer(_) => false,
        }
    }

    /// Checks if the type is eq/ord
    pub fn is_eq(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => false,
            TypeRef::Alias(alias) => source.resolve_type(alias.of()).expect("unknown alias").is_eq(source),
            TypeRef::Struct(struct_) => struct_.is_eq(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_) => true,

            // unions are not eq because we cannot know the variant, would require bitwise comparison
            TypeRef::Union(_) | TypeRef::FunctionPointer(_) => false,
        }
    }

    /// Checks if the type is hash
    pub fn is_hash(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => false,
            TypeRef::Alias(alias) => source.resolve_type(alias.of()).expect("unknown alias").is_hash(source),
            TypeRef::Struct(struct_) => struct_.is_hash(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_) => true,

            // unions are not hash because we cannot know the variant
            TypeRef::FunctionPointer(_) | TypeRef::Union(_) => false,
        }
    }

    /// Checks if the type is (de)serializable
    pub(crate) fn is_serde(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => false,
            TypeRef::Alias(alias) => source.resolve_type(alias.of()).expect("unknown alias").is_serde(source),
            TypeRef::Struct(struct_) => struct_.is_serde(source),
            TypeRef::Basetype(_) | TypeRef::Bitmask(_) | TypeRef::BitFlag(_) | TypeRef::Enum(_) => true,
            TypeRef::Handle(_) | TypeRef::FunctionPointer(_) | TypeRef::Union(_) => false,
        }
    }

    /// Creates the default value for this type
    pub fn default_tokens(&self, source: &Source, imports: Option<&Imports>, value: Option<&str>) -> TokenStream {
        match self {
            TypeRef::OpaqueType(opaque) => {
                warn!("Default value for opaque type: {}, {:?}", opaque.original_name(), value);

                quote! {
                    unsafe { std::mem::zeroed() }
                }
            },
            TypeRef::Alias(alias) => source
                .resolve_type(alias.of())
                .expect("unknwon alias")
                .default_tokens(source, imports, value),
            TypeRef::Struct(_) | TypeRef::Handle(_) | TypeRef::Basetype(_) => quote! { Default::default() },
            TypeRef::Enum(enum_) => {
                if let Some(value) = value {
                    let variant = enum_.variants().get_by_either(value).expect("unknown variant");
                    let ident = enum_.as_ident();
                    let variant = variant.as_ident();

                    quote! {
                        #ident::#variant
                    }
                } else {
                    quote! { Default::default() }
                }
            },
            TypeRef::BitFlag(bitflag) => {
                if let Some(value) = value {
                    let variant = bitflag.bits().get_by_either(value).expect("unknown variant");
                    let ident = bitflag.as_ident();
                    let variant = variant.as_ident();

                    quote! {
                        #ident::#variant
                    }
                } else {
                    quote! { Default::default() }
                }
            },
            TypeRef::Bitmask(bitmask) => {
                if let Some(value) = value {
                    let variant = source
                        .resolve_type(bitmask.bits().expect("bitmask without bits"))
                        .expect("unknown type")
                        .as_bitflag()
                        .expect("bits are not a bitflag")
                        .bits()
                        .get_by_either(value)
                        .expect("unknown variant");
                    let ident = bitmask.as_ident();
                    let variant = variant.as_flag_ident();

                    quote! {
                        #ident::#variant
                    }
                } else {
                    quote! { Default::default() }
                }
            },
            TypeRef::Union(_) => quote! {
                unsafe { std::mem::zeroed() }
            },
            TypeRef::FunctionPointer(_) => quote! { None },
        }
    }

    /*/// Does the type have a generic type parameter
    pub fn has_generics(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::Alias(alias) => source
                .resolve_type(alias.of())
                .expect("unknown alias")
                .has_generics(source),
            TypeRef::Struct(struct_) => struct_.has_generics(source),

            // handles are always true because we will be using the `Writable<'a>` and `Readable<'a>` types.
            TypeRef::Handle(_) => true,
            TypeRef::OpaqueType(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::BitFlag(_)
            | TypeRef::Enum(_)
            | TypeRef::FunctionPointer(_)
            | TypeRef::Union(_) => false,
        }
    }*/

    /// Is the type opaque
    pub fn is_opaque(&self) -> bool {
        matches!(self, Self::OpaqueType(_))
    }
}
