//! # WASM code generation
//! This module handles code generation for WASM compatible types and functions.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    codegen::ty::lifetime_as_generic_argument,
    doc::Documentation,
    imports::Imports,
    source::{reference::TypeRef, Field, Source, Struct},
};

impl<'a> Field<'a> {
    /// Generates the type for the WASM-compatible field
    pub fn as_wasm_ty(&self, source: &Source<'a>, imports: &Imports, pointer_chain: Option<&Ident>) -> TokenStream {
        // get the name as an identifier of the field
        let name = self.as_ident();

        if self.is_p_next() && pointer_chain.is_some() {
            return quote! {
                #name: SmallVec<#pointer_chain>
            };
        }

        // get the type of the field
        let ty = self.ty().as_wasm_ty(source, Some(imports));

        quote! {
            pub #name: #ty
        }
    }
}

impl<'a> Struct<'a> {
    /// Generates the code for the WASM-compatible struct
    pub fn generate_wasm_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the name as an identifier
        let name = self.as_ident();

        // create the lifetime generic argument
        let lifetime = self.has_lifetime(source).then(lifetime_as_generic_argument);

        let enum_ident = (self
            .has_p_next()
            .is_some() && !self.extended().is_empty())
            .then(|| format_ident!("{}Chain", self.name()));

        let enum_def = enum_ident.as_ref().map(|enum_ident| {
            let extenders = self
                .extended()
                .iter()
                .map(|e| source.find(e).expect("unknown type").as_type_ref().expect("not a type"))
                .collect::<Vec<_>>();

            let enum_lifetime = extenders
                .iter()
                .any(|e| e.has_lifetime(source, true))
                .then(|| lifetime_as_generic_argument());

            let extender_idents = extenders.iter().map(TypeRef::as_ident)
            .collect::<Vec<_>>();
            let extender_lifetime = extenders
                .iter()
                .map(|e| e.has_lifetime(source, true).then(|| lifetime_as_generic_argument()))
                .collect::<Vec<_>>();
            quote! {
                pub enum #enum_ident #enum_lifetime {
                    #(
                        #extender_idents (#extender_idents #extender_lifetime)
                    ),*
                }

                #(
                    impl #enum_lifetime From<#extender_idents #extender_lifetime> for #enum_ident #enum_lifetime {
                        fn from(value: #extender_idents #extender_lifetime) -> Self {
                            Self:: #extender_idents (value)
                        }
                    }
                )*
            }
        });

        let field_ty = self
            .fields()
            .iter()
            .filter(|f| !f.is_s_type())
            .filter(|f| !(f.is_p_next() && enum_ident.is_none()))
            .map(|field| field.as_wasm_ty(source, imports, enum_ident.as_ref()));

        quote::quote_each_token! {
            out

            #enum_def

            pub struct #name #lifetime {
                #(
                    #field_ty
                ),*
            }


        }
    }
}
