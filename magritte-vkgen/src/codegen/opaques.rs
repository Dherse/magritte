use proc_macro2::TokenStream;
use quote::{quote, quote_each_token};
use tracing::warn;

use crate::{
    doc::{Documentation, Queryable},
    source::{OpaqueType, Source},
};

impl<'a> OpaqueType<'a> {
    /// Generates the code for an opaque type
    pub(super) fn generate_code(&self, source: &Source<'a>, doc: &mut Documentation, mut out: &mut TokenStream) {
        // the name as an identifier
        let name = self.as_ident();

        // append the doc first
        self.generate_doc(source, doc, out);

        if self.requires().starts_with("vk_video/vulkan_video_codec_") {
            quote_each_token! {
                out

                pub type #name = crate::video::#name;
            }
        } else {
            let ty = match self.original_name() {
                "Display" => quote! { std::ffi::c_void },
                "VisualID" => quote! { std::os::raw::c_ulong },
                "Window" => quote! {  std::os::raw::c_ulong },
                "RROutput" => quote! { std::os::raw::c_ulong },
                "wl_display" => quote! { std::ffi::c_void },
                "wl_surface" => quote! { std::ffi::c_void },
                "HINSTANCE" => quote! { isize },
                "HWND" => quote! { isize },
                "HMONITOR" => quote! { isize },
                "HANDLE" => quote! { isize },
                "SECURITY_ATTRIBUTES" => quote! { std::ffi::c_void },
                "DWORD" => quote! { u32 },
                "LPCWSTR" => quote! { *const u16 },
                "xcb_connection_t" => quote! { std::ffi::c_void },
                "xcb_visualid_t" => quote! { u32 },
                "xcb_window_t" => quote! { u32 },
                "IDirectFB" => quote! { std::ffi::c_void },
                "IDirectFBSurface" => quote! { std::ffi::c_void },
                "zx_handle_t" => quote! { u32 },
                "GgpStreamDescriptor" => quote! { u32 },
                "GgpFrameToken" => quote! { u64 },
                "_screen_context" => quote! { std::ffi::c_void },
                "_screen_window" => quote! { std::ffi::c_void },
                _ => quote! { c_void },
            };

            quote_each_token! {
                out

                pub type #name = #ty;
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
