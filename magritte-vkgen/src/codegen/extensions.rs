use proc_macro2::TokenStream;
use tracing::warn;

use crate::{
    doc::Documentation,
    source::{Extension, Source},
};

impl<'a> Extension<'a> {
    /// Generates the module-level documentation for an extension
    pub fn generate_mod_doc(&self, source: &Source<'a>, doc: &mut Documentation, out: &mut TokenStream) {
        if let Some(mut doc) = doc.find(self.name()) {
            doc.set_mod_level_doc();

            // parse the name section and write it out
            doc.name(source, &(), out);

            // parse the description section and write it out
            doc.description(source, &(), out, None);

            // parse the documentation section**s** only found in extension level docs
            doc.extension(source, &(), out);

            // parse the related item section and write it out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);
        } else {
            warn!("No documentation for {}", self.name());

            // add the default no doc comment
            Documentation::no_doc(out);
        }
    }
}
