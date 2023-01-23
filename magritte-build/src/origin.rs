use magritte_types::{Origin, Source, SymbolName};
use proc_macro2::TokenStream;
use quote::quote;

pub fn cond_of<'a>(source: &Source<'a>, this: &Origin<'a>, of: &Origin<'a>) -> Option<TokenStream> {
    (of != this && !this.requires(source, of) && !of.always() && !of.is_opaque()).then(|| {
        let feature = of.name();
        quote! {
            #[cfg(feature = #feature)]
        }
    })
}