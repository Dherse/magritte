//! # Reference
//! References for elements of a source

use std::hint::unreachable_unchecked;

use crate::Queryable;

use super::{
    Alias, Basetype, Bitflag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function, FunctionPointer,
    Handle, OpaqueType, Origin, Source, Struct, Tag, Union, Vendor,
};

macro_rules! ref_function {
    ($name:ident: $is:ident, $as:ident, $as_unchecked:ident, $try_as:ident) => {
        pub const fn $is(&self) -> bool {
            matches!(self, Self::$name(_))
        }

        pub const fn $as(self) -> &'b $name<'a> {
            match self {
                Self::$name(value) => value,
                _ => panic!(concat!("reference is not a `", stringify!($name), "`"))
            }
        }

        pub const fn $try_as(self) -> Option<&'b $name<'a>> {
            match self {
                Self::$name(value) => Some(value),
                _ => None
            }
        }

        pub const unsafe fn $as_unchecked(self) -> &'b $name<'a> {
            match self {
                Self::$name(value) => value,
                _ => unreachable_unchecked()
            }
        }
    };
    ($($name:ident: $is:ident, $as:ident, $as_unchecked:ident, $try_as:ident);* $(;)*) => {
        $(ref_function! {
            $name: $is, $as, $as_unchecked, $try_as
        })*
    }
}

