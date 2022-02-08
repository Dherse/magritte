//! # Code generation
//! This module handles code generation for all types.

mod constants;
mod expr;
mod ty;
mod basetypes;

use ahash::AHashMap;
use proc_macro2::TokenStream;

use crate::{doc::Documentation, imports::Imports, origin::Origin, source::Source};

impl<'a> Source<'a> {
    /// Generates the code in a per-origin basis.
    pub fn generate_code(&self, doc: &Documentation) -> AHashMap<&'_ Origin<'a>, TokenStream> {
        let mut per_origin = self
            .origins
            .iter()
            .map(|o| (o, (Imports::new(o), TokenStream::new())))
            .collect::<AHashMap<_, _>>();

        for const_ in &self.constants {
            if const_.origin().is_disabled() {
                continue;
            }
            
            let (imports, out) = per_origin.get_mut(const_.origin()).unwrap();

            const_.generate_code(self, doc, &imports, out);
        }

        for const_alias in &self.constant_aliases {
            if const_alias.origin().is_disabled() {
                continue;
            }

            let (imports, out) = per_origin.get_mut(const_alias.origin()).unwrap();

            const_alias.generate_code(self, doc, &imports, out);
        }

        per_origin.into_iter().filter(|(o, _)| !o.is_disabled()).map(|(o, (_, out))| (o, out)).collect()
    }
}

fn alias_of(original: &str, name: &str, mut out: &mut TokenStream) {
    if original != name {
        quote::quote_each_token! {
            out

            #[doc(alias = #original)]
        }
    }
}