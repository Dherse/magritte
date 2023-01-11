use std::{
    iter::once,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::{
    codegen::{OriginVisitor, Visitor},
    origin::Origin,
    rustmft::run_rustfmt,
    source::{Extension, Handle, Source},
};

pub struct HandleVisitor {
    path_stub: PathBuf,
    handles: Vec<BuiltHandle>,
}

struct BuiltHandle {
    ident: Ident,
    path: TokenStream,
    is_ident: Ident,
    as_ident: Ident,
    try_as_ident: Ident,
    unchecked_as_ident: Ident,
    cond: Option<TokenStream>,
    object_type: Ident,
    dbg_object_type: Ident,

    doc: String,
    is_doc: String,
    as_doc: String,
    try_doc: String,
    unchecked_doc: String,
}

impl HandleVisitor {
    #[inline]
    pub fn new<P: AsRef<Path>>(path_stub: P) -> Self {
        Self {
            path_stub: path_stub.as_ref().to_owned(),
            handles: Vec::with_capacity(300),
        }
    }

    pub fn write(self) {
        let mut out = String::with_capacity(1 << 20);

        let handle_idents = self.handles.iter().map(|h| &h.ident).collect::<Vec<_>>();
        let handle_origins = self.handles.iter().map(|h| &h.path);
        let is_idents = self.handles.iter().map(|h| &h.is_ident);
        let as_idents = self.handles.iter().map(|h| &h.as_ident);
        let try_as_idents = self.handles.iter().map(|h| &h.try_as_ident);
        let unchecked_as_idents = self.handles.iter().map(|h| &h.unchecked_as_ident);
        let handle_conds = self.handles.iter().map(|h| &h.cond).collect::<Vec<_>>();
        let handle_object_types = self.handles.iter().map(|h| &h.object_type);
        let handle_dbg_object_types = self.handles.iter().map(|h| &h.dbg_object_type);

        let handle_docs = self.handles.iter().map(|h| &h.doc);
        let handle_is_docs = self.handles.iter().map(|h| &h.is_doc);
        let handle_as_docs = self.handles.iter().map(|h| &h.as_doc);
        let handle_try_docs = self.handles.iter().map(|h| &h.try_doc);
        let handle_unchecked_docs = self.handles.iter().map(|h| &h.unchecked_doc);

        let code = quote! {
            use crate::vulkan1_0::ObjectType;

            #[cfg(feature = "VK_EXT_debug_marker")]
            use crate::extensions::ext_debug_marker::DebugReportObjectTypeEXT;

            #(
                #handle_conds
                use #handle_origins :: #handle_idents;
            )*

            #[doc = "An enum containing all of the possible Vulkan handles."]
            #[doc = "Feature gated for convenience."]
            #[derive(Clone, Debug, PartialEq, Eq, Hash)]
            pub enum Handle {
                #(
                    #handle_conds
                    #[doc = #handle_docs]
                    #handle_idents (#handle_idents)
                ),*
            }

            impl Handle {
                #[doc = "Returns the raw value of the handle"]
                pub fn as_raw(&self) -> u64 {
                    match self {
                        #(
                            #handle_conds
                            Self::#handle_idents (handle) => handle.raw() as u64
                        ),*
                    }
                }

                #[doc = "Gets the [`ObjectType`] from the handle"]
                pub fn as_object_type(&self) -> ObjectType {
                    match self {
                        #(
                            #handle_conds
                            Self::#handle_idents (_) => ObjectType::#handle_object_types,
                        )*
                    }
                }

                #[cfg(feature = "VK_EXT_debug_marker")]
                #[doc = "Gets the [`DebugReportObjectTypeEXT`] from the handle"]
                pub fn as_debug_report_object_type(&self) -> DebugReportObjectTypeEXT {
                    match self {
                        #(
                            #handle_conds
                            Self::#handle_idents(_) => DebugReportObjectTypeEXT::#handle_dbg_object_types,
                        )*
                    }
                }

            }

            #(
                #handle_conds
                impl Handle {
                    #[doc = #handle_is_docs]
                    #[inline]
                    pub fn #is_idents(&self) -> bool {
                        matches!(self, Self:: #handle_idents (_))
                    }

                    #[doc = #handle_as_docs]
                    #[inline]
                    pub fn #as_idents(self) -> #handle_idents {
                        self. #try_as_idents () .expect(concat!("handle is not a `", stringify!(#handle_idents), "`"))
                    }

                    #[doc = #handle_try_docs]
                    #[inline]
                    pub const fn #try_as_idents(self) -> Option<#handle_idents> {
                        match self {
                            Self :: #handle_idents (value) => Some(value),
                            _ => None,
                        }
                    }

                    #[doc = #handle_unchecked_docs]
                    #[inline]
                    pub const unsafe fn #unchecked_as_idents(self) -> #handle_idents {
                        match self {
                            Self :: #handle_idents (value) => value,
                            _ => std::hint::unreachable_unchecked(),
                        }
                    }
                }

                #handle_conds
                impl From<#handle_idents> for Handle {
                    fn from(value: #handle_idents) -> Self {
                        Self :: #handle_idents (value)
                    }
                }

                #handle_conds
                impl Into<#handle_idents> for Handle {
                    fn into(self) -> #handle_idents {
                        match self {
                            Self :: #handle_idents (value) => value,
                            _ => unreachable!(concat!("handle is not a `", stringify!(#handle_idents), "`")),
                        }
                    }
                }
            )*
        };

        out.extend(once(code.to_string()));

        let path = self.path_stub.join("handles.rs");
        std::fs::write(path, run_rustfmt(out).expect("failed to run rustfmt")).expect("failed to write handles files");
    }
}

