use magritte_build::imports::Imports;
use magritte_types::{Source, Ty};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::r#const::constant_value;

pub fn field_type<'a>(source: &Source<'a>, ty: &Ty<'a>, imports: &mut Imports) -> TokenStream {
    match ty {
        Ty::Pointer(mut_, ty) | Ty::Slice(mut_, ty, _) => {
            let ty = field_type(source, ty, imports);
            let mut_ = match mut_ {
                magritte_types::Mutability::Mutable => quote! { mut },
                magritte_types::Mutability::Const => quote! { const },
            };

            quote! {
                *#mut_ #ty
            }
        },
        Ty::Native(native) => native.to_token_stream(),
        Ty::Named(name) => {
            let ref_ = source.find_type(name).expect("unknown type");

            imports.push_origin(source, ref_.origin(), ref_.name());

            let ident = ref_.as_ident();

            quote! {
                #ident
            }
        },
        Ty::StringArray(len) => {
            imports.push("std::ffi::c_char");

            let len = constant_value(len, source, imports);
            quote! {
                [c_char; #len as usize]
            }
        },
        Ty::NullTerminatedString(mut_) => {
            imports.push("std::ffi::c_char");

            let mut_ = match mut_ {
                magritte_types::Mutability::Mutable => quote! { mut },
                magritte_types::Mutability::Const => quote! { const },
            };

            quote! {
                *#mut_ c_char
            }
        },
        Ty::Array(ty, len) => {
            let ty = field_type(source, ty, imports);
            let len = constant_value(len, source, imports);
            quote! {
                [#ty; #len as usize]
            }
        },
    }
}
