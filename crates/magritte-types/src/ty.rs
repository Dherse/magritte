//! Utilities for dealing with C-like types.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{Expr, Source};

/// A Vulkan C-like type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ty<'a> {
    /// A pointer
    Pointer(Mutability, Box<Ty<'a>>),

    /// A native (C) type
    Native(Native),

    /// A named type
    Named(Cow<'a, str>),

    /// A fixed-sized string array
    StringArray(Expr<'a>),

    /// A null-terminated string pointer
    NullTerminatedString(Mutability),

    /// A fixed-sized array
    Array(Box<Ty<'a>>, Expr<'a>),

    /// A slice
    Slice(Mutability, Box<Ty<'a>>, Expr<'a>),
}

impl<'a> Ty<'a> {
    /// Simplifies a type: reduces constant len to simpler expressions
    #[must_use]
    pub fn simplify(self) -> Self {
        match self {
            Ty::Array(ty, len) => {
                if len == Expr::ConstantInt(1) {
                    *ty
                } else {
                    Self::Array(ty, len)
                }
            },
            Ty::Slice(mutability, ty, len) => {
                if len == Expr::ConstantInt(1) {
                    Ty::Pointer(mutability, ty)
                } else {
                    Self::Slice(mutability, ty, len)
                }
            },
            this => this,
        }
    }

    /// Is the type void
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Native(Native::Void))
    }

    /// Creates a new void pointer type
    #[inline]
    pub fn void_ptr(mut_: Mutability) -> Self {
        Self::Pointer(mut_, box Self::void())
    }

    /// Creates a new [`std::ffi::c_void`] type
    #[inline]
    pub const fn void() -> Self {
        Self::Native(Native::Void)
    }

    /// Creates a new [`bool`] type
    #[inline]
    pub fn bool() -> Self {
        Self::Native(Native::Bool)
    }

    /// Creates a new [`u32`] type
    #[inline]
    pub const fn u32() -> Self {
        Self::Native(Native::UInt(4))
    }

    /// Creates a new [`u64`] type
    #[inline]
    pub const fn u64() -> Self {
        Self::Native(Native::UInt(8))
    }

    /// Creates a new [`usize`] type
    #[inline]
    pub const fn usize() -> Self {
        Self::Native(Native::USize)
    }

    /// Creates a new [`i32`] type
    #[inline]
    pub const fn i32() -> Self {
        Self::Native(Native::Int(4))
    }

    /// Creates a new [`i64`] type
    #[inline]
    pub const fn i64() -> Self {
        Self::Native(Native::Int(8))
    }

    /// Checks whether the current type is a void pointer
    pub fn is_void_ptr(&self) -> bool {
        matches!(
            self,
            Ty::Pointer(_, box Ty::Native(Native::Void)) | Ty::Slice(_, box Ty::Native(Native::Void), _)
        )
    }

    /// Checks whether the current type is a void pointer and returns its mutability
    pub fn is_void_ptr_mut(&self) -> Option<Mutability> {
        match self {
            Ty::Pointer(mut_, box Ty::Native(Native::Void)) | Ty::Slice(mut_, box Ty::Native(Native::Void), _) => {
                Some(*mut_)
            },
            _ => None,
        }
    }

    /// Checks whether the current type is a [`std::ffi::CStr`]
    pub fn is_cstr(&self) -> bool {
        matches!(self, Ty::NullTerminatedString(Mutability::Const))
    }

    /// Is `self` a native type
    #[inline]
    pub const fn is_native(&self) -> bool {
        matches!(self, Self::Native(_))
    }

    /// Gets the length expression, returns some if `self` is an array or a slice
    pub fn length(&self) -> Option<&Expr<'a>> {
        match self {
            Ty::StringArray(len) | Ty::Array(_, len) | Ty::Slice(_, _, len) => Some(len),
            _ => None,
        }
    }

    /// Decomposes self into a pointer's components
    ///
    /// # Panics
    /// If `self` is not a pointer.
    pub fn as_ptr(&self) -> (Mutability, &Ty<'_>) {
        match self {
            Ty::Pointer(a, b) => (*a, &**b),
            _ => panic!("not a pointer: {:?}", self),
        }
    }

    /// Decomposes self into a pointer's components
    ///
    /// # Panics
    /// If `self` is not a pointer.
    pub fn as_ptr_mut(&mut self) -> (&'_ mut Mutability, &'_ mut Ty<'a>) {
        match self {
            Ty::Pointer(a, b) => (a, &mut **b),
            _ => panic!("not a pointer: {:?}", self),
        }
    }

    /// Gets the named type (if any) at the core of this complex type.
    pub fn base_named_type(&self) -> Option<&Cow<'a, str>> {
        match self {
            Ty::Pointer(_, ty) | Ty::Array(ty, _) | Ty::Slice(_, ty, _) => ty.base_named_type(),
            Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => None,
            Ty::Named(name) => Some(name),
        }
    }

    /// Decomposes self into a native type
    ///
    /// # Panics
    /// If `self` is not a native type.
    pub fn as_native(&self) -> Native {
        match self {
            Ty::Native(a) => *a,
            _ => panic!("not a native type: {:?}", self),
        }
    }

    /// Decomposes self into a named type's name
    ///
    /// # Panics
    /// If `self` is not a named type.
    pub fn as_named(&self) -> &Cow<'_, str> {
        match self {
            Ty::Named(a) => a,
            _ => panic!("not a named type: {:?}", self),
        }
    }

    /// Decomposes self into a named type's mutable name
    ///
    /// # Panics
    /// If `self` is not a named type.
    pub fn as_named_mut(&mut self) -> &mut Cow<'a, str> {
        match self {
            Ty::Named(a) => a,
            _ => panic!("not a named type: {:?}", self),
        }
    }

    /// Decomposes self into a string array's length
    ///
    /// # Panics
    /// If `self` is not a string array.
    pub fn as_string_array(&self) -> &Expr<'_> {
        match self {
            Ty::StringArray(a) => a,
            _ => panic!("not a string array: {:?}", self),
        }
    }

    /// Decomposes self into a null terminated string's mutability (should always be const)
    ///
    /// # Panics
    /// If `self` is not a null terminated string pointer.
    pub fn as_null_terminated_string(&self) -> Mutability {
        match self {
            Ty::NullTerminatedString(a) => *a,
            _ => panic!("not a null terminated string: {:?}", self),
        }
    }

    /// Decomposes self into an array's type and length
    ///
    /// # Panics
    /// If `self` is not an array.
    pub fn as_array(&self) -> (&Ty<'_>, &Expr<'_>) {
        match self {
            Ty::Array(a, b) => (&**a, b),
            _ => panic!("not an array: {:?}", self),
        }
    }

    /// Decomposes self into a slice's mutability, element type and length
    ///
    /// # Panics
    /// If `self` is not a slice.
    pub fn as_slice(&self) -> (Mutability, &Ty<'a>, &Expr<'a>) {
        match self {
            Ty::Slice(a, b, c) => (*a, &**b, c),
            _ => panic!("not a slice: {:?}", self),
        }
    }

    /// Decomposes self into a mutable slice's mutability, element type and length
    ///
    /// # Panics
    /// If `self` is not a slice.
    pub fn as_slice_mut(&mut self) -> (&mut Mutability, &mut Ty<'a>, &mut Expr<'a>) {
        match self {
            Ty::Slice(a, b, c) => (a, &mut **b, c),
            _ => panic!("not a slice: {:?}", self),
        }
    }

    pub fn is_opaque(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Native(Native::Void) => true,
            Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => false,
            Ty::Pointer(_, ty) | Ty::Array(ty, _) | Ty::Slice(_, ty, _) => ty.is_opaque(source),
            Ty::Named(name) => source
                .find(name)
                .expect("unknown type")
                .as_type_ref()
                .expect("not a type")
                .has_opaque(source),
        }
    }

    pub fn as_static(self) -> Ty<'static> {
        match self {
            Ty::Pointer(a, b) => Ty::Pointer(a, box b.as_static()),
            Ty::Native(a) => Ty::Native(a),
            Ty::Named(a) => Ty::Named(Cow::Owned(a.into_owned())),
            Ty::StringArray(a) => Ty::StringArray(a.as_static()),
            Ty::NullTerminatedString(a) => Ty::NullTerminatedString(a),
            Ty::Array(a, b) => Ty::Array(box a.as_static(), b.as_static()),
            Ty::Slice(a, b, c) => Ty::Slice(a, box b.as_static(), c.as_static()),
        }
    }
}

