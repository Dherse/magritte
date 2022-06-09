use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::TokenStream;
use quote::{format_ident, quote_each_token};

use crate::{
    imports::Imports,
    source::{Handle, Source},
};

impl<'a> Handle<'a> {
    /// Generates the code required to create a single enum containing all of the handles as uniques
    pub fn generate_handle_enum_code<'b>(
        handles: &[Handle<'a>],
        source: &Source<'a>,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        let handles = handles
            .into_iter()
            .filter(|h| !h.origin().is_disabled())
            .collect::<Vec<_>>();

        let handle_idents = handles.iter().cloned().map(Handle::as_ident).collect::<Vec<_>>();
        let is_idents = handles.iter().map(|s| format_ident!("is_{}", s.name().to_snake_case()));
        let try_idents = handles
            .iter()
            .map(|s| format_ident!("try_as_{}", s.name().to_snake_case()))
            .collect::<Vec<_>>();
        let as_idents = handles.iter().map(|s| format_ident!("as_{}", s.name().to_snake_case()));
        let as_unchecked_ident = handles
            .iter()
            .map(|s| format_ident!("as_{}_unchecked", s.name().to_snake_case()));

        let is_docs = handles
            .iter()
            .map(|h| format!("Checks if the handle is a `{}`", h.name()));
        let as_docs = handles
            .iter()
            .map(|h| format!("Gets the handle as a `{}`, if it is not then panic", h.name()));
        let try_docs = handles.iter().map(|h| {
            format!(
                "Gets the handle as a `{}`, if it is not then returns [`None`]",
                h.name()
            )
        });
        let as_unchecked_docs = handles
            .iter()
            .map(|h| format!("Gets the handle as a `{}`, if it is not then can cause UB", h.name()));

        let as_str = handles.iter().map(|h| format!("Handle is not a `{}`", h.name()));

        let conds = handles
            .iter()
            .map(|h| h.origin())
            .map(|o| o.condition(source))
            .collect::<Vec<_>>();

        let enum_ = source
            .enums
            .get_by_name("VkDebugReportObjectTypeEXT")
            .expect("VkDebugReportObjectTypeEXT missing");

        let variants = handles.iter().map(|h| {
            let shouty_name = if h.rename.is_some() {
                source
                    .handles
                    .get_by_name(&h.original_name)
                    .expect("unknown alias")
                    .name()
                    .to_shouty_snake_case()
            } else {
                h.name().to_shouty_snake_case()
            };

            if let Some(variant) = enum_
                .variants()
                .get_by_name(&format!("VK_DEBUG_REPORT_OBJECT_TYPE_{}_EXT", shouty_name))
            {
                variant.as_ident()
            } else if let Some(variant) = enum_
                .aliases()
                .get_by_name(&format!("VK_DEBUG_REPORT_OBJECT_TYPE_{}_EXT", shouty_name))
            {
                variant.as_ident()
            } else {
                format_ident!("UNKNOWN")
            }
        });

        for handle in &handles {
            if let Some(cond) = handle.origin().feature_gate(source) {
                imports.push_str(&format!(
                    "{} use {}::{};",
                    cond,
                    handle.origin().as_path_str(),
                    handle.name()
                ));
            } else {
                imports.push_origin(handle.origin(), handle.name());
            }
        }

        imports.push_str("#[cfg(feature = \"VK_EXT_debug_marker\")] use crate::generated::extensions::ext_debug_marker::DebugReportObjectTypeEXT;");

        quote_each_token! {
            out

            #[derive(Clone)]
            pub enum Handles {
                #(
                    #conds
                    #handle_idents(Unique<#handle_idents>)
                ),*
            }

            #[cfg(feature = "VK_EXT_debug_marker")]
            impl Handles {
                pub fn as_debug_report_object_type_ext(&self) -> DebugReportObjectTypeEXT {
                    match self {
                        #(
                            #conds
                            Self::#handle_idents(_) => DebugReportObjectTypeEXT::#variants,
                        )*
                    }
                }
            }

            #(
                #conds
                impl Handles {
                    #[doc = #is_docs]
                    #[inline]
                    pub fn #is_idents (&self) -> bool {
                        matches!(self, Self :: #handle_idents (_))
                    }

                    #[doc = #as_docs]
                    #[track_caller]
                    #[inline]
                    pub fn #as_idents (self) -> Unique<#handle_idents> {
                        self . #try_idents () . expect(#as_str)
                    }

                    #[doc = #try_docs]
                    #[inline]
                    pub fn #try_idents (self) -> Option<Unique<#handle_idents>> {
                        match self {
                            Self :: #handle_idents (value) => Some(value),
                            _ => None
                        }
                    }

                    #[doc = #as_unchecked_docs]
                    #[inline]
                    pub unsafe fn #as_unchecked_ident (self) -> Unique<#handle_idents> {
                        match self {
                            Self :: #handle_idents (value) => value,
                            _ => std::hint::unreachable_unchecked()
                        }
                    }
                }

                #conds
                impl From<Unique<#handle_idents>> for Handles {
                    fn from(value: Unique<#handle_idents>) -> Self {
                        Self :: #handle_idents (value)
                    }
                }
            )*
        }
    }
}
