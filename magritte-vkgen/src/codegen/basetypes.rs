use proc_macro2::TokenStream;
use tracing::warn;

use crate::{
    codegen::alias_of,
    doc::{Documentation, Queryable},
    imports::Imports,
    source::{Basetype, Source},
};

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
            doc.no_doc(out);

            None
        }
    }
}

impl<'a> Queryable for Basetype<'a> {
    fn find(&self, _: &str) -> Option<&str> {
        None
    }
}
