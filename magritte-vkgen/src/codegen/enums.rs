use ahash::AHashMap;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use tracing::warn;

use crate::{
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Bit, Enum, Source},
};

use super::alias_of;

impl<'a> Bit<'a> {
    fn generate_enum_variant(&self, parent: &Origin<'a>, doc: &AHashMap<String, String>) -> TokenStream {
        // get the doc of the bit
        let doc = doc.get(self.name()).map_or_else(
            || quote! { #[doc = "No documentation found"]},
            |t| quote! { #[doc = #t] },
        );

        // get the "provided by" of the bit
        let provided_by = (parent != self.origin()).then(|| {
            let path = self.origin().as_path_str();
            let doc = format!("\nProvided by [`{}`]", path);
            quote! {
                #[doc = #doc]
            }
        });

        // get the name
        let name = self.as_ident();
        let value = Literal::i64_unsuffixed(self.value());

        // conditional compilation for feature flags
        let conditional_compilation = self.condition(parent);

        quote! {
            #doc
            #provided_by
            #conditional_compilation
            #name = #value,
        }
    }
}

impl<'a> Enum<'a> {
    /// Generates the code for an enum
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // import serde and bytemuck if needed
        imports.serde();
        imports.bytemuck();

        // generate the doc for the enum
        let variant_docs = self.generate_doc(source, doc, out).unwrap_or_default();

        let has_empty = self
            .variants()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .any(|v| v.value() == 0);

        // create an empty declaration if none exists
        let empty_decl = (!has_empty).then(|| {
            quote! {
                #[doc(hidden)]
                Empty = 0,
            }
        });

        // get the default (empty) value or use the empty declaration if non exists
        let default = self
            .variants()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .find(|v| v.value() == 0)
            .map_or_else(|| Ident::new("Empty", Span::call_site()), |v| v.as_ident());

        // generate the code for each bit
        let bits = self
            .variants()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .map(|v| v.generate_enum_variant(self.origin(), &variant_docs));

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote::quote_each_token! {
            out

            #[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[non_exhaustive]
            #[repr(i32)]
            pub enum #name {
                #empty_decl
                #(#bits)*
            }

            impl const Default for #name {
                fn default() -> Self {
                    Self::#default
                }
            }

            impl #name {
                #[doc = "Default empty value"]
                #[inline]
                pub const fn empty() -> Self {
                    Self::default()
                }

                #[doc = "Gets the raw underlying value"]
                #[inline]
                pub const fn bits(&self) -> i32 {
                    *self as i32
                }

                #[doc = "Gets a value from a raw underlying value, unchecked and therefore unsafe"]
                #[inline]
                pub const unsafe fn from_bits(bits: i32) -> i32 {
                    std::mem::transmute(bits)
                }
            }
        }
    }

    /// Generates the documentation for an enum
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
            let mut variants = AHashMap::with_capacity(self.variants().len());
            doc.description(source, self, out, Some(&mut variants));

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(variants)
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}