macro_rules! ref_mut_function {
    ($name:ident: $is:ident, $as:ident, $as_unchecked:ident, $try_as:ident) => {
        pub const fn $is(&self) -> bool {
            matches!(self, Self::$name(_))
        }

        pub fn $as(self) -> &'b mut $name<'a> {
            match self {
                Self::$name(value) => value,
                _ => panic!(concat!("reference is not a `", stringify!($name), "`"))
            }
        }

        pub fn $try_as(self) -> Option<&'b mut $name<'a>> {
            match self {
                Self::$name(value) => Some(value),
                _ => None
            }
        }

        pub unsafe fn $as_unchecked(self) -> &'b mut $name<'a> {
            match self {
                Self::$name(value) => value,
                _ => unreachable_unchecked()
            }
        }
    };
    ($($name:ident: $is:ident, $as:ident, $as_unchecked:ident, $try_as:ident);* $(;)*) => {
        $(ref_mut_function! {
            $name: $is, $as, $as_unchecked, $try_as
        })*
    }
}

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
    Bitflag(&'b Bitflag<'a>),

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
            Self::Bitflag(r) => r.origin(),
            Self::Enum(r) => r.origin(),
            Self::CommandAlias(r) => r.origin(),
            Self::Function(r) => r.origin(),
            Self::Origin(r) => *r,
        }
    }

    /// Turns a reference into a type reference, returns `None` if it is not a type
    #[inline]
    pub const fn as_type_ref(self) -> Option<TypeRef<'a, 'b>> {
        match self {
            Ref::OpaqueType(r) => Some(TypeRef::OpaqueType(r)),
            Ref::Alias(r) => Some(TypeRef::Alias(r)),
            Ref::Struct(r) => Some(TypeRef::Struct(r)),
            Ref::Union(r) => Some(TypeRef::Union(r)),
            Ref::Handle(r) => Some(TypeRef::Handle(r)),
            Ref::FunctionPointer(r) => Some(TypeRef::FunctionPointer(r)),
            Ref::Basetype(r) => Some(TypeRef::Basetype(r)),
            Ref::Bitmask(r) => Some(TypeRef::Bitmask(r)),
            Ref::Bitflag(r) => Some(TypeRef::Bitflag(r)),
            Ref::Enum(r) => Some(TypeRef::Enum(r)),
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
    pub const fn is_any_alias(&self) -> bool {
        matches!(self, Self::Alias(_) | Self::ConstAlias(_) | Self::CommandAlias(_))
    }

    /// Gets the name of the reference
    pub fn name(&self) -> String {
        match self {
            Self::Vendor(_) => panic!("a vendor cannot be made into a name"),
            Self::Extension(r) => r.name().to_owned(),
            Self::Tag(_) => panic!("a tag cannot be made into a name"),
            Self::OpaqueType(r) => r.original_name().to_owned(),
            Self::Alias(r) => r.name().to_owned(),
            Self::Struct(r) => r.name().to_owned(),
            Self::Union(r) => r.name().to_owned(),
            Self::Handle(r) => r.name().to_owned(),
            Self::FunctionPointer(r) => r.name().to_owned(),
            Self::Basetype(r) => r.name().to_owned(),
            Self::Bitmask(r) => r.name().to_owned(),
            Self::Const(r) => r.name().to_owned(),
            Self::ConstAlias(r) => r.name().to_owned(),
            Self::Bitflag(r) => r.name().to_owned(),
            Self::Enum(r) => r.name().to_owned(),
            Self::CommandAlias(r) => r.name().to_owned(),
            Self::Function(r) => r.name().to_owned(),
            Self::Origin(origin) => origin.as_name(),
        }
    }

    /// Gets the rust path with a given crate prefix for this object
    #[inline]
    pub fn as_rust_path(&self, prefix: &str) -> String {
        format!("{}::{}.rs", self.origin().as_rust_path(prefix), self.name())
    }

    ref_function! {
        Vendor: is_vendor, as_vendor, as_vendor_unchecked, try_as_vendor;
        Extension: is_extension, as_extension, as_extension_unchecked, try_as_extension;
        Tag: is_tag, as_tag, as_tag_unchecked, try_as_tag;
        OpaqueType: is_opaque_type, as_opaque_type, as_opaque_type_unchecked, try_as_opaque_type;
        Alias: is_alias, as_alias, as_alias_unchecked, try_as_alias;
        Struct: is_struct, as_struct, as_struct_unchecked, try_as_struct;
        Union: is_union, as_union, as_union_unchecked, try_as_union;
        Handle: is_handle, as_handle, as_handle_unchecked, try_as_handle;
        FunctionPointer: is_function_pointer, as_function_pointer, as_function_pointer_unchecked, try_as_function_pointer;
        Basetype: is_basetype, as_basetype, as_basetype_unchecked, try_as_basetype;
        Bitmask: is_bitmask, as_bitmask, as_bitmask_unchecked, try_as_bitmask;
        Bitflag: is_bitflag, as_bitflag, as_bitflag_unchecked, try_as_bitflag;
        Const: is_const, as_const, as_const_unchecked, try_as_const;
        ConstAlias: is_consta_lias, as_consta_lias, as_const_alias_unchecked, try_as_const_alias;
        Enum: is_enum, as_enum, as_enum_unchecked, try_as_enum;
        CommandAlias: is_command_alias, as_command_alias, as_command_alias_unchecked, try_as_command_alias;
        Function: is_function, as_function, as_function_unchecked, try_as_function;
        Origin: is_origin, as_origin, as_origin_unchecked, try_as_origin;
    }
}

/// A reference to a Vulkan element
#[derive(Debug, PartialEq)]
pub enum RefMut<'a: 'b, 'b> {
    /// A vendor defined in the Vulkan specification
    Vendor(&'b mut Vendor<'a>),

    /// An extension defined in the Vulkan specification
    Extension(&'b mut Extension<'a>),

    /// A tag defined in the Vulkan specification
    Tag(&'b mut Tag<'a>),

    /// An opaque type defined in the Vulkan specification
    OpaqueType(&'b mut OpaqueType<'a>),

    /// An alias defined in the Vulkan specification
    Alias(&'b mut Alias<'a>),

    /// A struct defined in the Vulkan specification
    Struct(&'b mut Struct<'a>),

    /// A union defined in the Vulkan specification
    Union(&'b mut Union<'a>),

    /// A handle defined in the Vulkan specification
    Handle(&'b mut Handle<'a>),

    /// A function pointer defined in the Vulkan specification
    FunctionPointer(&'b mut FunctionPointer<'a>),

    /// A base type defined in the Vulkan specification
    Basetype(&'b mut Basetype<'a>),

    /// A bitmask defined in the Vulkan specification
    Bitmask(&'b mut Bitmask<'a>),

    /// A constant defined in the Vulkan specification
    Const(&'b mut Const<'a>),

    /// A constant alias defined in the Vulkan specification
    ConstAlias(&'b mut ConstAlias<'a>),

    /// A bitflag defined in the Vulkan specification
    Bitflag(&'b mut Bitflag<'a>),

    /// An enum defined in the Vulkan specification
    Enum(&'b mut Enum<'a>),

    /// A command alias defined in the Vulkan specification
    CommandAlias(&'b mut CommandAlias<'a>),

    /// A function defined in the Vulkan specification
    Function(&'b mut Function<'a>),

    /// An origin defined in the Vulkan specification
    Origin(&'b mut Origin<'a>),
}

