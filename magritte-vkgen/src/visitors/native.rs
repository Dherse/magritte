mod const_ty;
mod struct_base;

use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Arc,
};

use proc_macro2::TokenStream;
use quote::{quote, quote_each_token, ToTokens};
use tracing::warn;

use crate::{
    codegen::{OriginVisitor, Visitor},
    imports::Imports,
    origin::Origin,
    rustmft::run_rustfmt,
    source::{
        Alias, Basetype, BitFlag, Bitmask, CommandAlias, Const, ConstAlias, Enum, Extension, Function, FunctionPointer,
        Handle, OpaqueType, Source, Struct, Union,
    },
    visitors::doc_path,
};

pub use const_ty::{constant_type, constant_value};

use super::DocVisitor;

pub struct NativeVisitor {
    doc: Arc<DocVisitor>,
    path_stub: PathBuf,
}

impl NativeVisitor {
    pub fn new<P: AsRef<Path>>(doc: Arc<DocVisitor>, path_stub: P) -> Self {
        Self {
            doc,
            path_stub: path_stub.as_ref().to_owned(),
        }
    }
}

impl Visitor for NativeVisitor {
    type OriginVisitor<'parent> = NativeOriginVisitor<'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, origin: &Origin<'a>) -> Self::OriginVisitor<'_> {
        NativeOriginVisitor {
            path: origin.as_file_path(&self.path_stub),
            parent: self,
            imports: Imports::new(origin),
            out: TokenStream::new(),
        }
    }

    fn visit_version<'a>(&mut self, _source: &Source<'a>, _origin: &Origin<'a>) -> Self::VersionVisitor<'_> {
        ()
    }

    fn visit_extension<'a>(&mut self, _source: &Source<'a>, _extension: &Extension<'a>) -> Self::ExtensionVisitor<'_> {
        ()
    }
}

pub struct NativeOriginVisitor<'parent> {
    parent: &'parent mut NativeVisitor,
    path: PathBuf,
    imports: Imports,
    out: TokenStream,
}

impl<'parent> NativeOriginVisitor<'parent> {
    pub fn write(self) {
        let mut out = String::with_capacity(1 << 20);
        out.extend_one(self.imports.to_token_stream().to_string());
        out.extend_one(self.out.to_string());

        let out = run_rustfmt(out).expect("failed to run rustfmt");
        std::fs::write(&self.path, out).expect("Failed to write code");
    }
}

impl<'parent> Deref for NativeOriginVisitor<'parent> {
    type Target = NativeVisitor;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'parent> DerefMut for NativeOriginVisitor<'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'parent> OriginVisitor<'parent> for NativeOriginVisitor<'parent> {
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {
        let name = const_.as_ident();
        let ty = constant_type(const_.ty(), &self.imports);
        let value = constant_value(const_.value(), source, &self.imports);

        let doc = self.doc.find(const_.name()).map(|_| doc_path(const_.original_name()));

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #doc
            pub const #name: #ty = #value;
        };
    }

    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {
        let name = const_alias.as_ident();

        let of = source.resolve(const_alias.of()).expect("unknown alias");
        self.imports.push_origin(of.origin(), of.name());

        let value = of.as_path();
        let ty = constant_type(of.as_const().expect("not a constant").ty(), &self.imports);

        let doc_str = format!("See [`{}`].", of.as_string_path());

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #[doc = #doc_str]
            pub const #name: #ty = #value;
        }
    }

    fn visit_opaque_type<'a>(&mut self, _source: &Source<'a>, opaque_type: &OpaqueType<'a>) {
        let name = opaque_type.as_ident();

        if opaque_type.requires().starts_with("vk_video/") {
            warn!("Ignoring VK_VIDEO opaque type: {}", opaque_type.original_name());
            return;
        }

        let ty = match opaque_type.original_name() {
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
            other => unreachable!("unknown opaque type: {}", other),
        };

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #[doc = "An opaque type, usually needing to be defined by an external library"]
            pub type #name = #ty;
        }
    }

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {
        let of = source.resolve_type(alias.of()).expect("unknown alias");
        let doc_str = format!("See [`{}::{}`]", of.origin().as_path_str(), of.name());

        let name = alias.as_ident();
        let ty = of.as_ident();

        self.imports.push_origin(of.origin(), of.name());

        let mut out = &mut self.out;
        quote_each_token! {
            out

            #[doc = #doc_str]
            pub type #name = #ty;
        }
    }

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        struct_base::base_struct(&self.parent.doc, source, struct_, &mut self.out, &self.imports);
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {}

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {}

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {}

    fn visit_base_type<'a>(&mut self, source: &Source<'a>, base_type: &Basetype<'a>) {}

    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {}

    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &BitFlag<'a>) {}

    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {}

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {}

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {}

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {}

    fn finish(self) {
        self.write();
    }
}
