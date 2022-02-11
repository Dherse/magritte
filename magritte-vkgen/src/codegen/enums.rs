use ahash::AHashMap;
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use tracing::warn;

use crate::{
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Alias, Bit, Enum, Source},
};

use super::alias_of;

impl<'a> Alias<'a> {
    fn generate_code(&self, parent: &Origin<'a>, doc: &AHashMap<String, String>) -> TokenStream {
        // get the doc of the bit
        let doc = doc
            .get(self.of())
            .map(|t| quote! { #[doc = #t] })
            .unwrap_or_else(|| quote! { #[doc = "No documentation found"]});

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
        let of = Ident::new(self.of(), Span::call_site());

        quote! {
            #doc
            #provided_by
            pub const #name: Self = Self::#of;
        }
    }
}

impl<'a> Bit<'a> {
    fn generate_code(&self, parent: &Origin<'a>, doc: &AHashMap<String, String>) -> TokenStream {
        // get the doc of the bit
        let doc = doc
            .get(self.name())
            .map(|t| quote! { #[doc = #t] })
            .unwrap_or_else(|| quote! { #[doc = "No documentation found"]});

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

        quote! {
            #doc
            #provided_by
            pub const #name: Self = Self(#value);
        }
    }

    fn as_debug_match_arm(&self) -> TokenStream {
        let ident = self.as_ident();
        let name = self.name();

        quote! {
            Self::#ident => &#name
        }
    }
}

impl<'a> Enum<'a> {
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

        // import serde and bytemuck if needed
        imports.serde();
        imports.bytemuck();

        // generate the doc for the enum
        let variant_docs = self.generate_doc(source, doc, out).unwrap_or_default();

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        // generate the code for each bit
        let bits = self
            .variants()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .map(|v| v.generate_code(self.origin(), &variant_docs));

        // generate the code for each alias
        let aliases = self
            .aliases()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .map(|v| v.generate_code(self.origin(), &variant_docs));

        // generate the debug match arms of each bit
        let debugs = self
            .variants()
            .iter()
            .filter(|v| !v.origin().is_disabled())
            .map(Bit::as_debug_match_arm);

        let name_as_str = self.name();
        let invalid_value_str = format!("invalid value for `{}`: {{:?}}", name_as_str);
        quote::quote_each_token! {
            out

            #[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
            #[repr(C)]
            pub struct #name(i32);

            impl const Default for #name {
                fn default() -> Self {
                    Self(0)
                }
            }

            impl std::fmt::Debug for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                    f.debug_tuple(#name_as_str)
                        .field(match *self {
                            #(#debugs,)*
                            other => unreachable!(#invalid_value_str, other),
                        })
                        .finish()
                }
            }

            impl #name {
                #(#bits)*
                #(#aliases)*

                #[doc = "Default empty value"]
                #[inline]
                pub const fn empty() -> Self {
                    Self::default()
                }

                #[doc = "Gets the raw underlying value"]
                #[inline]
                pub const fn bits(&self) -> i32 {
                    self.0
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
            doc.no_doc(out);

            None
        }
    }
}