/// A native type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Native {
    /// Void type
    Void,

    /// A Uint with a certain number of bytes
    UInt(u8),

    /// An Int with a certain number of bytes
    Int(u8),

    /// An unsigned size (`usize`)
    USize,

    /// An unsigned size (`isize`)
    SSize,

    /// A character (`i8`)
    Char,

    /// An unsigned character (`u8`)
    UChar,

    /// Single precision float
    Float,

    /// Double precision float
    Double,

    /// A boolean (unused)
    Bool,

    /// A null terminated string
    NullTerminatedString,
}

impl Native {
    pub fn name(&self) -> &'static str {
        match self {
            Native::Void => "std::ffi::c_void",
            Native::UInt(size) => match size {
                1 => "u8",
                2 => "u16",
                4 => "u32",
                8 => "u64",
                other => unreachable!("unsupported size: {other}"),
            },
            Native::Int(size) => match size {
                1 => "i8",
                2 => "i16",
                4 => "i32",
                8 => "i64",
                other => unreachable!("unsupported size: {other}"),
            },
            Native::USize => "usize",
            Native::SSize => "isize",
            Native::Char => "std::ffi::c_char",
            Native::UChar => "std::ffi::c_uchar",
            Native::Float => "f32",
            Native::Double => "f64",
            Native::Bool => "bool",
            Native::NullTerminatedString => "std::ffi::CStr",
        }
    }
}

