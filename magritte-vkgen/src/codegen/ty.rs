use proc_macro2::{Ident, Span};
use syn::{
    punctuated::Punctuated, Lifetime, Path, PathSegment, Token, Type, TypeArray, TypePath, TypePtr, TypeReference,
};

use crate::{
    imports::Imports,
    source::{reference::TypeRef, Source},
    ty::{Mutability, Native, Ty},
};

impl<'a> Ty<'a> {
    /// Turns a type into a token stream
    pub(super) fn as_ty(&self, source: &Source<'a>, imports: Option<&Imports>) -> Type {
        match self {
            Ty::Native(native) => native.as_type(imports),
            Ty::Pointer(mutability, ty) => Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: mutability.as_const_token(),
                mutability: mutability.as_mutability_token(),
                elem: box ty.as_ty(source, imports),
            }),
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_type(imports),
            Ty::StringArray(len) => Type::Array(TypeArray {
                bracket_token: Default::default(),
                elem: box Native::Char.as_type(None),
                semi_token: Default::default(),
                len: len.as_const_expr(source, imports),
            }),
            Ty::Array(ty, len) => {
                let elem = box ty.as_ty(source, imports);
                let len = len.as_const_expr(source, imports);

                Type::Array(TypeArray {
                    bracket_token: Default::default(),
                    elem,
                    semi_token: Default::default(),
                    len,
                })
            },
            Ty::Slice(mutability, ty, _) => Type::Ptr(TypePtr {
                star_token: Default::default(),
                const_token: mutability.as_const_token(),
                mutability: mutability.as_mutability_token(),
                elem: box ty.as_ty(source, imports),
            }),
            Ty::NullTerminatedString(_) => Native::NullTerminatedString.as_type(imports),
        }
    }
}

impl<'a: 'b, 'b> TypeRef<'a, 'b> {
    /// Turns a type reference into a tokenized type
    pub fn as_type(&self, imports: Option<&Imports>) -> Type {
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
    /// Gets a type from this native type, imports it if needed
    pub fn as_type(&self, imports: Option<&Imports>) -> Type {
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
                        ident: Ident::new("static", Span::call_site()),
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
                        ident: Ident::new("static", Span::call_site()),
                    }),
                    mutability: None,
                    elem: box Type::Path(TypePath {
                        qself: None,
                        path: Path {
                            leading_colon: None,
                            segments: Punctuated::from_iter(
                                [
                                    PathSegment::from(Ident::new("std", Span::call_site())),
                                    PathSegment::from(Ident::new("ffi", Span::call_site())),
                                    PathSegment::from(Ident::new("CStr", Span::call_site())),
                                ]
                                .into_iter(),
                            ),
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
                    segments: Punctuated::from_iter(
                        [
                            PathSegment::from(Ident::new("std", Span::call_site())),
                            PathSegment::from(Ident::new("ffi", Span::call_site())),
                            PathSegment::from(Ident::new("c_void", Span::call_site())),
                        ]
                        .into_iter(),
                    ),
                },
            },
            Native::Char => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(
                        [
                            PathSegment::from(Ident::new("std", Span::call_site())),
                            PathSegment::from(Ident::new("os", Span::call_site())),
                            PathSegment::from(Ident::new("raw", Span::call_site())),
                            PathSegment::from(Ident::new("c_char", Span::call_site())),
                        ]
                        .into_iter(),
                    ),
                },
            },
            Native::UChar => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(
                        [
                            PathSegment::from(Ident::new("std", Span::call_site())),
                            PathSegment::from(Ident::new("os", Span::call_site())),
                            PathSegment::from(Ident::new("raw", Span::call_site())),
                            PathSegment::from(Ident::new("c_uchar", Span::call_site())),
                        ]
                        .into_iter(),
                    ),
                },
            },
            Native::NullTerminatedString => TypePath {
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter(
                        [
                            PathSegment::from(Ident::new("std", Span::call_site())),
                            PathSegment::from(Ident::new("ffi", Span::call_site())),
                            PathSegment::from(Ident::new("CStr", Span::call_site())),
                        ]
                        .into_iter(),
                    ),
                },
            },
            _ => TypePath {
                qself: None,
                path: Path::from(PathSegment::from(self.as_ident())),
            },
        }
    }
}
