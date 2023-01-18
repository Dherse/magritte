use std::path::Path;

use magritte_types::{Source, Extension};
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Visitor, rustfmt::run_rustfmt};

#[derive(Default)]
pub struct ExtensionsVisitor {
    extensions: TokenStream,
}

impl ExtensionsVisitor {
    pub fn write<P: AsRef<Path>>(self, path: P) {
        let path = path.as_ref().join("extensions.rs");
        let out = run_rustfmt(self.extensions.to_string()).expect("failed to run rustfmt");

        std::fs::write(path, out.as_bytes()).expect("failed to write extensions.toml");
    }
}

impl Visitor for ExtensionsVisitor {
    type OriginVisitor<'parent> = ()
    where
        Self: 'parent;

    type VersionVisitor<'parent> = ()
    where
        Self: 'parent;

    type ExtensionVisitor<'parent> = ()
    where
        Self: 'parent;

    fn visit_extension<'a>(
        &mut self,
        source: &Source<'a>,
        extension: &Extension<'a>,
    ) -> Option<Self::ExtensionVisitor<'_>> {
        if extension.disabled() {
            return None;
        }

        let cond = extension.origin().feature_flag(source).expect("extensions always have feature flags");
        let ident = extension.as_ident();
        let path = format!("./extensions/{}.rs", ident);
        self.extensions.extend(quote! {
            #[cfg(any(#(feature = #cond),*))]
            #[path = #path]
            pub mod #ident;
        });

        None
    }
}