#[cfg(feature = "codegen")]
impl quote::ToTokens for Native {
    fn to_tokens(&self, mut tokens: &mut proc_macro2::TokenStream) {
        match self {
            Native::Void => {
                quote::quote_each_token!(tokens std::ffi::c_void);
            },
            Native::UInt(size) => match size {
                1 => {
                    quote::quote_each_token!(tokens u8);
                },
                2 => {
                    quote::quote_each_token!(tokens u16);
                },
                4 => {
                    quote::quote_each_token!(tokens u32);
                },
                8 => {
                    quote::quote_each_token!(tokens u64);
                },
                other => unreachable!("unsupported size: {other}"),
            },
            Native::Int(size) => match size {
                1 => {
                    quote::quote_each_token!(tokens i8);
                },
                2 => {
                    quote::quote_each_token!(tokens i16);
                },
                4 => {
                    quote::quote_each_token!(tokens i32);
                },
                8 => {
                    quote::quote_each_token!(tokens i64);
                },
                other => unreachable!("unsupported size: {other}"),
            },
            Native::USize => {
                quote::quote_each_token!(tokens usize);
            },
            Native::SSize => {
                quote::quote_each_token!(tokens isize);
            },
            Native::Char => {
                quote::quote_each_token!(tokens std::ffi::c_char);
            },
            Native::UChar => {
                quote::quote_each_token!(tokens std::ffi::c_uchar);
            },
            Native::Float => {
                quote::quote_each_token!(tokens f32);
            },
            Native::Double => {
                quote::quote_each_token!(tokens f64);
            },
            Native::Bool => {
                quote::quote_each_token!(tokens bool);
            },
            Native::NullTerminatedString => {
                quote::quote_each_token!(tokens std::ffi::CStr);
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mutability {
    /// Is mutable
    Mutable,

    /// Is not mutable
    Const,
}

impl Default for Mutability {
    fn default() -> Self {
        Self::Const
    }
}

impl Mutability {
    /// Is `self` mutable
    #[inline]
    pub const fn is_mut(&self) -> bool {
        matches!(self, Self::Mutable)
    }

    /// Is `self` immutable
    #[inline]
    pub const fn is_const(&self) -> bool {
        matches!(self, Self::Const)
    }
}