impl Visitor for HandleVisitor {
    type OriginVisitor<'parent> = HandleOriginVisitor<'parent>;

    type VersionVisitor<'parent> = ();

    type ExtensionVisitor<'parent> = ();

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, origin: &Origin<'a>) -> Self::OriginVisitor<'_> {
        HandleOriginVisitor(self, origin.as_static())
    }

    fn visit_version<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Self::VersionVisitor<'_> {
        ()
    }

    fn visit_extension<'a>(&mut self, _source: &Source<'a>, _extension: &Extension<'a>) -> Self::ExtensionVisitor<'_> {
        ()
    }
}

pub struct HandleOriginVisitor<'parent>(&'parent mut HandleVisitor, Origin<'static>);

impl<'parent> Deref for HandleOriginVisitor<'parent> {
    type Target = HandleVisitor;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'parent> DerefMut for HandleOriginVisitor<'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl<'parent> OriginVisitor<'parent> for HandleOriginVisitor<'parent> {
    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {
        let object_type_enum = source.enums.get_by_name("VkObjectType").expect("VkObjectType missing");
        let dbg_type_enum = source
            .enums
            .get_by_name("VkDebugReportObjectTypeEXT")
            .expect("VkDebugReportObjectTypeEXT missing");

        let snake = handle.name().to_snake_case();

        let shouty = if handle.rename().is_some() {
            source
                .handles
                .get_by_name(handle.original_name())
                .expect("unknwon alias")
                .name()
                .to_shouty_snake_case()
        } else {
            handle.name().to_shouty_snake_case()
        };

        let object_name = format!("VK_OBJECT_TYPE_{}", shouty);
        let object_type = if let Some(variant) = object_type_enum.variants().get_by_name(&object_name) {
            variant.as_ident()
        } else if let Some(variant) = object_type_enum.aliases().get_by_name(&object_name) {
            variant.as_ident()
        } else {
            format_ident!("UNKNOWN")
        };

        let dbg_type_name = format!("VK_DEBUG_REPORT_OBJECT_TYPE_{}_EXT", shouty);
        let dbg_object_type = if let Some(variant) = dbg_type_enum.variants().get_by_name(&dbg_type_name) {
            variant.as_ident()
        } else if let Some(variant) = dbg_type_enum.aliases().get_by_name(&dbg_type_name) {
            variant.as_ident()
        } else {
            format_ident!("UNKNOWN")
        };

        let path = handle.origin().as_path();

        self.handles.push(BuiltHandle {
            ident: handle.as_ident(),
            path,
            is_ident: format_ident!("is_{snake}"),
            as_ident: format_ident!("as_{snake}"),
            try_as_ident: format_ident!("try_as_{snake}"),
            unchecked_as_ident: format_ident!("as_{snake}_unchecked"),
            cond: handle.origin().condition(source),
            object_type,
            dbg_object_type,
            doc: format!("Container for a [`{}`].", handle.name()),
            is_doc: format!("Checks if the container contains a [`{}`].", handle.name()),
            as_doc: format!(
                r#"
Unwraps the container into [`{}`].

# Panics
Panics if the container does not contain a [`{0}`].
            "#,
                handle.name()
            ),
            try_doc: format!(
                "Tries to unwrap the container into [`{}`], returning `None` if it is not.",
                handle.name()
            ),
            unchecked_doc: format!(
                r#"
Unwraps the container into [`{}`] without checking if it is correct.

# Safety
This is extremely unsafe and you should verify that the container does contain a [`{0}`].
"#,
                handle.name()
            ),
        })
    }
}
