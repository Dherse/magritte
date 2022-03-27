use ahash::AHashMap;
use proc_macro2::TokenStream;
use quote::{quote, quote_each_token};
use tracing::warn;

use crate::{
    codegen::ty::{lifetime_as_generic_argument, lifetime_as_lifetime},
    doc::Documentation,
    imports::Imports,
    source::{Field, Source, Struct},
};

impl<'a> Struct<'a> {
    /// Generates the code for the RAW C-compatible struct
    pub fn generate_raw_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the name as an identifier
        let name = self.as_ident();

        // generate the derives
        let copy = self.is_copy(source).then(|| quote! { #[derive(Copy)] });
        let partial_eq_ord = self
            .is_copy(source)
            .then(|| quote! { #[derive(PartialEq, PartialOrd)] });
        let eq_ord = self.is_eq(source).then(|| quote! { #[derive(Eq, Ord)] });
        let hash = self.is_hash(source).then(|| quote! { #[derive(Hash)] });

        // create the lifetime generic argument
        let lt = lifetime_as_generic_argument();
        let lifetime = self.has_lifetime(source).then(|| {
            quote! {
                <#lt>
            }
        });

        // create a transparent, zero-sized field if there is a lifetime
        let lifetime_field = self.has_lifetime(source).then(|| {
            imports.push("std::marker::PhantomData");

            let lt = lifetime_as_lifetime();
            quote! {
                _lifetime: PhantomData<&#lt ()>,
            }
        });

        // get the documentation and the documentation of each field
        let field_doc = self.generate_doc(source, doc, out).unwrap_or_default();

        // creates the field list
        let fields = self
            .fields()
            .iter()
            .map(|field| field.generate_raw_code(source, imports, &field_doc));

        quote_each_token! {
            out


            #[derive(Clone, Debug)]
            #copy
            #eq_ord
            #partial_eq_ord
            #hash
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(C)]
            pub struct #name #lifetime {
                #lifetime_field
                #(#fields),*
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

impl<'a> Field<'a> {
    /// Generates the code for the RAW C-compatible struct
    pub(super) fn generate_raw_code(&self, source: &Source<'a>, imports: &Imports, doc: &AHashMap<String, String>) -> TokenStream {
        // get the name as an identifier of the field
        let name = self.as_ident();

        // get the type of the field
        let ty = self.ty().as_raw_ty(source, Some(imports)).0;

        // get the doc of the field
        let doc = doc.get(self.name()).map_or_else(
            || quote! { #[doc = "No documentation found"]},
            |t| quote! { #[doc = #t] },
        );

        quote! {
            #doc
            #name: #ty
        }
    }
}
