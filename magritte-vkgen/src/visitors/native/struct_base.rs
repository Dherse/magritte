use proc_macro2::TokenStream;
use quote::{quote_each_token, format_ident};
use smallvec::SmallVec;

use crate::{
    imports::Imports,
    source::{Field, Source, Struct},
    visitors::{doc_path, field_docs, DocVisitor, NativeType},
};

pub(crate) fn base_struct<'a>(
    doc: &DocVisitor,
    source: &Source<'a>,
    struct_: &Struct<'a>,
    mut out: &mut TokenStream,
    imports: &Imports,
) {
    let name = struct_.as_ident();
    let alias = struct_.original_name();

    let doc = doc.find(struct_.name());

    let struct_doc = doc.map(|_| doc_path(struct_.original_name()));
    let field_docs = field_docs(doc, struct_.original_name(), struct_.fields().iter().map(Field::original_name));

    let field_idents = struct_.fields().iter().map(Field::as_ident).collect::<SmallVec<[_; 8]>>();
    let field_tys = struct_.fields().iter().map(|f| NativeType(f.ty(), source, imports)).collect::<SmallVec<[_; 8]>>();
    
    let with_idents = struct_.fields().iter().map(|field| format_ident!("with_{}", field.name()));
    let with_docs = struct_.fields().iter().map(|field| format!("Builder method for setting `{}`", field.name()));
    
    let getter_docs = struct_.fields().iter().map(|field| format!("Gets `{}`", field.name()));
    let field_getters_mut = struct_.fields().iter().map(|field| format_ident!("{}_mut", field.name()));

    let setter_docs = struct_.fields().iter().map(|field| format!("Sets `{}`", field.name()));
    let field_setters = struct_.fields().iter().map(|field| format_ident!("set_{}", field.name()));

    quote_each_token! {
        out

        #[repr(C)]
        #[doc(alias = #alias)]
        #struct_doc
        #[derive(Clone, Copy)]
        pub struct #name {
            #(
                #field_docs
                #field_idents: #field_tys
            ),*
        }

        impl Default for #name {
            fn default() -> Self {
                unsafe {
                    // Vulkan structs can be zero-initialized and are generally valid.
                    // This makes sure that all pointers, values, etc. are zeros and therefore NULL.
                    std::mem::MaybeUninit::zeroes().assume_init()
                }
            }
        }

        impl #name {
            #(
                #[doc = #with_docs]
                pub fn #with_idents(mut self, value: #field_tys) -> Self {
                    self.#field_idents = value;

                    self
                }

                #[doc = #getter_docs]
                pub fn #field_idents(&self) -> &#field_tys {
                    &self.#field_idents
                }

                #[doc = #getter_docs]
                pub fn #field_getters_mut(&self) -> &mut #field_tys {
                    &mut self.#field_idents
                }

                #[doc = #setter_docs]
                pub fn #field_setters(&mut self, value: #field_tys) -> &mut Self {
                    self.#field_idents = value;

                    self
                }
            )*
        }
    }
}
