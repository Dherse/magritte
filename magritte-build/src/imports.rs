//! Utilities to generate nicer imports.

use std::{collections::HashSet, fmt::Display, str::FromStr};

use magritte_types::{Source, TypeRef};
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{origin::cond_of, Origin};

/// A set of imports
#[derive(Debug, Clone, Default)]
pub struct Imports(
    pub(crate) HashSet<String>,
    pub(crate) Origin<'static>,
    pub(crate) String,
);

impl Imports {
    /// Creates a new imports with an origin
    pub fn new<S: AsRef<str>>(origin: &Origin<'_>, prefix: S) -> Self {
        Self(HashSet::new(), origin.to_static(), prefix.as_ref().to_string())
    }

    /// Push a new imports from its name
    pub fn push<D: Display>(&mut self, import: D) {
        self.0.insert(format!("use {};", import));
    }

    /// Pust a string to the list of imports
    pub fn push_str<S: Into<String>>(&mut self, import: S) {
        self.0.insert(import.into());
    }

    /// Push an imports form an origin and a name
    pub fn push_origin<D: Display>(&mut self, source: &Source<'_>, origin: &Origin<'_>, import: D) {
        if origin != &self.1 {
            let cond = cond_of(source, &self.1, origin)
                .map(|tt| tt.to_string())
                .unwrap_or_else(String::new);
            self.0
                .insert(format!("{cond} use {}::{};", origin.as_rust_path(&self.2), import));
        }
    }

    pub fn serde(&mut self) {
        self.push_str("#[cfg(feature = \"serde\")]\nuse serde::{Deserialize, Serialize};");
    }

    pub fn import(&mut self, source: &Source<'_>, ty: TypeRef<'_, '_>) {
        self.push_origin(source, ty.origin(), ty.name());
    }
}

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for elem in &self.0 {
            tokens.extend(TokenStream::from_str(elem).unwrap());
        }
    }
}
