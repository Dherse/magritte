//! # Code generation
//! This module handles code generation for all types.

mod constants;
mod expr;
mod ty;

use ahash::AHashMap;
use proc_macro2::TokenStream;

use crate::{doc::Documentation, imports::Imports, origin::Origin, source::Source};

impl<'a> Source<'a> {
    /// Generates the code in a per-origin basis.
    pub fn generate_code(&self, doc: &Documentation) -> AHashMap<&'_ Origin<'a>, TokenStream> {
        let mut per_origin = self
            .origins
            .iter()
            .map(|o| (o, TokenStream::new()))
            .collect::<AHashMap<_, _>>();

        for const_ in &self.constants {
            let imports = Imports::new(const_.origin());

            let out = per_origin.get_mut(const_.origin()).unwrap();

            const_.generate_code(self, doc, &imports, out);
        }

        per_origin.into_iter().filter(|(o, _)| !o.is_disabled()).collect()
    }
}