impl<'a: 'b, 'b> RefMut<'a, 'b> {
    ref_mut_function! {
        Vendor: is_vendor, as_vendor, as_vendor_unchecked, try_as_vendor;
        Extension: is_extension, as_extension, as_extension_unchecked, try_as_extension;
        Tag: is_tag, as_tag, as_tag_unchecked, try_as_tag;
        OpaqueType: is_opaque_type, as_opaque_type, as_opaque_type_unchecked, try_as_opaque_type;
        Alias: is_alias, as_alias, as_alias_unchecked, try_as_alias;
        Struct: is_struct, as_struct, as_struct_unchecked, try_as_struct;
        Union: is_union, as_union, as_union_unchecked, try_as_union;
        Handle: is_handle, as_handle, as_handle_unchecked, try_as_handle;
        FunctionPointer: is_function_pointer, as_function_pointer, as_function_pointer_unchecked, try_as_function_pointer;
        Basetype: is_basetype, as_basetype, as_basetype_unchecked, try_as_basetype;
        Bitmask: is_bitmask, as_bitmask, as_bitmask_unchecked, try_as_bitmask;
        Bitflag: is_bitflag, as_bitflag, as_bitflag_unchecked, try_as_bitflag;
        Const: is_const, as_const, as_const_unchecked, try_as_const;
        ConstAlias: is_consta_lias, as_consta_lias, as_const_alias_unchecked, try_as_const_alias;
        Enum: is_enum, as_enum, as_enum_unchecked, try_as_enum;
        CommandAlias: is_command_alias, as_command_alias, as_command_alias_unchecked, try_as_command_alias;
        Function: is_function, as_function, as_function_unchecked, try_as_function;
        Origin: is_origin, as_origin, as_origin_unchecked, try_as_origin;
    }

    pub fn assign_origin(self, origin: Origin<'a>) {
        match self {
            RefMut::Vendor(_) | RefMut::Extension(_) | RefMut::Tag(_) | RefMut::OpaqueType(_) | RefMut::Origin(_) => (),
            RefMut::Alias(alias) => alias.set_origin(origin),
            RefMut::Struct(struct_) => struct_.set_origin(origin),
            RefMut::Union(union_) => union_.set_origin(origin),
            RefMut::Handle(handle) => handle.set_origin(origin),
            RefMut::FunctionPointer(function_pointer) => function_pointer.set_origin(origin),
            RefMut::Basetype(basetype) => basetype.set_origin(origin),
            RefMut::Bitmask(bitmask) => bitmask.set_origin(origin),
            RefMut::Const(const_) => const_.set_origin(origin),
            RefMut::ConstAlias(const_alias) => const_alias.set_origin(origin),
            RefMut::Bitflag(bitflag) => {
                bitflag.set_origin(origin.clone());

                bitflag
                    .aliases_mut()
                    .iter_mut()
                    .filter(|a| a.origin().is_unknown())
                    .for_each(|a| a.set_origin(origin.clone()));
                bitflag
                    .bits_mut()
                    .iter_mut()
                    .filter(|a| a.origin().is_unknown())
                    .for_each(|a| a.set_origin(origin.clone()));
            },
            RefMut::Enum(enum_) => {
                enum_.set_origin(origin.clone());

                enum_
                    .aliases_mut()
                    .iter_mut()
                    .filter(|a| a.origin().is_unknown())
                    .for_each(|a| a.set_origin(origin.clone()));
                enum_
                    .variants_mut()
                    .iter_mut()
                    .filter(|a| a.origin().is_unknown())
                    .for_each(|a| a.set_origin(origin.clone()));
            },
            RefMut::CommandAlias(command_alias) => command_alias.set_origin(origin),
            RefMut::Function(function) => function.set_origin(origin),
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
    Bitflag(&'b Bitflag<'a>),

    /// An enum defined in the Vulkan specification
    Enum(&'b Enum<'a>),
}

impl<'a: 'b, 'b> TypeRef<'a, 'b> {
    ref_function! {
        OpaqueType: is_opaque_type, as_opaque_type, as_opaque_type_unchecked, try_as_opaque_type;
        Alias: is_alias, as_alias, as_alias_unchecked, try_as_alias;
        Struct: is_struct, as_struct, as_struct_unchecked, try_as_struct;
        Union: is_union, as_union, as_union_unchecked, try_as_union;
        Handle: is_handle, as_handle, as_handle_unchecked, try_as_handle;
        FunctionPointer: is_function_pointer, as_function_pointer, as_function_pointer_unchecked, try_as_function_pointer;
        Basetype: is_basetype, as_basetype, as_basetype_unchecked, try_as_basetype;
        Bitmask: is_bitmask, as_bitmask, as_bitmask_unchecked, try_as_bitmask;
        Bitflag: is_bitflag, as_bitflag, as_bitflag_unchecked, try_as_bitflag;
        Enum: is_enum, as_enum, as_enum_unchecked, try_as_enum;
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
            Self::Bitflag(r) => r.origin(),
            Self::Enum(r) => r.origin(),
        }
    }

    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        match self {
            TypeRef::OpaqueType(_) => true,
            TypeRef::Alias(alias) => source
                .find(alias.of())
                .expect("unknown alias")
                .as_type_ref()
                .expect("not a type")
                .has_opaque(source),
            TypeRef::Struct(s) => s.has_opaque(source),
            TypeRef::Union(u) => u.has_opaque(source),
            TypeRef::FunctionPointer(f) => f.has_opaque(source),
            TypeRef::Handle(_)
            | TypeRef::Basetype(_)
            | TypeRef::Bitmask(_)
            | TypeRef::Bitflag(_)
            | TypeRef::Enum(_) => false,
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
            TypeRef::Bitflag(flags) => flags.find(source, name),
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
            Self::Bitflag(r) => r.name(),
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
            Self::Bitflag(r) => Ref::Bitflag(*r),
            Self::Enum(r) => Ref::Enum(*r),
        }
    }
}

impl<'a, 'b> Queryable<'a> for TypeRef<'a, 'b> {
    fn find<'c>(&'c self, source: &'c Source<'a>, name: &str) -> Option<&'c str> {
        match self {
            TypeRef::OpaqueType(value) => value.find(source, name),
            TypeRef::Alias(value) => value.find(source, name),
            TypeRef::Struct(value) => value.find(source, name),
            TypeRef::Union(value) => value.find(source, name),
            TypeRef::Handle(value) => value.find(source, name),
            TypeRef::FunctionPointer(value) => value.find(source, name),
            TypeRef::Basetype(value) => value.find(source, name),
            TypeRef::Bitmask(value) => value.find(source, name),
            TypeRef::Bitflag(value) => value.find(source, name),
            TypeRef::Enum(value) => value.find(source, name),
        }
    }
}
