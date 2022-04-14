//! Additional implementation for [`Ty`] used to generate code

use std::borrow::Cow;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use crate::{
    imports::Imports,
    source::{reference::TypeRef, Source},
    ty::{Mutability, Native, Ty},
};

/// Global lifetime name
pub const LIFETIME: &str = "lt";

/// Creates an identifier for the global lifetime name
pub fn lifetime_as_ident() -> Ident {
    Ident::new(LIFETIME, Span::call_site())
}

/// Creates a lifetime for the global lifetime name
pub fn lifetime_as_lifetime() -> TokenStream {
    quote! {
        'lt
    }
}

/// Creates a generic lifetime argument for the global lifetime name
pub fn lifetime_as_generic_argument() -> TokenStream {
    quote! { <'lt> }
}

impl<'a> Ty<'a> {
    /// Check if this type is an opaque type (which are **always** treated as void pointers)
    pub fn is_opaque(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) | Ty::Array(ty, _) => ty.is_opaque(source),
            Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => false,
            Ty::Named(named) => source.resolve_type(named).expect("unknown type").is_opaque(),
        }
    }

    /// Check if this type contains an opaque type (which are **always** treated as void pointers)
    pub fn has_opaque(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) | Ty::Array(ty, _) => ty.has_opaque(source),
            Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => false,
            Ty::Named(named) => source.resolve_type(named).expect("unknown type").is_opaque(),
        }
    }

    /// Does the type have a lifetime (with deep checking)
    pub fn has_lifetime(&self, source: &Source<'a>, pointer_has_lifetime: bool) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) => {
                (pointer_has_lifetime && !ty.has_opaque(source)) || ty.has_lifetime(source, pointer_has_lifetime)
            },
            Ty::NullTerminatedString(_) => pointer_has_lifetime,
            Ty::Native(_) | Ty::StringArray(_) => false,
            Ty::Array(ty, _) => ty.has_lifetime(source, false),
            Ty::Named(named) => source
                .resolve_type(named)
                .expect("unknown type")
                .has_lifetime(source, pointer_has_lifetime),
        }
    }

    /// Checks whether the type is copy
    pub fn is_debug(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, _)
            | Ty::Native(_)
            | Ty::StringArray(_)
            | Ty::NullTerminatedString(_)
            | Ty::Slice(_, _, _) => true,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_debug(source),
            Ty::Array(ty, _) => ty.is_debug(source),
        }
    }

    /// Checks whether the type is copy
    pub fn is_copy(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, _)
            | Ty::Native(_)
            | Ty::StringArray(_)
            | Ty::NullTerminatedString(_)
            | Ty::Slice(_, _, _) => true,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_copy(source),
            Ty::Array(ty, _) => ty.is_copy(source),
        }
    }

    /// Checks whether a type is PartialEq and PartialOrd
    pub fn is_partial_eq(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, _) | Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => true,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_partial_eq(source),
            Ty::Array(ty, _) => ty.is_partial_eq(source),

            // slice **are** eq because while only their pointers are compared, structures always also
            // contain a "length" field which will also be compared, therefore slices of different lengths
            // will not be equal but their containers won't.
            Ty::Slice(_, _, _) => true,
        }
    }

    /// Checks whether a type is Eq and Ord
    pub fn is_eq(&self, source: &Source<'a>) -> bool {
        // additional case, shouldn't be necessary tho
        // TODO: remove this?
        if !self.is_partial_eq(source) {
            return false;
        }

        match self {
            Ty::Native(Native::Float) | Ty::Native(Native::Double) => false,
            Ty::Pointer(_, _) | Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => true,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_eq(source),
            Ty::Array(ty, _) => ty.is_eq(source),

            // slice **are** eq because while only their pointers are compared, structures always also
            // contain a "length" field which will also be compared, therefore slices of different lengths
            // will not be equal but their containers won't.
            Ty::Slice(_, _, _) => true,
        }
    }

    /// Checks whether the type is Hash
    pub fn is_hash(&self, source: &Source<'a>) -> bool {
        match self {
            // floats are not hash
            Ty::Native(Native::Float) | Ty::Native(Native::Double) => false,
            Ty::Pointer(_, _) | Ty::Native(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) => true,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_hash(source),
            Ty::Array(ty, _) => ty.is_hash(source),

            // slices are just raw pointers which impl hash!
            Ty::Slice(_, _, _) => true,
        }
    }

    /// Checks whether the type is (de)serializable
    pub fn is_serde(&self, source: &Source<'a>) -> bool {
        match self {
            // floats are not hash
            Ty::Native(_) | Ty::StringArray(_) => true,
            Ty::Pointer(_, _) | Ty::Slice(_, _, _) | Ty::NullTerminatedString(_) => false,
            Ty::Named(name) => source.resolve_type(name).expect("unknown type").is_serde(source),
            Ty::Array(ty, _) => ty.is_serde(source),
        }
    }

    /// Checks whether the type requires conversion to be a "rustified" type
    pub fn requires_conversion(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Native(_) => false,
            Ty::Named(Cow::Borrowed("VkBool32")) => true,
            Ty::Pointer(_, ty) => !ty.is_opaque(source),
            Ty::Named(ty) => source.resolve_type(ty).expect("unknown type").is_opaque(),
            Ty::StringArray(_) => false,
            Ty::NullTerminatedString(_) => true,
            Ty::Array(_, _) => false,
            Ty::Slice(_, ty, _) => !ty.is_opaque(source),
        }
    }

    /// Checks whether the type conversion is safe
    pub fn is_safe_conversion(&self) -> bool {
        match self {
            Ty::Native(_) | Ty::Named(_) | Ty::StringArray(_) | Ty::Array(_, _) => true,
            Ty::Pointer(_, _) | Ty::Slice(_, _, _) | Ty::NullTerminatedString(_) => false,
        }
    }

    /*/// Does the type have a generic (with deep checking)
    pub fn has_generics(&self, source: &Source<'a>) -> bool {
        match self {
            Ty::Pointer(_, ty) | Ty::Slice(_, ty, _) => !ty.has_opaque(source) && ty.has_generics(source),
            Ty::NullTerminatedString(_) | Ty::Native(_) | Ty::StringArray(_) => false,
            Ty::Array(ty, _) => !ty.has_opaque(source) && ty.has_generics(source),
            Ty::Named(named) => source.resolve_type(named).expect("unknown type").has_generics(source),
        }
    }*/

    /// Default tokens for a type
    pub fn default_tokens(&self, source: &Source<'a>, imports: Option<&Imports>, value: Option<&str>) -> TokenStream {
        match self {
            Ty::Pointer(mut_, _) | Ty::Slice(mut_, _, _) => match mut_ {
                Mutability::Mutable => quote! { std::ptr::null_mut() },
                Mutability::Const => quote! { std::ptr::null() },
            },
            Ty::Native(native) => native.default_tokens(),
            Ty::Named(Cow::Borrowed("VkBool32")) => quote! { 0 },
            Ty::Named(name) => source
                .resolve_type(name)
                .expect("unknown type")
                .default_tokens(source, imports, value),
            Ty::StringArray(len) => {
                let len = len.as_const_expr(source, imports);
                quote! { [0; #len as usize]}
            },
            Ty::NullTerminatedString(_) => quote! {
                std::ptr::null()
            },
            Ty::Array(ty, len) => {
                assert!(
                    ty.is_copy(source),
                    "cannot create a default value of a non-copy type: {:#?}",
                    self
                );

                let len = len.as_const_expr(source, imports);
                let default = ty.default_tokens(source, imports, None);
                quote! {
                    [#default; #len as usize]
                }
            },
        }
    }

    /// Creates a converter from the "raw" type to its "rustified type" as an inline call, only one
    /// layer deep so as to not allocate.
    pub fn rust_to_c_converter_inline(&self, value: &str) -> TokenStream {
        let value_ident = Ident::new(value, Span::call_site());
        match self {
            Ty::Named(Cow::Borrowed("VkBool32")) => quote! { #value_ident as u8 as u32 },
            Ty::Pointer(mutability, _) => {
                let ptr_mut = mutability.as_ptr_token();

                quote! {
                    #value_ident as *#ptr_mut _
                }
            },
            Ty::Native(_) | Ty::Named(_) | Ty::StringArray(_) | Ty::Array(_, _) => value_ident.to_token_stream(),
            Ty::NullTerminatedString(_) => quote! {
                #value_ident.as_ptr()
            },
            Ty::Slice(mutability, _, _) => match mutability {
                Mutability::Mutable => quote! {
                    #value_ident.as_mut_ptr()
                },
                Mutability::Const => quote! {
                    #value_ident.as_ptr()
                },
            },
        }
    }

    /// Creates a converter from the "raw" type to its "rustified type", only one layer deep
    /// so as to not allocate. Returns the converter and whether or not the output is a reference.
    /// TODO: make name more explicit
    pub fn rust_to_c_converter(
        &self,
        source: &Source<'a>,
        setter: &impl Fn(&str, TokenStream) -> TokenStream,
        type_of_field: &impl Fn(&str) -> Ty<'a>,
        field: &str,
        len_field: Option<&str>,
    ) -> Option<(Vec<TokenStream>, TokenStream)> {
        let mut fields = Vec::with_capacity(1);

        let value_ident = Ident::new("value", Span::call_site());
        let lt = lifetime_as_lifetime();

        let out = match self {
            Ty::Named(Cow::Borrowed("VkBool32")) => {
                fields.push(quote! { value: bool });
                setter(field, quote! { #value_ident as u8 as u32})
            },
            Ty::Native(_) | Ty::Named(_) | Ty::StringArray(_) | Ty::NullTerminatedString(_) | Ty::Array(_, _) => {
                let ty = self.as_raw_ty(source, None, false).0;
                fields.push(quote! { value: #ty });

                setter(field, value_ident.to_token_stream())
            },
            Ty::Pointer(mutability, ty) => {
                let ty = ty.as_raw_ty(source, None, false).0;
                let mut_ = mutability.as_ref_token();
                fields.push(quote! { value: &#lt #mut_ #ty});

                let ptr_mut = mutability.as_ptr_token();

                setter(
                    field,
                    quote! {
                       #value_ident as *#ptr_mut _
                    },
                )
            },
            Ty::Slice(mutability, ty, len) => {
                let ty = ty.as_raw_ty(source, None, false).0;
                let mut_ = mutability.as_ref_token();

                fields.push(quote! { value: &#lt #mut_ [#ty]});

                let value_setter = setter(
                    field,
                    match mutability {
                        Mutability::Mutable => quote! {
                            #value_ident.as_mut_ptr()
                        },
                        Mutability::Const => quote! {
                            #value_ident.as_ptr()
                        },
                    },
                );

                let vars = len.variables();
                if vars.is_empty() {
                    let len = len.as_const_expr(source, None);
                    quote! {
                        assert_eq!(value.len(), (#len) as usize);

                        #value_setter
                    }
                } else {
                    assert_eq!(vars.len(), 1, "more than one variable");

                    let ty = type_of_field(len_field.unwrap_or_else(|| &vars[0]));

                    if !ty.is_native() {
                        return None;
                    }

                    let ty = ty.as_const_ty(source, None);

                    let len_expr = len.pivot("len_", len_field.unwrap_or_else(|| &vars[0])).as_expr(
                        source,
                        &|_| quote! { len_ },
                        None,
                    );
                    let len_setter = setter(
                        len_field.unwrap_or_else(|| &vars[0]),
                        Ident::new("len_", Span::call_site()).to_token_stream(),
                    );

                    quote! {
                        let len_ = value.len() as #ty;
                        let len_ = #len_expr;

                        #value_setter
                        #len_setter
                    }
                }
            },
        };

        Some((fields, out))
    }

    /// Creates a converter from the "raw" type to its "rustified type", only one layer deep
    /// so as to not allocate. Returns the converter and whether or not the output is a reference.
    pub fn c_to_rust_converter(
        &self,
        source: &Source<'a>,
        mutability: Mutability,
        getter: TokenStream,
        len: Option<TokenStream>,
        unsafe_: bool,
    ) -> Option<(TokenStream, bool)> {
        let mut_ = mutability.as_ref_token();

        match self {
            Ty::Native(_) => {
                if mutability.is_const() {
                    Some((getter, false))
                } else {
                    Some((quote! { &#mut_ #getter }, true))
                }
            },
            Ty::Named(Cow::Borrowed("VkBool32")) => {
                match mutability {
                    Mutability::Mutable => {
                        if unsafe_ {
                            Some((
                                quote! {
                                    if cfg!(target_endian = "little") {
                                        &mut *(#getter as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()

                                    } else {
                                        // TODO: check that this is actually correct on a big endian system
                                        // don't even know if those exist in the wild, a problem for a future me
                                        eprintln!("Big-endianess has not been tested!");
                                        &mut *(#getter as *mut Bool32).cast::<u32>().cast::<u8>().add(3).cast::<bool>()
                                    }
                                },
                                true,
                            ))
                        } else {
                            Some((
                                quote! {
                                    unsafe {
                                        if cfg!(target_endian = "little") {
                                            &mut *(#getter as *mut Bool32).cast::<u32>().cast::<u8>().cast::<bool>()

                                        } else {
                                            // TODO: check that this is actually correct on a big endian system
                                            // don't even know if those exist in the wild, a problem for a future me
                                            eprintln!("Big-endianess has not been tested!");
                                            &mut *(#getter as *mut Bool32).cast::<u32>().cast::<u8>().add(3).cast::<bool>()
                                        }
                                    }
                                },
                                true,
                            ))
                        }
                    },
                    Mutability::Const => {
                        if unsafe_ {
                            Some((quote! { std::mem::transmute(#getter as u8)}, false))
                        } else {
                            Some((quote! { unsafe { std::mem::transmute(#getter as u8) }}, false))
                        }
                    },
                }
            },
            Ty::Named(_) => Some(if self.is_copy(source) && mutability.is_const() {
                (getter, false)
            } else {
                match mutability {
                    Mutability::Mutable => (quote! { &mut #getter}, true),
                    Mutability::Const => (quote! { &#getter}, true),
                }
            }),
            Ty::Pointer(this_mut, _) => Some((
                match (this_mut, mutability) {
                    (Mutability::Mutable, Mutability::Const) | (Mutability::Const, Mutability::Const) => quote! {
                        &*#getter
                    },
                    (Mutability::Mutable, Mutability::Mutable) => quote! {
                        &mut *#getter
                    },
                    (Mutability::Const, Mutability::Mutable) => {
                        return None;
                    },
                },
                true,
            )),
            Ty::StringArray(_) | Ty::Array(_, _) => Some((quote! { &#mut_ #getter }, true)),
            Ty::NullTerminatedString(_) => {
                if mutability.is_mut() {
                    return None;
                }

                Some((
                    quote! {
                        CStr::from_ptr(#getter)
                    },
                    false,
                ))
            },
            Ty::Slice(this_mut, _, this_len) => {
                let len = len.unwrap_or_else(|| this_len.as_const_expr(source, None).to_token_stream());

                Some((
                    match (this_mut, mutability) {
                        (Mutability::Mutable, Mutability::Const) | (Mutability::Const, Mutability::Const) => quote! {
                            std::slice::from_raw_parts(#getter, #len as usize)
                        },
                        (Mutability::Mutable, Mutability::Mutable) => quote! {
                            std::slice::from_raw_parts_mut(#getter, #len as usize)
                        },
                        (Mutability::Const, Mutability::Mutable) => {
                            return None;
                        },
                    },
                    true,
                ))
            },
        }
    }

    /// Turns a type into a tokenized raw C-compatible type and an optional lifetime argument
    pub fn as_raw_ty(
        &self,
        source: &Source<'a>,
        imports: Option<&Imports>,
        pointer_has_lifetime: bool,
    ) -> (TokenStream, bool) {
        match self {
            Ty::Native(_) | Ty::StringArray(_) => (self.as_const_ty(source, imports), false),
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_type(source, imports),
            Ty::NullTerminatedString(_) => (Native::NullTerminatedString.as_raw_type(imports), true),
            Ty::Array(ty, len) => {
                let (elem, lt) = ty.as_raw_ty(source, imports, pointer_has_lifetime);
                let len = len.as_const_expr(source, imports);
                let len = quote! {
                    #len as usize
                };

                (
                    quote! {
                        [#elem; #len]
                    },
                    lt,
                )
            },
            Ty::Pointer(mutability, ty) | Ty::Slice(mutability, ty, _) => {
                let mutability = mutability.as_ptr_token();
                let ty = ty.as_raw_ty(source, imports, false).0;

                (
                    quote! {
                        *#mutability #ty
                    },
                    pointer_has_lifetime
                )
                
            },
        }
    }

    /// Turns a type into a tokenized type and an optional lifetime argument
    pub fn as_ty(&self, source: &Source<'a>, imports: Option<&Imports>) -> (TokenStream, bool) {
        match self {
            Ty::Native(_) | Ty::StringArray(_) => (self.as_const_ty(source, imports), false),
            Ty::Pointer(_, ty) => (ty.as_raw_ty(source, imports, true).0, true),
            Ty::Named(Cow::Borrowed("VkBool32")) => (
                quote! { bool },
                false,
            ),
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_type(source, imports),
            Ty::NullTerminatedString(_) => (Native::NullTerminatedString.as_type(imports), true),
            Ty::Array(ty, len) => {
                let (elem, lt) = ty.as_raw_ty(source, imports, false);
                let len = len.as_const_expr(source, imports);
                (
                    quote! {
                        [#elem; #len as usize]
                    },
                    lt,
                )
            },
            Ty::Slice(_, ty, _) => {
                let elem = ty.as_raw_ty(source, imports, false).0;
                (
                    quote! {
                        [#elem]
                    },
                    true,
                )
            },
        }
    }

    /// Turns a type into a tokenized type
    pub(super) fn as_const_ty(&self, source: &Source<'a>, imports: Option<&Imports>) -> TokenStream {
        match self {
            Ty::Native(native) => native.as_type(imports),
            Ty::Pointer(mutability, ty) | Ty::Slice(mutability, ty, _) => {
                let mutability = mutability.as_ptr_token();
                let elem = ty.as_const_ty(source, imports);
                
                quote! {
                    *#mutability #elem
                }
            },
            Ty::Named(name) => source
                .find(name)
                .expect("type not found")
                .as_type_ref()
                .expect("not a type")
                .as_const_type(source, imports),
            Ty::StringArray(len) => {
                let len = len.as_const_expr(source, imports);
                let elem = Native::Char.as_type(None);
                
                quote! {
                    [#elem; #len as usize]
                }
            },
            Ty::Array(ty, len) => {
                let len = len.as_const_expr(source, imports);
                let elem = box ty.as_const_ty(source, imports);
                
                quote! {
                    [#elem; #len as usize]
                }
            },
            Ty::NullTerminatedString(_) => Native::NullTerminatedString.as_const_type(imports),
        }
    }
}

impl<'a: 'b, 'b> TypeRef<'a, 'b> {
    /// Turns a type reference into a tokenized type
    pub fn as_const_type(&self, source: &Source<'a>, imports: Option<&Imports>) -> TokenStream {
        assert!(
            !self.has_lifetime(source, false),
            "type cannot be made into static type"
        );

        if let Some(imports) = imports {
            self.import(imports);
            
            self.as_ident().to_token_stream()
        } else {
            let path = self.origin().as_path();
            let elem = self.as_ident().to_token_stream();

            quote! {
                #path::#elem
            }
        }
    }

    /// Turns a type reference into a tokenized type and a boolean designating whether it
    /// has a lifetime
    pub fn as_type(&self, source: &Source<'a>, imports: Option<&Imports>) -> (TokenStream, bool) {
        // special case for flattening aliases
        if let Some(alias) = self.as_alias() {
            return source
                .resolve_type(alias.of())
                .expect("unknown alias")
                .as_type(source, imports);
        }

        let lt = self.has_lifetime(source, true) && !self.is_opaque();
        let lifetime = lt.then(lifetime_as_generic_argument);

        let ident = self.as_ident();
        (if let Some(imports) = imports {
            self.import(imports);

            quote! {
                #ident #lifetime
            }
        } else {
            let path = self.origin().as_path();

            quote! {
                #path :: #ident #lifetime
            }
        }, lt)
    }

    /*/// Gets the list of generic type parameters
    pub fn generics(&self, source: &Source<'a>) -> SmallVec<[Ident; 1]> {
        match self {
            TypeRef::OpaqueType(_) | TypeRef::Union(_) => SmallVec::with_capacity(0),
            TypeRef::Alias(_) => todo!(),
            TypeRef::Struct(_) => todo!(),
            TypeRef::Handle(handle) => smallvec::smallvec![],
            TypeRef::FunctionPointer(_) => todo!(),
            TypeRef::Basetype(_) => todo!(),
            TypeRef::Bitmask(_) => todo!(),
            TypeRef::BitFlag(_) => todo!(),
            TypeRef::Enum(_) => todo!(),
        }
    }*/
}

impl Mutability {
    /// Returns the mutability for a pointer
    #[inline]
    pub fn as_ptr_token(&self) -> TokenStream {
        match self {
            Mutability::Mutable => quote! { mut },
            Mutability::Const => quote! { const },
        }
    }

    /// Returns the mutability for a pointer
    #[inline]
    pub fn as_ref_token(&self) -> Option<TokenStream> {
        match self {
            Mutability::Mutable => Some(quote! { mut }),
            Mutability::Const => None,
        }
    }
}

impl Native {
    /// Gets the native type has a dynamic type that will (if needed) use the global lifetime name
    pub fn as_type(&self, imports: Option<&Imports>) -> TokenStream {
        self.as_type_with_lifetime(lifetime_as_lifetime(), imports)
    }

    /// Gets the native type has a dynamic type that will (if needed) use the static lifetime
    pub fn as_const_type(&self, imports: Option<&Imports>) -> TokenStream {
        self.as_type_with_lifetime(quote! { 'static }, imports)
    }

    /// Gets the native type has a pointer type
    pub fn as_raw_type(&self, imports: Option<&Imports>) -> TokenStream {
        if let Some(imports) = imports {
            imports.push("std::os::raw::c_char");

            quote! {
                *const c_char
            }
        } else {
            quote! {
                *const std::os::raw::c_char
            }
        }
    }

    /// Gets a type from this native type, imports it if needed
    pub fn as_type_with_lifetime(&self, lifetime: TokenStream, imports: Option<&Imports>) -> TokenStream {
        if let Some(imports) = imports {
            self.import(imports);

            let ident = self.as_ident();
            match self {
                Self::NullTerminatedString => quote! {
                    &#lifetime #ident
                },
                _ => ident.to_token_stream(),
            }
        } else {
            match self {
                Self::NullTerminatedString => quote! {
                    &#lifetime std::ffi::CStr
                },
                _ => self.as_type_path()
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
                Native::Char => "c_char",
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
    pub fn as_type_path(&self) -> TokenStream {
        match self {
            Native::Void => quote! { std::ffi::c_void },
            Native::Char => quote! { std::os::raw::c_char},
            Native::UChar => quote! { std::os::raw::c_uchar },
            Native::NullTerminatedString => quote! { std::ffi::CStr },
            _ => self.as_ident().to_token_stream(),
        }
    }
}
