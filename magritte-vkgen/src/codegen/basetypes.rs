use proc_macro2::TokenStream;
use tracing::warn;

use crate::{source::{Basetype, Source}, doc::{Documentation, Queryable}, imports::Imports, codegen::alias_of};

impl<'a> Basetype<'a> {
    /// Generates the code for a base type
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // the type of the base type
        let ty = self.of().as_const_ty(source, Some(imports));

        // append the doc first
        self.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub type #name = #ty;
        }
    }

    /// Generates the documentation for a base type
    fn generate_doc(&self, source: &Source<'a>, doc: &mut Documentation, mut out: &mut TokenStream) -> Option<()> {
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

            quote::quote_each_token! {
                out

                #[doc = "This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)."]
                #[doc = "See the module level documentation where a description may be given."]
                #[doc = "You can also check the [Vulkan Specification]()."]
            }

            None
        }
    }
}

impl<'a> Queryable for Basetype<'a> {
    fn find(&self, _: &str) -> Option<&str> {
        None
    }
}