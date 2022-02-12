//! Additional implementation for [`Ty`] used to generate code

use std::iter::once;

use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, AngleBracketedGenericArguments, GenericArgument, Lifetime, Path, PathArguments,
    PathSegment, Token, Type, TypeArray, TypePath, TypePtr, TypeReference, TypeSlice,
};

use crate::{
    imports::Imports,
    source::{reference::TypeRef, Source},
    ty::{Mutability, Native, Ty},
};

/// Global lifetime name
pub const LIFETIME: &str = "this";

/// Creates an identifier for the global lifetime name
pub fn lifetime_as_ident() -> Ident {
    Ident::new(LIFETIME, Span::call_site())
}

/// Creates a lifetime for the global lifetime name
pub fn lifetime_as_lifetime() -> Lifetime {
    Lifetime {
        apostrophe: Span::call_site(),
        ident: lifetime_as_ident(),
    }
}

/// Creates a generic lifetime argument for the global lifetime name
pub fn lifetime_as_generic_argument() -> GenericArgument {
    GenericArgument::Lifetime(lifetime_as_lifetime())
}

impl<'a> Ty<'a> {
    /// Check if this type contains an opaque type (which are **always** treated as void pointers)
    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) | Ty::Array(ty, _) => ty.has_opaque(source),
            Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => false,
            Ty::Named(named) => source.resolve_type(named).expect("unknown type").is_opaque(),
        }
    }

    /// Does the type have a lifetime (with deep checking)
    pub fn has_lifetime(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) => !ty.has_opaque(source),
            Ty::NullTerminatedString(_) => true,
            Ty::Native(_) | Ty::StringArray(_) => false,
            Ty::Array(ty, _) => ty.has_lifetime(source),
            Ty::Named(named) => source.resolve_type(named).expect("unknown type").has_lifetime(source),
        }
    }

    /// Turns a type into a tokenized type and an optional lifetime argument
    pub fn as_ty(&self, source: &Source<'a>, imports: Option<&Imports>) -> (Type, bool) {
        match self {
            Ty::Native(_) | Ty::StringArray(_) => (self.as_const_ty(source, imports), false),
            Ty::Pointer(mutability, ty) => {
                if ty.has_opaque(source) {
                    (
                        Type::Reference(TypeReference {
                            and_token: Default::default(),
                            lifetime: Some(lifetime_as_lifetime()),
                            mutability: mutability.as_mutability_token(),
                            elem: box ty.as_ty(source, imports).0,
                        }),
                        true,
                    )
                } else {
                    (self.as_const_ty(source, imports), false)
                }
            },
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_type(source, imports),
            Ty::NullTerminatedString(_) => (Native::NullTerminatedString.as_type(imports), true),
            Ty::Array(ty, len) => {
                let (elem, lt) = ty.as_ty(source, imports);
                let len = len.as_const_expr(source, imports);

                (
                    Type::Array(TypeArray {
                        bracket_token: Default::default(),
                        elem: box elem,
                        semi_token: Default::default(),
                        len,
                    }),
                    lt,
                )
            },
            Ty::Slice(mutability, ty, _) => {
                if ty.has_opaque(source) {
                    (self.as_const_ty(source, imports), false)
                } else {
                    (
                        Type::Reference(TypeReference {
                            and_token: Default::default(),
                            lifetime: Some(lifetime_as_lifetime()),
                            mutability: mutability.as_mutability_token(),
                            elem: box Type::Slice(TypeSlice {
                                bracket_token: Default::default(),
                                elem: box ty.as_ty(source, imports).0,
                            }),
                        }),
                        true,
                    )
                }
            },
        }
    }

    /// Turns a type into a tokenized type
    pub(super) fn as_const_ty(&self, source: &Source<'a>, imports: Option<&Imports>) -> Type {
        match self {
            Ty::Native(native) => native.as_type(imports),
            Ty::Pointer(mutability, ty) | Ty::Slice(mutability, ty, _) => Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: mutability.as_const_token(),
                mutability: mutability.as_mutability_token(),
                elem: box ty.as_const_ty(source, imports),
            }),
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_const_type(source, imports),
            Ty::StringArray(len) => Type::Array(TypeArray {
                bracket_token: Default::default(),
                elem: box Native::Char.as_type(None),
                semi_token: Default::default(),
                len: len.as_const_expr(source, imports),
            }),
            Ty::Array(ty, len) => {
                let elem = box ty.as_const_ty(source, imports);
                let len = len.as_const_expr(source, imports);

                Type::Array(TypeArray {
                    bracket_token: Default::default(),
                    elem,
                    semi_token: Default::default(),
                    len,
                })
            },
            Ty::NullTerminatedString(_) => Native::NullTerminatedString.as_const_type(imports),
        }
    }
}

impl<'a: 'b, 'b> TypeRef<'a, 'b> {
    /// Turns a type reference into a tokenized type
    pub fn as_const_type(&self, source: &Source<'a>, imports: Option<&Imports>) -> Type {
        assert!(!self.has_lifetime(source), "type cannot be made into static type");

        if let Some(imports) = imports {
            self.import(imports);

            Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(self.as_ident())),
            })
        } else {
            Type::Path(TypePath {
                qself: None,
                path: {
                    let mut path = self.origin().as_path();

                    path.segments.extend_one(PathSegment::from(self.as_ident()));

                    path
                },
            })
        }
    }

    /// Turns a type reference into a tokenized type and a boolean designating whether it
    /// has a lifetime
    pub fn as_type(&self, source: &Source<'a>, imports: Option<&Imports>) -> (Type, bool) {
        let lt = self.has_lifetime(source) && !self.is_opaque();

        let mut path = if let Some(imports) = imports {
            self.import(imports);

            Path {
                leading_colon: None,
                segments: Punctuated::new(),
            }
        } else {
            self.origin().as_path()
        };

        path.segments.push(if lt {
            PathSegment {
                ident: self.as_ident(),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: once(lifetime_as_generic_argument()).collect(),
                    gt_token: Default::default(),
                }),
            }
        } else {
            PathSegment::from(self.as_ident())
        });

        (Type::Path(TypePath { qself: None, path }), lt)
    }
}

