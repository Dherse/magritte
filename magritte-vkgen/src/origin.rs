//! # Origin
//! An origin is **where** a Vulkan spec element comes from.
//! This can be the base spec, a specific Vulkan version or an extension.

use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::PathSegment;

use crate::symbols::SymbolName;

/// The origin of an element of the Vulkan spec
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Origin<'a> {
    /// The origin is unknown
    Unknown,

    /// The core Vulkan specification
    Core,

    /// An extension with its name and whether it is disabled or not
    Extension(Cow<'a, str>, i64, bool),

    /// Vulkan 1.0
    Vulkan1_0,

    /// Vulkan 1.1
    Vulkan1_1,

    /// Vulkan 1.2
    Vulkan1_2,

    /// Vulkan 1.3
    Vulkan1_3,

    /// An opaque external library
    Opaque,
}

impl<'a> Origin<'a> {
    /// Checks whether a string matches a Vulkan core version string or not
    pub fn match_version(s: &str) -> bool {
        matches!(
            s,
            "VK_VERSION_1_0" | "1.0" | "VK_VERSION_1_1" | "1.1" | "VK_VERSION_1_2" | "1.2" | "VK_VERSION_1_3" | "1.3"
        )
    }

    /// Creates a new origin from a core version string
    pub fn from_core(value: &str) -> Self {
        match value {
            "VK_VERSION_1_0" | "1.0" => Self::Vulkan1_0,
            "VK_VERSION_1_1" | "1.1" => Self::Vulkan1_1,
            "VK_VERSION_1_2" | "1.2" => Self::Vulkan1_2,
            "VK_VERSION_1_3" | "1.3" => Self::Vulkan1_3,
            s => unreachable!("unknown vulkan version: {}", s),
        }
    }

    /// Is the origin unknown
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Is the origin an extension
    pub const fn is_extension(&self) -> bool {
        matches!(self, Self::Extension(_, _, _))
    }

    /// Is the origin the base Vulkan spec
    pub const fn is_vulkan(&self) -> bool {
        matches!(
            self,
            Self::Vulkan1_0 | Self::Vulkan1_1 | Self::Vulkan1_2 | Self::Vulkan1_3
        )
    }

