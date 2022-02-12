//! # Code generation
//! This module handles code generation for all types.

mod basetypes;
mod constants;
mod enums;
mod expr;
mod extensions;
pub mod ty;

use ahash::AHashMap;
use proc_macro2::TokenStream;

use crate::{doc::Documentation, imports::Imports, origin::Origin, source::Source};

#[doc(hidden)]
pub struct CodeOut(pub Origin<'static>, pub Imports, pub TokenStream, pub TokenStream);

unsafe impl Send for CodeOut {}

impl<'a> Source<'a> {
    /// Generates the code in a per-origin basis.
    pub fn generate_code(&self, doc: &mut Documentation) -> Vec<CodeOut> {
        let mut per_origin = self
            .origins
            .iter()
            .filter(|o| !o.is_disabled())
            .map(|o| (o, (Imports::new(o), TokenStream::new(), TokenStream::new())))
            .collect::<AHashMap<_, _>>();

        for const_ in &self.constants {
            if const_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(const_.origin()).unwrap();

            const_.generate_code(self, doc, imports, out);
        }

        for const_alias in &self.constant_aliases {
            if const_alias.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(const_alias.origin()).unwrap();

            const_alias.generate_code(self, doc, imports, out);
        }

        for basetype in &self.basetypes {
            if basetype.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(basetype.origin()).unwrap();

            basetype.generate_code(self, doc, imports, out);
        }

        for enum_ in &self.enums {
            if enum_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(enum_.origin()).unwrap();

            enum_.generate_code(self, doc, imports, out);
        }

        for ext in &self.extensions {
            if ext.origin().is_disabled() {
                continue;
            }

            let (_, header, _) = per_origin.get_mut(ext.origin()).unwrap();

            ext.generate_mod_doc(self, doc, header);
        }

        per_origin
            .into_iter()
            .map(|(key, (i, h, t))| CodeOut(key.as_static(), i, h, t))
            .collect()
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
