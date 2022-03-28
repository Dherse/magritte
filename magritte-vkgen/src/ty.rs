//! Utilities for dealing with C-like types.

use std::{borrow::Cow, hint::unreachable_unchecked};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete,
        complete::{alpha1, alphanumeric1, space0},
    },
    combinator::{map, recognize},
    multi::many0,
    sequence::{delimited, pair, preceded, tuple},
    IResult,
};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use crate::expr::{parse_expr, variable_raw, Expr};

/// A Vulkan C-like type
#[derive(Debug, Clone, PartialEq)]
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
    /// Creates a new type from a definition and a length
    #[must_use]
    pub fn new(definition: &'a str, length_str: &'a str) -> (Cow<'a, str>, Self) {
        let definition = definition.trim();
        let lengths = length_str.split(',');

        let (_, base_ty) = ty(definition).unwrap();

        let mut name = "unknown";
        let ty = process_base_ty(&mut name, base_ty, lengths.filter(|s| !s.is_empty()));

        (Cow::Borrowed(name), ty)
    }

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
    pub fn as_slice(&self) -> (Mutability, &Ty<'_>, &Expr<'_>) {
        match self {
            Ty::Slice(a, b, c) => (*a, &**b, c),
            _ => panic!("not a slice: {:?}", self),
        }
    }
}

/// A native type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Gets the default value of this native type as tokens
    pub fn default_tokens(&self) -> TokenStream {
        match self {
            Native::Void => panic!("no default value for void"),
            Native::UInt(_) | Native::Int(_) | Native::USize | Native::SSize => quote! { 0 },
            Native::Char => quote! { b'\0' as i8 },
            Native::UChar => quote! { b'\0' },
            Native::Float | Native::Double => quote! { 0.0 },
            Native::Bool => quote! { false },
            Native::NullTerminatedString => quote! { std::ptr::null() },
        }
    }
}

impl ToTokens for Native {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Native::Void => quote! { std::ffi::c_void },

            Native::UInt(size) => match size {
                1 => quote! { u8 },
                2 => quote! { u16 },
                4 => quote! { u32 },
                8 => quote! { u64 },
                _ => unsafe { unreachable_unchecked() },
            },

            Native::Int(size) => match size {
                1 => quote! { i8 },
                2 => quote! { i16 },
                4 => quote! { i32 },
                8 => quote! { i64 },
                _ => unsafe { unreachable_unchecked() },
            },

            Native::USize => quote! { usize },
            Native::SSize => quote! { isize },
            Native::Char => quote! { i8 },
            Native::UChar => quote! { u8 },
            Native::Float => quote! { f32 },
            Native::Double => quote! { f64 },
            Native::Bool => quote! { bool },
            Native::NullTerminatedString => quote! { &std::ffi::CStr },
        }
        .to_tokens(tokens);
    }
}

/// Enum representing whether a reference, slice or pointer is mutable or not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mutability {
    /// Is mutable
    Mutable,

    /// Is not mutable
    Const,
}

impl Mutability {
    /// As a mutability token for pointers
    #[inline]
    pub fn to_pointer_tokens(&self) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { mut },
            Mutability::Const => quote! { const },
        }
    }

    /// As a mutability tokens for references
    #[inline]
    pub fn to_ref_token(&self) -> Option<syn::Token![mut]> {
        match self {
            Mutability::Mutable => Some(syn::Token![mut](Span::call_site())),
            Mutability::Const => None,
        }
    }

    /// As a `as_ptr` identifier
    #[inline]
    pub fn to_as_ptr_tokens(&self) -> Ident {
        match self {
            Mutability::Mutable => Ident::new("as_mut", Span::call_site()),
            Mutability::Const => Ident::new("as_ptr", Span::call_site()),
        }
    }

    /// To a null pointer function identifier
    #[inline]
    pub fn to_null_tokens(&self) -> Ident {
        match self {
            Mutability::Mutable => Ident::new("null_mut", Span::call_site()),
            Mutability::Const => Ident::new("null", Span::call_site()),
        }
    }

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

