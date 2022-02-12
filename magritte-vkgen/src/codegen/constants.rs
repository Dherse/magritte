use proc_macro2::TokenStream;
use tracing::warn;

use crate::{
    codegen::alias_of,
    doc::{DocRef, Documentation},
    imports::Imports,
    source::{Const, ConstAlias, Source},
};

impl<'a> Const<'a> {
    /// Generates the code for a constant
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // the type of the constant
        let ty = self.ty().as_const_ty(source, Some(imports));

        // the value of the constant
        let value = self.value().as_const_expr(source, Some(imports));

        // append the doc first
        self.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub const #name: #ty = #value;
        }
    }

    /// Generates the documentation for a constant
    fn generate_doc(&self, source: &Source<'a>, doc: &mut Documentation, out: &mut TokenStream) -> Option<()> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            DocRef::copyright(out);

            Some(())
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}

impl<'a> ConstAlias<'a> {
    /// Generates the code for a constant
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the identifier of the constant
        let name = self.as_ident();

        // find the original constant
        let of = source.constants.get_by_name(self.of()).expect("unknown constant");

        // the identifier of the original constant
        let of_ident = of.as_ident();

        // imports the original constant
        imports.push_origin(of.origin(), of.name());

        // get the type of the constant
        let ty = of.ty().as_const_ty(source, Some(imports));

        // generate the documentation based on the original constant
        of.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub const #name: #ty = #of_ident;
        }
    }
}
