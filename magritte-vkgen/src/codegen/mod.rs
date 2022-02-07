//! # Code generation
//! This module handles code generation for all types.

mod constants;

use ahash::AHashMap;
use proc_macro2::TokenStream;

use crate::{doc::Documentation, imports::Imports, source::Source};

impl<'a> Source<'a> {
    /// Generates the code in a per-origin basis.
    pub fn generate_code(&self, doc: &Documentation) {
        let mut per_origin = self
            .origins
            .iter()
            .map(|o| (o.clone(), TokenStream::new()))
            .collect::<AHashMap<_, _>>();

        for const_ in &self.constants {
            let mut imports = Imports::new(const_.origin());

            let out = per_origin.get_mut(const_.origin()).unwrap();

            const_.generate_code(self, doc, &mut imports, out);
        }
    }
}
