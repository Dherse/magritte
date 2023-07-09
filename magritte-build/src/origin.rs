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

pub fn hl_cond_of<'a>(source: &Source<'a>, this: &Origin<'a>, of: &Origin<'a>) -> Option<TokenStream> {
    (of != this && !this.requires(source, of) && !of.always())
        .then(|| {
            let feature = of.name();
            quote! {
                #[cfg(feature = #feature)]
            }
        })
        .or_else(|| of.is_opaque().then(|| quote! { #[cfg(not(feature = "native"))] }))
}
