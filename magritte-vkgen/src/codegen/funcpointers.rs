use proc_macro2::TokenStream;
use quote::quote;
use tracing::warn;

use crate::{
    codegen::{alias_of, ty::lifetime_as_generic_argument},
    doc::Documentation,
    imports::Imports,
    source::{FunctionPointer, FunctionPointerArgument, Source},
};

impl<'a> FunctionPointer<'a> {
    /// Generates the code for a function pointer
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the name as an identifier
        let name = self.as_ident();

        // generate the documentation
        self.generate_doc(source, doc, out);

        // creates the field list
        let args = self
            .arguments()
            .iter()
            .map(|field| field.generate_funcpointer_arg(source, imports).0);

        let has_lifetime = self.has_lifetime(source);

        // create the lifetime generic argument
        let lifetime = has_lifetime.then(|| {
            let lt = lifetime_as_generic_argument();
            quote! {
                for #lt
            }
        });

        // creates the return type
        let ret = self.return_type().map(|ty| {
            let (ty, _) = ty.as_raw_ty(source, Some(imports), false);

            quote! {
                -> #ty
            }
        });

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            pub type #name = Option<#lifetime unsafe extern "system" fn(#(#args),*) #ret>;
        }
    }

    /// Generates the documentation for a constant
    pub fn generate_doc(&self, source: &Source<'a>, doc: &mut Documentation, out: &mut TokenStream) -> Option<()> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the parameters and write them out
            doc.parameters(source, self, out, None);

            // parse the descirption and write it out
            // if we did not find the fields in `members`, try again here, this happens in some man pages
            doc.description(source, self, out, None);

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

impl<'a> FunctionPointerArgument<'a> {
    pub(super) fn generate_funcpointer_arg(&self, source: &Source<'a>, imports: &Imports) -> (TokenStream, bool) {
        // get the name as an identifier
        let name = self.as_ident();

        let (ty, lt) = self.ty().as_raw_ty(source, Some(imports), false);

        (
            quote! {
                #name: #ty
            },
            lt,
        )
    }
}
