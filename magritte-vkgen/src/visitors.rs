mod doc;
mod features;
mod handle;
mod native;
mod wasm;
mod types;

pub use doc::*;
pub use features::*;
pub use handle::*;
pub use native::*;
pub use types::*;
pub use wasm::*;

use smallvec::SmallVec;
use proc_macro2::TokenStream;
use quote::quote;

pub fn doc_path(original_name: &str) -> TokenStream {
    let path = format!("./doc/{original_name}.md");

    quote! {
        #[doc = include_str!(#path)]
    }
}

pub fn field_doc(original_name: &str, field: &str) -> TokenStream {
    let path = format!("./doc/{original_name}${field}.md");

    quote! {
        #[doc = include_str!(#path)]
    }
}

pub fn field_docs<'a>(
    doc: Option<&BuiltDoc>,
    original_name: &str,
    fields: impl IntoIterator<Item = &'a str>,
) -> SmallVec<[Option<TokenStream>; 8]> {
    fields
        .into_iter()
        .map(|field| {
            if let Some(members) = doc.and_then(|doc| doc.members.as_ref()) {
                members.get(field).map(|_| field_doc(original_name, field))
            } else {
                None
            }
        })
        .collect()
}
