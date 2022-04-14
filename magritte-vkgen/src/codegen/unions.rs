use ahash::AHashMap;
use proc_macro2::TokenStream;
use quote::{quote, quote_each_token};
use tracing::warn;

use crate::{
    doc::Documentation,
    imports::Imports,
    source::{Source, Union},
};

use super::{alias_of, ty::lifetime_as_generic_argument};

impl<'a> Union<'a> {
    /// Generates the code for a union
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the name as an identifier
        let name = self.as_ident();

        // generate the derives
        let copy = self.is_copy(source).then(|| quote! { #[derive(Clone, Copy)] });

        // create the lifetime generic argument
        let lifetime = self.has_lifetime(source).then(lifetime_as_generic_argument);

        // get the documentation and the documentation of each field
        let field_doc = self.generate_doc(source, doc, out).unwrap_or_default();

        // creates the field list
        let fields = self
            .fields()
            .iter()
            .map(|field| field.generate_raw_code(source, imports, &field_doc));

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote_each_token! {
            out

            #copy
            #[repr(C)]
            pub union #name #lifetime {
                #(#fields),*
            }

            impl #lifetime Default for #name #lifetime {
                fn default() -> Self {
                    unsafe {
                        std::mem::zeroed()
                    }
                }
            }
        }
    }

    /// Generates the documentation for a constant
    fn generate_doc(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        out: &mut TokenStream,
    ) -> Option<AHashMap<String, String>> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the description section
            let mut fields = AHashMap::with_capacity(self.fields().len());

            // parse the members and write them out
            doc.members(source, self, out, Some(&mut fields));

            // parse the descirption and write it out
            // if we did not find the fields in `members`, try again here, this happens in some man pages
            doc.description(source, self, out, fields.is_empty().then(|| &mut fields));

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(fields)
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}
