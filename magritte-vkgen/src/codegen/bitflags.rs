use ahash::AHashMap;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use tracing::warn;

use crate::{
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Bit, BitFlag, Source},
};

use super::alias_of;

impl<'a> Bit<'a> {
    /// Generate the code for a Bitflag variant
    fn generate_bitflag_variant(
        &self,
        source: &Source<'a>,
        parent: &Origin<'a>,
        doc: &AHashMap<String, String>,
    ) -> TokenStream {
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
        let conditional_compilation = self.condition(source, parent);

        quote! {
            #doc
            #provided_by
            #conditional_compilation
            pub const #name: Self = Self(#value);
        }
    }
}

impl<'a> BitFlag<'a> {
    /// Generates the code for a bitflag
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // import serde if needed
        imports.serde();

        // generate the doc for the bitflag
        let variant_docs = self.generate_doc(source, doc, out).unwrap_or_default();

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);
        // generate the code for each bit
        let bits = self
            .bits()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .map(|v| v.generate_bitflag_variant(source, self.origin(), &variant_docs));

        // get the underlying bit type
        let ty = match self.width() {
            4 => quote! { u32 },
            8 => quote! { u64 },
            _ => unreachable!("unknown bit width for a mask: {:?}", self),
        };

        quote::quote_each_token! {
            out

            #[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(transparent)]
            pub struct #name(#ty);

            impl const Default for #name {
                fn default() -> Self {
                    Self(0)
                }
            }

            impl #name {
                #(
                    #bits
                )*

                #[doc = "Default empty value"]
                #[inline]
                pub const fn empty() -> Self {
                    Self::default()
                }

                #[doc = "Gets the raw underlying value"]
                #[inline]
                pub const fn bits(&self) -> #ty {
                    self.0
                }

                #[doc = "Gets a value from a raw underlying value, unchecked and therefore unsafe."]
                #[doc = ""]
                #[doc = "# Safety"]
                #[doc = "The caller of this function must ensure that all of the bits are valid."]
                #[inline]
                pub const unsafe fn from_bits_unchecked(bits: #ty) -> Self {
                    Self(bits)
                }
            }
        }
    }

    /// Generates the documentation for a bitflag
    pub(super) fn generate_doc(
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
            let mut variants = AHashMap::with_capacity(self.bits().len());
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
