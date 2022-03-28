use proc_macro2::TokenStream;
use quote::quote;
use tracing::warn;

use crate::{
    codegen::alias_of,
    doc::Documentation,
    source::{Handle, Source},
};

impl<'a> Handle<'a> {
    /// Generates the code for a handle
    pub(super) fn generate_code(&self, source: &Source<'a>, doc: &mut Documentation, mut out: &mut TokenStream) {
        // the name as an identifier
        let name = self.as_ident();

        // the type of the constant
        let ty = if self.dispatchable() {
            quote! { *mut () }
        } else {
            quote! { u64 }
        };

        let default = if self.dispatchable() {
            quote! { std::ptr::null_mut() }
        } else {
            quote! { 0 }
        };

        // append the doc first
        self.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            #[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[repr(transparent)]
            pub struct #name(pub #ty);

            impl #name {
                #[doc = "Creates a new null handle"]
                #[inline]
                pub const fn null() -> Self {
                    Self(#default)
                }

                #[doc = "Checks if this is a null handle"]
                #[inline]
                pub fn is_null(&self) -> bool {
                    self == &Self::null()
                }

                #[doc = "Gets the raw value"]
                #[inline]
                pub fn raw(&self) -> #ty {
                    self.0
                }
            }

            unsafe impl Send for #name {}

            impl Default for #name {
                fn default() -> Self {
                    Self::null()
                }
            }
        }
    }

    /// Generates the documentation for a handle
    fn generate_doc(&self, source: &Source<'a>, doc: &mut Documentation, out: &mut TokenStream) -> Option<()> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(())
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}
