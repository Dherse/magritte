//! Utilities to generate nicer imports.

use std::{cell::RefCell, fmt::Display};

use ahash::AHashSet;
use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::origin::Origin;

/// A set of imports
#[derive(Debug, Clone, Default)]
pub struct Imports(pub(crate) RefCell<AHashSet<String>>, pub(crate) Origin<'static>);

impl Imports {
    /// Creates a new imports with an origin
    pub fn new(origin: &Origin<'_>) -> Self {
        Self(RefCell::new(AHashSet::new()), origin.as_static())
    }

    /// Push a new imports from its name
    pub fn push<D: Display>(&self, import: D) {
        self.0.borrow_mut().insert(format!("use {};", import));
    }

    /// Pust a string to the list of imports
    pub fn push_str(&self, import: &str) {
        self.0.borrow_mut().insert(import.to_string());
    }

    /// Adds conditional imports to serde
    pub fn smallvec(&self) {
        self.0.borrow_mut().insert("use crate::SmallVec;".to_string());
    }

    /// Adds conditional imports to serde
    pub fn serde(&self) {
        self.0.borrow_mut().insert(
            r#"
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};
        "#
            .to_string(),
        );
    }

    /// Push an imports form an origin and a name
    pub fn push_origin<D: Display>(&self, origin: &Origin<'_>, import: D) {
        if origin != &self.1 {
            self.0
                .borrow_mut()
                .insert(format!("use {}::{};", origin.as_path_str(), import));
        }
    }
}

impl ToTokens for Imports {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for elem in &*self.0.borrow() {
            tokens.extend(syn::parse_str::<TokenStream>(elem));
        }
    }
}
