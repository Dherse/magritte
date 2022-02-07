//! Utilities to generate nicer imports.

use std::fmt::Display;

use ahash::AHashSet;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::origin::Origin;

/// A set of imports
#[derive(Debug, Clone, Default)]
pub struct Imports(pub(crate) AHashSet<String>, pub(crate) Origin<'static>);

impl Imports {
    /// Creates a new imports with an origin
    pub fn new(origin: &Origin<'_>) -> Self {
        Self(AHashSet::new(), origin.as_static())
    }

    /// Push a new imports from its name
    pub fn push<D: Display>(&mut self, import: D) {
        self.0.insert(format!("use {};", import));
    }

    /// Pust a string to the list of imports
    pub fn push_str(&mut self, import: &str) {
        self.0.insert(import.to_string());
    }

    /// Push an imports form an origin and a name
    pub fn push_origin<D: Display>(&mut self, origin: &Origin<'_>, import: D) {
        if origin != &self.1 {
            self.0.insert(format!("use {}::{};", origin.as_path_str(), import));
        }
    }
}

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for elem in &self.0 {
            tokens.extend(syn::parse_str::<TokenStream>(elem));
        }
    }
}
