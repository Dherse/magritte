use std::{borrow::Cow, hint::unreachable_unchecked};

use magritte_types::{Expr, Mutability, Native, Ty};

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

use crate::expr::{parse_expr, variable_raw};

/// Creates a new type from a definition and a length
#[must_use]
pub fn ty_new(definitions: &str, name: &str, length_str: &str) -> Ty<'static> {
    let code = definitions.trim_end_matches(name);

    let lengths = length_str.split(',').map(ToString::to_string);

    let (_, base_ty) = ty(&code).unwrap();

    let ty = process_base_ty(&mut String::default(), base_ty, lengths.filter(|s| !s.is_empty())).as_static();

    ty
}

/// Creates a new type from a definition and a length
#[must_use]
pub fn ty_with_name(definition: &str, length_str: &str) -> (String, Ty<'static>) {
    let lengths = length_str.split(',').map(ToString::to_string);

    let (_, base_ty) = ty(definition.trim()).unwrap();

    let mut name = "unknown".to_string();
    let ty = process_base_ty(&mut name, base_ty, lengths.filter(|s| !s.is_empty())).as_static();

    (name, ty)
}

fn process_base_ty<'a>(name: &mut String, base_ty: BaseTy<'a>, mut lengths: impl Iterator<Item = String>) -> Ty<'a> {
    match base_ty {
        BaseTy::ArgName(n, ty) => {
            *name = n.to_string();
            process_base_ty(name, *ty, lengths)
        },
        BaseTy::Pointer(mutability, ty) => {
            if let Some(length) = lengths.next() {
                let inside = process_base_ty(name, *ty, lengths);
                if inside == Ty::Native(Native::Char) {
                    if length == "null-terminated" {
                        Ty::NullTerminatedString(mutability)
                    } else {
                        Ty::Slice(mutability, box inside, parse_expr(&length).unwrap().1).as_static()
                    }
                } else {
                    Ty::Slice(mutability, box inside, parse_expr(&length).unwrap().1).as_static()
                }
            } else {
                Ty::Pointer(mutability, box process_base_ty(name, *ty, lengths))
            }
        },
        BaseTy::Native(native) => Ty::Native(native),
        BaseTy::Named(named) => Ty::Named(Cow::Owned(named.to_string())),
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
