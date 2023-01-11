use std::{
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
    sync::Arc,
};

use proc_macro2::TokenStream;
use quote::ToTokens;
use tracing::debug;

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

use super::DocVisitor;

pub struct WasmBackendVisitor {
    doc: Arc<DocVisitor>,
    path_stub: PathBuf,
}

impl WasmBackendVisitor {
    pub fn new<P: AsRef<Path>>(doc: Arc<DocVisitor>, path_stub: P) -> Self {
        Self {
            doc,
            path_stub: path_stub.as_ref().to_owned(),
        }
    }
}

impl Visitor for WasmBackendVisitor {
    type OriginVisitor<'parent> = WasmBackendOriginVisitor<'parent>
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_origin<'a>(&mut self, _source: &Source<'a>, origin: &Origin<'a>) -> Self::OriginVisitor<'_> {
        WasmBackendOriginVisitor {
            path: origin.as_file_path(&self.path_stub),
            parent: self,
            origin: origin.as_static(),
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

pub struct WasmBackendOriginVisitor<'parent> {
    parent: &'parent mut WasmBackendVisitor,
    path: PathBuf,
    imports: Imports,
    out: TokenStream,
    origin: Origin<'static>,
}

impl<'parent> Deref for WasmBackendOriginVisitor<'parent> {
    type Target = WasmBackendVisitor;

    fn deref(&self) -> &Self::Target {
        &*self.parent
    }
}

impl<'parent> DerefMut for WasmBackendOriginVisitor<'parent> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'parent> Drop for WasmBackendOriginVisitor<'parent> {
    fn drop(&mut self) {
        let mut out = String::with_capacity(1 << 20);
        out.extend_one(self.imports.to_token_stream().to_string());
        out.extend_one(self.out.to_string());

        let out = run_rustfmt(out).expect("failed to run rustfmt");
        std::fs::write(&self.path, out).expect("Failed to write code");
    }
}

impl<'parent> OriginVisitor<'parent> for WasmBackendOriginVisitor<'parent> {
    fn visit_const<'a>(&mut self, source: &Source<'a>, const_: &Const<'a>) {
        let name = const_.as_ident();
        let doc = self.doc.find(const_.name()).map(|_| doc_path(const_.original_name()));

        let mut out = &mut self.out;
        quote::quote_each_token! {
            out

            #doc
            const #name: usize = 0;
        };
    }

    fn visit_const_alias<'a>(&mut self, source: &Source<'a>, const_alias: &ConstAlias<'a>) {}

    fn visit_opaque_type<'a>(&mut self, source: &Source<'a>, opaque_type: &OpaqueType<'a>) {}

    fn visit_type_alias<'a>(&mut self, source: &Source<'a>, alias: &Alias<'a>) {}

    fn visit_struct<'a>(&mut self, source: &Source<'a>, struct_: &Struct<'a>) {
        if struct_.has_opaque(source) {
            debug!(
                "WASM backend visitor ignored struct `{}` from `{}`",
                struct_.original_name(),
                struct_.origin().as_name()
            );
            return;
        }
    }

    fn visit_union<'a>(&mut self, source: &Source<'a>, union_: &Union<'a>) {
        if union_.has_opaque(source) {
            debug!(
                "WASM backend visitor ignored union `{}` from `{}`",
                union_.original_name(),
                union_.origin().as_name()
            );
            return;
        }
    }

    fn visit_handle<'a>(&mut self, source: &Source<'a>, handle: &Handle<'a>) {}

    fn visit_function_pointer<'a>(&mut self, source: &Source<'a>, function_pointer: &FunctionPointer<'a>) {
        if function_pointer.has_opaque(source) {
            debug!(
                "WASM backend visitor ignored function pointer `{}` from `{}`",
                function_pointer.original_name(),
                function_pointer.origin().as_name()
            );
            return;
        }
    }

    fn visit_base_type<'a>(&mut self, source: &Source<'a>, base_type: &Basetype<'a>) {}

    fn visit_bitmask<'a>(&mut self, source: &Source<'a>, bitmask: &Bitmask<'a>) {}

    fn visit_bitflag<'a>(&mut self, source: &Source<'a>, bitflag: &BitFlag<'a>) {}

    fn visit_enum<'a>(&mut self, source: &Source<'a>, enum_: &Enum<'a>) {}

    fn visit_command_alias<'a>(&mut self, source: &Source<'a>, command_alias: &CommandAlias<'a>) {}

    fn visit_function<'a>(&mut self, source: &Source<'a>, function: &Function<'a>) {}

    fn visit_command<'a>(&mut self, source: &Source<'a>, command: &Function<'a>) {}
}