impl Mutability {
    /// Returns this mutability as an optional const token
    #[inline]
    pub fn as_const_token(&self) -> Option<Token![const]> {
        if self.is_const() {
            Some(Default::default())
        } else {
            None
        }
    }

    /// Returns this mutability as an optional mut token
    #[inline]
    pub fn as_mutability_token(&self) -> Option<Token![mut]> {
        if self.is_const() {
            Some(Default::default())
        } else {
            None
        }
    }
}

impl Native {
    /// Gets the native type has a dynamic type that will (if needed) use the global lifetime name
    pub fn as_type(&self, imports: Option<&Imports>) -> Type {
        self.as_type_with_lifetime(lifetime_as_ident(), imports)
    }

    /// Gets the native type has a dynamic type that will (if needed) use the static lifetime
    pub fn as_const_type(&self, imports: Option<&Imports>) -> Type {
        self.as_type_with_lifetime(Ident::new("static", Span::call_site()), imports)
    }

    /// Gets a type from this native type, imports it if needed
    pub fn as_type_with_lifetime(&self, lifetime: Ident, imports: Option<&Imports>) -> Type {
        if let Some(imports) = imports {
            self.import(imports);

            let path = Type::Path(TypePath {
                qself: None,
                path: Path::from(PathSegment::from(self.as_ident())),
            });

            match self {
                Self::NullTerminatedString => Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: Some(Lifetime {
                        apostrophe: Span::call_site(),
                        ident: lifetime,
                    }),
                    mutability: None,
                    elem: box path,
                }),
                _ => path,
            }
        } else {
            match self {
                Self::NullTerminatedString => Type::Reference(TypeReference {
                    and_token: Default::default(),
                    lifetime: Some(Lifetime {
                        apostrophe: Span::call_site(),
                        ident: lifetime,
                    }),
                    mutability: None,
                    elem: box Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: [
                                PathSegment::from(Ident::new("std", Span::call_site())),
                                PathSegment::from(Ident::new("ffi", Span::call_site())),
                                PathSegment::from(Ident::new("CStr", Span::call_site())),
                            ]
                            .into_iter()
                            .collect(),
                        },
                    }),
                }),
                _ => Type::Path(self.as_type_path()),
            }
        }
    }

    /// Imports the current type
    #[inline]
    pub fn import(&self, imports: &Imports) {
        match self {
            Native::Void => imports.push("std::ffi::c_void"),
            Native::Char => imports.push("std::os::raw::c_char"),
            Native::UChar => imports.push("std::os::raw::c_uchar"),
            Native::NullTerminatedString => imports.push("std::ffi::CStr"),
            _ => {},
        }
    }

    /// Turns a native type into an identifier
    pub fn as_ident(&self) -> Ident {
        Ident::new(
            match self {
                Native::Void => "c_void",
                Native::UInt(1) => "u8",
                Native::UInt(2) => "u16",
                Native::UInt(4) => "u32",
                Native::UInt(8) => "u64",
                Native::UInt(other) => panic!("unsupported unsigned int size: {}", other),
                Native::Int(1) => "i8",
                Native::Int(2) => "i16",
                Native::Int(4) => "i32",
                Native::Int(8) => "i64",
                Native::Int(other) => panic!("unsupported signed int size: {}", other),
                Native::USize => "usize",
                Native::SSize => "isize",
                Native::Char => "c_schar",
                Native::UChar => "c_uchar",
                Native::Float => "f32",
                Native::Double => "f64",
                Native::Bool => "bool",
                Native::NullTerminatedString => "CStr",
            },
            Span::call_site(),
        )
    }

    /// Turns a native type into a type path
    pub fn as_type_path(&self) -> TypePath {
        match self {
            Native::Void => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment::from(Ident::new("std", Span::call_site())),
                        PathSegment::from(Ident::new("ffi", Span::call_site())),
                        PathSegment::from(Ident::new("c_void", Span::call_site())),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            Native::Char => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment::from(Ident::new("std", Span::call_site())),
                        PathSegment::from(Ident::new("os", Span::call_site())),
                        PathSegment::from(Ident::new("raw", Span::call_site())),
                        PathSegment::from(Ident::new("c_char", Span::call_site())),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            Native::UChar => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment::from(Ident::new("std", Span::call_site())),
                        PathSegment::from(Ident::new("os", Span::call_site())),
                        PathSegment::from(Ident::new("raw", Span::call_site())),
                        PathSegment::from(Ident::new("c_uchar", Span::call_site())),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            Native::NullTerminatedString => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment::from(Ident::new("std", Span::call_site())),
                        PathSegment::from(Ident::new("ffi", Span::call_site())),
                        PathSegment::from(Ident::new("CStr", Span::call_site())),
                    ]
                    .into_iter()
                    .collect(),
                },
            },
            _ => TypePath {
                qself: None,
                path: Path::from(PathSegment::from(self.as_ident())),
            },
        }
    }
}
