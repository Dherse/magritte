use ahash::AHashMap;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};
use tracing::warn;

use crate::{
    doc::Documentation,
    source::{Extension, ExtensionType, Source, DeprecationStatus}, origin::Origin,
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

    fn generate_deprecation(&self) -> Option<TokenStream> {
        match self.deprecation_status() {
            DeprecationStatus::Current => None,
            DeprecationStatus::Promoted(other) => {
                let note = format!("This extensions was promoted as part of `{}`", other);
                Some(quote! {
                    #[deprecated = #note]
                })
            },
            DeprecationStatus::Deprecated(other) => {
                let note = format!("This extensions was deprecated by `{}`", other);
                Some(quote! {
                    #[deprecated = #note]
                })
            },
            DeprecationStatus::Obsoleted(other) => {
                let note = format!("This extensions was made obsolete by `{}`", other);
                Some(quote! {
                    #[deprecated = #note]
                })
            },
            DeprecationStatus::PromotedVersion(_) | DeprecationStatus::DeprecatedVersion(_) | DeprecationStatus::ObsoletedVersion(_) => None,
        }
    }

    fn generate_field_code(
        &self,
    ) -> TokenStream {
        let ident = self.as_ident();
        let name = self.name();

        quote! {
            #[cfg(feature = #name)]
            pub #ident: bool
        }
    }

    fn generate_initialization(
        &self
    ) -> TokenStream {
        let ident = self.as_ident();
        let name = self.name();

        quote! {
            #[cfg(feature = #name)]
            #ident: false
        }
    }

    fn generate_getter_function(
        &self
    ) -> TokenStream {
        let ident = self.as_ident();
        let name = self.name();

        quote! {
            #[cfg(feature = #name)]
            #[inline]
            pub fn #ident(&self) -> bool {
                self.#ident
            }
        }
    }

    fn generate_setter_function(
        &self,
        source: &Source<'a>,
    ) -> TokenStream {
        let ident = self.as_ident();
        let enable_ident = self.as_enable_ident();
        let name = self.name();

        let deprecation = self.generate_deprecation();

        let max_version = match self.deprecation_status() {
            DeprecationStatus::PromotedVersion(version) 
            | DeprecationStatus::DeprecatedVersion(version) 
            | DeprecationStatus::ObsoletedVersion(version) => {
                let max = match version {
                    Origin::Vulkan1_0 => quote! { Version::VULKAN1_0 },
                    Origin::Vulkan1_1 => quote! { Version::VULKAN1_1 },
                    Origin::Vulkan1_2 => quote! { Version::VULKAN1_2 },
                    Origin::Vulkan1_3 => quote! { Version::VULKAN1_3 },
                    other => unreachable!("not a vulkan version: {:?}", other)
                };

                Some(quote! {
                    if self.version() >= #max {
                        return self;
                    }
                })
            },
            _ => None,
        };

        let min_version = match self.min_core() {
            Origin::Vulkan1_0 => None,
            Origin::Vulkan1_1 => Some(quote! { assert!(self.version() >= Version::VULKAN1_1) }),
            Origin::Vulkan1_2 => Some(quote! { assert!(self.version() >= Version::VULKAN1_2) }),
            Origin::Vulkan1_3 => Some(quote! { assert!(self.version() >= Version::VULKAN1_3) }),
            other => unreachable!("not a vulkan version: {:?}", other)
        };

        let dependencies = self.required().iter()
            .filter_map(|other| source.extensions.get_by_name(other))
            .filter(|other| !other.disabled())
            .filter(|other| other.ty() == self.ty())
            .map(|other| other.as_enable_ident());

        quote! {
            #[cfg(feature = #name)]
            #[inline]
            #deprecation
            pub fn #enable_ident(mut self) -> Self {
                #max_version

                #min_version

                #(
                    self = self.#dependencies();
                )*

                self.count += 1;
                self.#ident = true;

                self
            }
        }
    }

    fn generate_name_code(&self) -> TokenStream {
        let ident = self.as_ident();
        let name = self.name();

        quote! {
            #[cfg(feature = #name)]
            if self.#ident() {
                out.push(cstr_ptr!(#name));
            }
        }
    }

    /// Generate the code for extensions of a certain type
    pub fn generate_extensions<'b>(
        source: &Source<'a>,
        extension_type: ExtensionType,
        mut out: &mut TokenStream,
    ) where
        'a: 'b,
    {
        let exts = source.extensions.iter()
            .filter(|ext| !ext.disabled())
            .filter(|ext| ext.ty() == extension_type)
            .collect::<Vec<_>>();

        let name = match extension_type {
            ExtensionType::Device => Ident::new("DeviceExtensions", Span::call_site()),
            ExtensionType::Instance => Ident::new("InstanceExtensions", Span::call_site()),
        };

        let fields = exts.iter().map(|ext| ext.generate_field_code());
        let inits = exts.iter().map(|ext| ext.generate_initialization());
        let getters = exts.iter().map(|ext| ext.generate_getter_function());
        let setters = exts.iter().map(|ext| ext.generate_setter_function(source));
        let extension_names = exts.iter().map(|ext| ext.generate_name_code());
        
        quote_each_token! {
            out

            #[doc = "A list of Vulkan extensions"]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct #name {
                pub version: Version,
                pub count: usize,
                #(
                    #fields
                ),*
            }

            impl const Default for #name {
                fn default() -> Self {
                    Self {
                        version: Version::VULKAN1_0,
                        count: 0,
                        #(
                            #inits
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
                    #getters
                    #setters
                )*

                #[doc = "Gets the list of extension names"]
                pub fn extension_names(&self) -> Vec<*const c_char> {
                    let mut out = Vec::with_capacity(self.count);

                    #(
                        #extension_names
                    )*

                    out
                }
            }
        }
    }
}
