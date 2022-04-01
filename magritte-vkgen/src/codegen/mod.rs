//! # Code generation
//! This module handles code generation for all types.

mod basetypes;
mod bitflags;
mod bitmasks;
mod constants;
mod enums;
mod expr;
mod extensions;
mod funcpointers;
mod handles;
mod opaques;
mod structs;
pub mod ty;
mod unions;

use std::{collections::BTreeMap, fmt::Write};

use ahash::AHashMap;
use proc_macro2::TokenStream;
use quote::quote_each_token;

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

        {
            let (_, out, _) = per_origin.get_mut(&Origin::Opaque).unwrap();

            {
                let mut out = out;
                quote_each_token! {
                    out

                    #![allow(non_camel_case_types)]
                }
            }
        }

        for extension in &self.extensions {
            if extension.origin().is_disabled() {
                continue;
            }

            let (_, out, _) = per_origin.get_mut(extension.origin()).unwrap();

            extension.generate_mod_doc(self, doc, out);
        }

        for const_ in &self.constants {
            if const_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(const_.origin()).unwrap();

            const_.generate_code(self, doc, imports, out);
        }

        for basetype in &self.basetypes {
            if basetype.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(basetype.origin()).unwrap();

            basetype.generate_code(self, doc, imports, out);
        }

        for funcpointer in &self.funcpointers {
            if funcpointer.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(funcpointer.origin()).unwrap();

            funcpointer.generate_code(self, doc, imports, out);
        }

        for enum_ in &self.enums {
            if enum_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(enum_.origin()).unwrap();

            enum_.generate_code(self, doc, imports, out);
        }

        for bitflag in &self.bitflags {
            if bitflag.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(bitflag.origin()).unwrap();

            bitflag.generate_code(self, doc, imports, out);
        }

        for bitmask in &self.bitmasks {
            if bitmask.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(bitmask.origin()).unwrap();

            bitmask.generate_code(self, doc, imports, out);
        }

        for struct_ in &self.structs {
            if struct_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(struct_.origin()).unwrap();

            struct_.generate_raw_code(self, doc, imports, out);
        }

        for union_ in &self.unions {
            if union_.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(union_.origin()).unwrap();

            union_.generate_code(self, doc, imports, out);
        }

        for handle in &self.handles {
            if handle.origin().is_disabled() {
                continue;
            }

            let (_, _, out) = per_origin.get_mut(handle.origin()).unwrap();

            handle.generate_code(self, doc, out);
        }

        for opaque in &self.opaque_types {
            if opaque.origin().is_disabled() {
                continue;
            }

            let (imports, _, out) = per_origin.get_mut(opaque.origin()).unwrap();

            opaque.generate_code(self, doc, imports, out);
        }

        per_origin
            .into_iter()
            .map(|(key, (i, h, t))| CodeOut(key.as_static(), i, h, t))
            .collect()
    }

    /// Generate the global module file
    pub fn generate_mod(&self) -> String {
        let out = format!(
            r##"
            #![doc = "# Generated code"]
            #![doc = "This module contains all of the generated code for Vulkan gated by relevant feature gates."]

            pub mod extensions;
        "##
        );

        self.origins
            .iter()
            .filter(|origin| !origin.is_extension())
            .fold(out, |mut out, origin| {
                writeln!(
                    out,
                    "{}pub mod {};",
                    origin.feature_gate().unwrap_or_default(),
                    origin.as_name()
                )
                .unwrap();

                out
            })
    }

    /// Generate the extension module file
    pub fn generate_extension_mod(&self) -> String {
        let out = format!(
            r##"
            #![doc = "# Extensions"]
            #![doc = "This module contains all of the registered extensions gated by relevant feature gates."]
        
        "##
        );

        self.origins
            .iter()
            .filter(|origin| origin.is_extension() && !origin.is_disabled())
            .fold(out, |mut out, origin| {
                writeln!(
                    out,
                    "{}pub mod {};",
                    origin.feature_gate().unwrap_or_default(),
                    origin.as_name()
                )
                .unwrap();

                out
            })
    }

    /// Generate the feature set for a `Cargo.toml` file
    pub fn generate_feature_set(&self, out: &mut BTreeMap<String, Vec<String>>) {
        let mut all = Vec::new();
        for extension in &self.extensions {
            if extension.disabled() {
                continue;
            }

            all.push(extension.name().to_string());

            out.insert(
                extension.name().to_string(),
                extension
                    .required()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>(),
            );
        }

        out.insert("all".to_string(), all);
    }
}

fn alias_of(original: &str, name: &str, mut out: &mut TokenStream) {
    if original != name {
        quote_each_token! {
            out

            #[doc(alias = #original)]
        }
    }
}
