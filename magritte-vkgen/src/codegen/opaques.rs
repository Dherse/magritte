use proc_macro2::TokenStream;
use tracing::warn;

use crate::{
    doc::{Documentation, Queryable},
    imports::Imports,
    source::{OpaqueType, Source},
};

impl<'a> OpaqueType<'a> {
    /// Generates the code for an opaque type
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // append the doc first
        self.generate_doc(source, doc, out);

        // import the c_void type
        imports.push("std::ffi::c_void");

        if self.requires().starts_with("vk_video/vulkan_video_codec_") {
            quote::quote_each_token! {
                out

                pub type #name = crate::video::#name;
            }
        } else {
            quote::quote_each_token! {
                out

                pub type #name = c_void;
            }
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
            Documentation::no_doc(out);

            None
        }
    }
}

impl<'a> Queryable<'a> for OpaqueType<'a> {
    fn find(&self, _: &Source<'a>, _: &str) -> Option<&'a str> {
        None
    }
}