fn process_base_ty<'a>(name: &mut &'a str, base_ty: BaseTy<'a>, mut lengths: impl Iterator<Item = &'a str>) -> Ty<'a> {
    match base_ty {
        BaseTy::ArgName(n, ty) => {
            *name = n;
            process_base_ty(name, *ty, lengths)
        },
        BaseTy::Pointer(mutability, ty) => {
            if let Some(length) = lengths.next() {
                let inside = process_base_ty(name, *ty, lengths);
                if inside == Ty::Native(Native::Char) {
                    if length == "null-terminated" {
                        Ty::NullTerminatedString(mutability)
                    } else {
                        Ty::Slice(mutability, box inside, Expr::new(length))
                    }
                } else {
                    Ty::Slice(mutability, box inside, Expr::new(length))
                }
            } else {
                Ty::Pointer(mutability, box process_base_ty(name, *ty, lengths))
            }
        },
        BaseTy::Native(native) => Ty::Native(native),
        BaseTy::Named(named) => Ty::Named(Cow::Borrowed(named)),
        BaseTy::Array(length, ty) => Ty::Array(box process_base_ty(name, *ty, lengths), length),
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BaseTy<'a> {
    ArgName(&'a str, Box<Self>),
    Array(Expr<'a>, Box<Self>),
    Pointer(Mutability, Box<Self>),
    Native(Native),
    Named(&'a str),
}

fn ty(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    alt((
        array,
        map(
            tuple((alt((pointer_of_pointer, pointer, native, named)), variable_raw)),
            |(ty, name)| BaseTy::ArgName(name, box ty),
        ),
        alt((pointer_of_pointer, pointer, native, named)),
    ))(input)
}

fn ty_no_ptr(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    alt((native, named))(input)
}

/// Matches an array
fn array(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    map(
        tuple((
            many0(delimited(space0, tag("const"), space0)),
            ty_no_ptr,
            delimited(space0, variable_raw, space0),
            delimited(
                space0,
                delimited(
                    complete::char('['),
                    delimited(space0, parse_expr, space0),
                    complete::char(']'),
                ),
                space0,
            ),
        )),
        |(_, ty, arg, len)| BaseTy::ArgName(arg, box BaseTy::Array(len, box ty)),
    )(input)
}

/// Matches a const keyword surrounded by spaces
fn const_keyword(input: &str) -> IResult<&str, &str> {
    delimited(space0, recognize(tag("const")), space0)(input)
}

/// Matches the pointer start (i.e `*`) surrounded by spaces
fn pointer_star(input: &str) -> IResult<&str, char> {
    delimited(space0, complete::char('*'), space0)(input)
}

/// Matches a named type name
fn named(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    map(
        delimited(
            space0,
            preceded(
                delimited(space0, many0(tag("struct")), space0),
                recognize(pair(alt((alpha1, tag("_"))), many0(alt((alphanumeric1, tag("_")))))),
            ),
            space0,
        ),
        BaseTy::Named,
    )(input)
}

/// Matches a native C type
fn native(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    map(native_raw, BaseTy::Native)(input)
}

/// Matches a native C type
#[doc(hidden)]
pub fn native_raw(input: &'_ str) -> IResult<&'_ str, Native> {
    map(
        delimited(
            space0,
            alt((
                tag("void"),
                tag("uint8_t"),
                tag("uint16_t"),
                tag("uint32_t"),
                tag("uint64_t"),
                tag("int8_t"),
                tag("int16_t"),
                tag("int32_t"),
                tag("int64_t"),
                tag("size_t"),
                tag("ssize_t"),
                tag("float"),
                tag("double"),
                tag("unsigned int"),
                tag("unsigned long"),
                tag("unsigned long long"),
                tag("unsigned short"),
                tag("char"),
                tag("int"),
            )),
            space0,
        ),
        |s| match s {
            "void" => Native::Void,
            "uint8_t" => Native::UInt(1),
            "uint16_t" => Native::UInt(2),
            "uint32_t" => Native::UInt(4),
            "uint64_t" => Native::UInt(8),
            "int8_t" => Native::Int(1),
            "int16_t" => Native::Int(2),
            "int32_t" | "int" => Native::Int(4),
            "int64_t" => Native::Int(8),
            "size_t" => Native::USize,
            "ssize_t" => Native::SSize,
            "float" => Native::Float,
            "double" => Native::Double,
            "unsigned char" => Native::UChar,
            "char" => Native::Char,
            _ => unsafe { unreachable_unchecked() },
        },
    )(input)
}

fn pointer(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    map(
        tuple((many0(const_keyword), ty_no_ptr, pointer_star)),
        |(const_a, ty, _)| {
            BaseTy::Pointer(
                if const_a.is_empty() {
                    Mutability::Mutable
                } else {
                    Mutability::Const
                },
                box ty,
            )
        },
    )(input)
}

fn pointer_of_pointer(input: &'_ str) -> IResult<&'_ str, BaseTy<'_>> {
    map(
        tuple((
            many0(const_keyword),
            ty_no_ptr,
            pointer_star,
            many0(const_keyword),
            pointer_star,
        )),
        |(const_a, ty, _, const_b, _)| {
            BaseTy::Pointer(
                if const_a.is_empty() {
                    Mutability::Mutable
                } else {
                    Mutability::Const
                },
                box BaseTy::Pointer(
                    if const_b.is_empty() {
                        Mutability::Mutable
                    } else {
                        Mutability::Const
                    },
                    box ty,
                ),
            )
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_natives() {
        assert_eq!(
            ty("uint32_t this"),
            Ok(("", BaseTy::ArgName("this", box BaseTy::Native(Native::UInt(4)))))
        );

        assert_eq!(
            ty("float this"),
            Ok(("", BaseTy::ArgName("this", box BaseTy::Native(Native::Float))))
        );

        assert_eq!(
            ty("size_t this"),
            Ok(("", BaseTy::ArgName("this", box BaseTy::Native(Native::USize))))
        );
    }

    #[test]
    fn test_named() {
        assert_eq!(
            ty("VkInstance this"),
            Ok(("", BaseTy::ArgName("this", box BaseTy::Named("VkInstance"))))
        );

        assert_eq!(
            ty("MyType this"),
            Ok(("", BaseTy::ArgName("this", box BaseTy::Named("MyType"))))
        );

        assert_eq!(
            ty("VkVeryLongVulkanTypeNameAsTheyLoveIt this"),
            Ok((
                "",
                BaseTy::ArgName("this", box BaseTy::Named("VkVeryLongVulkanTypeNameAsTheyLoveIt"))
            ))
        );
    }

    #[test]
    fn test_pointers() {
        assert_eq!(
            ty("const VkInstance* this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(Mutability::Const, box BaseTy::Named("VkInstance"))
                )
            ))
        );

        assert_eq!(
            ty("VkInstance* this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(Mutability::Mutable, box BaseTy::Named("VkInstance"))
                )
            ))
        );

        assert_eq!(
            ty("const uint32_t* this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(Mutability::Const, box BaseTy::Native(Native::UInt(4)))
                )
            ))
        );

        assert_eq!(
            ty("const int32_t* pViewOffsets"),
            Ok((
                "",
                BaseTy::ArgName(
                    "pViewOffsets",
                    box BaseTy::Pointer(Mutability::Const, box BaseTy::Native(Native::Int(4)))
                )
            ))
        );

        assert_eq!(
            ty("uint32_t* this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(Mutability::Mutable, box BaseTy::Native(Native::UInt(4)))
                )
            ))
        );
    }

    #[test]
    fn test_pointers_of_pointers() {
        assert_eq!(
            ty("const VkInstance* const* this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(
                        Mutability::Const,
                        box BaseTy::Pointer(Mutability::Const, box BaseTy::Named("VkInstance"))
                    )
                )
            ))
        );

        assert_eq!(
            ty("void** this"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Pointer(
                        Mutability::Mutable,
                        box BaseTy::Pointer(Mutability::Mutable, box BaseTy::Native(Native::Void))
                    )
                )
            ))
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            ty("VkInstance this[5]"),
            Ok((
                "",
                BaseTy::ArgName(
                    "this",
                    box BaseTy::Array(Expr::ConstantInt(5), box BaseTy::Named("VkInstance"))
                )
            ))
        );

        assert_eq!(
            ty("uint32_t uuid[VK_UUID_SIZE]"),
            Ok((
                "",
                BaseTy::ArgName(
                    "uuid",
                    box BaseTy::Array(
                        Expr::Constant(Cow::Borrowed("VK_UUID_SIZE")),
                        box BaseTy::Native(Native::UInt(4))
                    )
                )
            ))
        );
    }
}