    /// Is the origin disabled
    /// If this is an extension, whether or not it is disabled
    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Unknown | Self::Extension(_, _, true))
    }

    /// Is the origin an opaque external type
    pub const fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque)
    }

    /// Gets a token stream reprsenting the path to the module of this origin
    pub fn as_path_tokens(&self) -> TokenStream {
        match self {
            Origin::Unknown => panic!("Unknown origin cannot be turned into a module"),
            Origin::Core => quote! { crate::core },
            Origin::Extension(ext, _, _) => {
                let ext = Ident::new(&ext.to_case(Case::Snake), Span::call_site());

                quote! { crate::extensions::#ext }
            },
            Origin::Vulkan1_0 => quote! { crate::vulkan1_0 },
            Origin::Vulkan1_1 => quote! { crate::vulkan1_1 },
            Origin::Vulkan1_2 => quote! { crate::vulkan1_2 },
            Origin::Vulkan1_3 => quote! { crate::vulkan1_3 },
            Origin::Opaque => quote::quote! { crate::native },
        }
    }

    /// Gets a string representing the path to the module of this origin
    pub fn as_path_str(&self) -> String {
        match self {
            Origin::Unknown => panic!("Unknown origin cannot be turned into a module"),
            Origin::Core => "crate::core".to_owned(),
            Origin::Extension(ext, _, _) => format!(
                "crate::extensions::{}",
                ext.trim_start_matches("VK_").to_case(Case::Snake)
            ),
            Origin::Vulkan1_0 => "crate::vulkan1_0".to_owned(),
            Origin::Vulkan1_1 => "crate::vulkan1_1".to_owned(),
            Origin::Vulkan1_2 => "crate::vulkan1_2".to_owned(),
            Origin::Vulkan1_3 => "crate::vulkan1_3".to_owned(),
            Origin::Opaque => "crate::native".to_owned(),
        }
    }

    /// Is this type always present (i.e it's part of the base Vulkan spec)
    pub fn always(&self) -> bool {
        matches!(self, Origin::Core | Origin::Vulkan1_0)
    }

    /// Gets the ID of an extension, panics otherwise.
    pub fn id(&self) -> i64 {
        match self {
            Self::Extension(_, id, _) => *id,
            Self::Core | Self::Vulkan1_0 | Self::Vulkan1_1 | Self::Vulkan1_2 | Self::Vulkan1_3 => 0,
            other => panic!("not an extension: {:?}", other),
        }
    }

    /// As a boolean check whether the origin is enabled or not
    pub fn as_bool_tokens(&self, var: &Ident) -> TokenStream {
        match self {
            Origin::Unknown | Origin::Opaque | Origin::Core | Origin::Vulkan1_0 => quote::quote! {
                true
            },
            Origin::Extension(ext, _, _) => {
                let ext_name = ext.trim_start_matches("VK_").to_case(Case::Snake);
                let check = Ident::new(&ext_name, Span::call_site());

                quote! {
                    #var.#check()
                }
            },
            Origin::Vulkan1_1 => quote::quote! {
                #var.version() >= Version::new(1, 1, 0)
            },
            Origin::Vulkan1_2 => quote::quote! {
                #var.version() >= Version::new(1, 2, 0)
            },
            Origin::Vulkan1_3 => quote::quote! {
                #var.version() >= Version::new(1, 3, 0)
            },
        }
    }

    /// As a result check whether the origin is enabled or not
    pub fn as_try_tokens(&self, var: &Ident) -> TokenStream {
        match self {
            Origin::Extension(ext, _, _) => {
                let ext_name = ext.trim_start_matches("VK_").to_case(Case::Snake);
                let check = Ident::new(&format!("is_{}", ext_name), Span::call_site());

                quote! {
                    #var.#check()
                }
            },
            Origin::Core | Origin::Vulkan1_0 | Origin::Unknown | Origin::Opaque => quote::quote! {
                Ok(())
            },
            Origin::Vulkan1_1 => quote::quote! {
                #var.min_version(Version::new(1, 1, 0))
            },
            Origin::Vulkan1_2 => quote::quote! {
                #var.min_version(Version::new(1, 2, 0))
            },
            Origin::Vulkan1_3 => quote::quote! {
                #var.min_version(Version::new(1, 3, 0))
            },
        }
    }

    /// Turns the origin into a tokenized rust path
    pub fn as_path(&self) -> syn::Path {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Core => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("core", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Extension(name, _, _) => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("extensions", Span::call_site())),
                    PathSegment::from(Ident::new(
                        &name.trim_start_matches("VK_").to_case(Case::Snake),
                        Span::call_site(),
                    )),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Vulkan1_0 => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("vulkan1_0", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Vulkan1_1 => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("vulkan1_1", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Vulkan1_2 => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("vulkan1_2", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Vulkan1_3 => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("vulkan1_3", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
            Origin::Opaque => syn::Path {
                leading_colon: None,
                segments: [
                    PathSegment::from(Ident::new("crate", Span::call_site())),
                    PathSegment::from(Ident::new("native", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
        }
    }

    /// As a file path of the output file for this origin
    pub fn as_file_path(&self, path: &Path) -> PathBuf {
        let mut path = PathBuf::from(path);

        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Core => path.push("core.rs"),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => path.push(format!(
                "extensions/{}.rs",
                ext.trim_start_matches("VK_").to_case(Case::Snake)
            )),
            Origin::Vulkan1_0 => path.push("vulkan1_0.rs"),
            Origin::Vulkan1_1 => path.push("vulkan1_1.rs"),
            Origin::Vulkan1_2 => path.push("vulkan1_2.rs"),
            Origin::Vulkan1_3 => path.push("vulkan1_3.rs"),
            Origin::Opaque => path.push("native.rs"),
        }

        path
    }

    /// Converts an origin into the same origin with a static lifetime
    pub fn as_static(&self) -> Origin<'static> {
        match self {
            Origin::Unknown => Origin::Unknown,
            Origin::Core => Origin::Core,
            Origin::Extension(name, id, disabled) => Origin::Extension(Cow::Owned(name.to_string()), *id, *disabled),
            Origin::Vulkan1_0 => Origin::Vulkan1_0,
            Origin::Vulkan1_1 => Origin::Vulkan1_1,
            Origin::Vulkan1_2 => Origin::Vulkan1_2,
            Origin::Vulkan1_3 => Origin::Vulkan1_3,
            Origin::Opaque => Origin::Opaque,
        }
    }

    /// Gets the name/module name of the origin
    pub fn as_name(&self) -> String {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Core => "core".to_string(),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => ext.trim_start_matches("VK_").to_case(Case::Snake),
            Origin::Vulkan1_0 => "vulkan1_0".to_string(),
            Origin::Vulkan1_1 => "vulkan1_1".to_string(),
            Origin::Vulkan1_2 => "vulkan1_2".to_string(),
            Origin::Vulkan1_3 => "vulkan1_3".to_string(),
            Origin::Opaque => "native".to_string(),
        }
    }

    /// Generate the feature gate (if any) for this origin
    pub fn condition(&self) -> Option<TokenStream> {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => Some(quote! {
                #[cfg(feature = #ext)]
            }),
            Origin::Core
            | Origin::Opaque
            | Origin::Vulkan1_0
            | Origin::Vulkan1_1
            | Origin::Vulkan1_2
            | Origin::Vulkan1_3 => None,
        }
    }

    /// Generate the feature gate (if any) for this origin
    pub fn feature_gate(&self) -> Option<String> {
        match self {
            Origin::Unknown => panic!("unknown origin cannot be turned into a module"),
            Origin::Extension(_, _, true) => panic!("cannot write files for disabled extensions"),
            Origin::Extension(ext, _, _) => Some(format!("#[cfg(feature = \"{}\")]\n", ext)),
            Origin::Core
            | Origin::Opaque
            | Origin::Vulkan1_0
            | Origin::Vulkan1_1
            | Origin::Vulkan1_2
            | Origin::Vulkan1_3 => None,
            /*Origin::Vulkan1_1 => None,
            Origin::Vulkan1_2 => Some("#[cfg(feature = \"VULKAN_1_2\")]\n".to_string()),
            Origin::Vulkan1_3 => Some("#[cfg(feature = \"VULKAN_1_3\")]\n".to_string()),
            Origin::Opaque => None,*/
        }
    }
}

impl<'a> ToTokens for Origin<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.as_path_tokens());
    }
}

impl<'a> Default for Origin<'a> {
    fn default() -> Self {
        Self::Unknown
    }
}

impl<'a> SymbolName<'a> for Origin<'a> {
    fn name(&self) -> Cow<'a, str> {
        match self {
            Origin::Unknown => panic!("Unknown origin cannot be turned into a name"),
            Origin::Core => Cow::Borrowed("CORE"),
            Origin::Extension(ext, _, _) => ext.clone(),
            Origin::Vulkan1_0 => Cow::Borrowed("VULKAN_1_0"),
            Origin::Vulkan1_1 => Cow::Borrowed("VULKAN_1_1"),
            Origin::Vulkan1_2 => Cow::Borrowed("VULKAN_1_2"),
            Origin::Vulkan1_3 => Cow::Borrowed("VULKAN_1_3"),
            Origin::Opaque => Cow::Borrowed("OPAQUE"),
        }
    }

    fn pretty_name(&self) -> String {
        self.name().to_string()
    }
}
