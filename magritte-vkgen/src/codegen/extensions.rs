use ahash::AHashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};
use tracing::warn;

use crate::{
    doc::Documentation,
    source::{Extension, ExtensionType, Source},
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

    /// Generates an extension set structure
    pub fn generate_extension_set<'b>(
        _source: &Source<'a>,
        exts: impl Iterator<Item = &'b Extension<'a>>,
        name: Ident,
        mut out: &mut TokenStream,
    ) where
        'a: 'b,
    {
        let exts = exts.map(|ext| (ext.name(), ext)).collect::<AHashMap<_, _>>();

        let names = exts.values().map(|ext| ext.name()).collect::<Vec<_>>();
        let extensions = exts
            .values()
            .map(|o| Ident::new(&o.origin().as_name(), Span::call_site()))
            .collect::<Vec<_>>();
        let enable_extensions = exts
            .values()
            .map(|o| Ident::new(&format!("enable_{}", o.origin().as_name()), Span::call_site()))
            .collect::<Vec<_>>();
        let get_docs = exts.values().map(|o| format!("Checks if `{}` is enabled", o.name()));
        let set_docs = exts
            .values()
            .map(|o| format!("Enables `{}` and it dependencies", o.name()));

        let dependencies = exts.values().map(|ext| {
            let idents = ext
                .required()
                .iter()
                .filter_map(|ext| exts.get(&**ext))
                .map(|o| Ident::new(&format!("enable_{}", o.origin().as_name()), Span::call_site()));

            quote! {
                self = self #(
                    .#idents()
                )*;
            }
        });

        let device_extensions = exts
            .values()
            .filter(|e| matches!(e.ty(), ExtensionType::Device))
            .map(|o| Ident::new(&o.origin().as_name(), Span::call_site()))
            .collect::<Vec<_>>();

        let device_ext_len = device_extensions.len();

        let device_names = exts
            .values()
            .filter(|e| matches!(e.ty(), ExtensionType::Device))
            .map(|ext| ext.name());

        let instance_extensions = exts
            .values()
            .filter(|e| matches!(e.ty(), ExtensionType::Instance))
            .map(|o| Ident::new(&o.origin().as_name(), Span::call_site()))
            .collect::<Vec<_>>();

        let instance_ext_len = device_extensions.len();

        let instance_names = exts
            .values()
            .filter(|e| matches!(e.ty(), ExtensionType::Instance))
            .map(|ext| ext.name());

        quote_each_token! {
            out

            #[doc = "A list of Vulkan extensions"]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct #name {
                version: Version,
                #(
                    #[cfg(feature = #names)]
                    pub #extensions: bool
                ),*
            }

            impl const Default for #name {
                fn default() -> Self {
                    Self {
                        version: Version::VULKAN1_0,
                        #(
                            #[cfg(feature = #names)]
                            #extensions: false
                        ),*
                    }
                }
            }

            impl #name {
                #[doc = "Creates an empty set of extensions with Vulkan v1.0"]
                pub const fn new() -> Self {
                    Self::default()
                }

                #[doc = "Creates an empty set of extensions with a vulkan version"]
                pub const fn from_version(version: Version) -> Self {
                    Self {
                        version,
                        .. Default::default()
                    }
                }

                #[doc = "Creates an empty set of extensions with Vulkan v1.0"]
                pub const fn vulkan1_0() -> Self {
                    Self::default()
                }

                #[doc = "Creates an empty set of extensions with Vulkan v1.1"]
                pub const fn vulkan1_1() -> Self {
                    Self {
                        version: Version::VULKAN1_1,
                        .. Default::default()
                    }
                }

                #[doc = "Creates an empty set of extensions with Vulkan v1.2"]
                pub const fn vulkan1_2() -> Self {
                    Self {
                        version: Version::VULKAN1_2,
                        .. Default::default()
                    }
                }

                #[doc = "Creates an empty set of extensions with Vulkan v1.3"]
                pub const fn vulkan1_3() -> Self {
                    Self {
                        version: Version::VULKAN1_3,
                        .. Default::default()
                    }
                }

                #[doc = "Gets the version of this extension set."]
                #[inline(always)]
                pub const fn version(&self) -> Version {
                    self.version
                }

                #(
                    #[doc = #set_docs]
                    #[cfg(feature = #names)]
                    pub const fn #enable_extensions(mut self) -> Self {
                        #dependencies

                        self.#extensions = true;

                        self
                    }

                    #[doc = #get_docs]
                    #[inline(always)]
                    #[cfg(feature = #names)]
                    pub const fn #extensions(&self) -> bool {
                        self.#extensions
                    }
                )*

                #[doc = "Gets the list of extension names to use when creating the [`crate::vulkan1_0::Device`"]
                pub fn device_extension_names(&self) -> Vec<&'static CStr> {
                    let mut out = Vec::with_capacity(#device_ext_len);

                    #(
                        if self.#device_extensions() {
                            out.push(cstr!(#device_names));
                        }
                    )*

                    out
                }

                #[doc = "Gets the list of extension names to use when creating the [`crate::vulkan1_0::Instance`"]
                pub fn instance_extension_names(&self) -> Vec<&'static CStr> {
                    let mut out = Vec::with_capacity(#instance_ext_len);

                    #(
                        if self.#instance_extensions() {
                            out.push(cstr!(#instance_names));
                        }
                    )*

                    out
                }
            }
        }
    }
}
