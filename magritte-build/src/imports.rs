//! Utilities to generate nicer imports.

use std::{collections::HashSet, fmt::Display, str::FromStr};

use magritte_types::Source;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::{Origin, origin::cond_of};

/// A set of imports
#[derive(Debug, Clone, Default)]
pub struct Imports(pub(crate) HashSet<String>, pub(crate) Origin<'static>);

impl Imports {
    /// Creates a new imports with an origin
    pub fn new(origin: &Origin<'_>) -> Self {
        Self(HashSet::new(), origin.to_static())
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
    pub fn push_origin<D: Display>(&mut self, source: &Source<'_>, origin: &Origin<'_>, import: D) {
        if origin != &self.1 {
            let cond = cond_of(source, &self.1, origin).map(|tt| tt.to_string()).unwrap_or_else(String::new);
            self.0
                .insert(format!("{cond} use {}::{};", origin.as_rust_path("crate"), import));
        }
    }
}

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for elem in &self.0 {
            tokens.extend(TokenStream::from_str(elem).unwrap());
        }
    }
}
