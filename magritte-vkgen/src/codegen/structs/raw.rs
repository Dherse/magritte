use std::borrow::Cow;

use ahash::AHashMap;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_each_token};
use syn::{Ident, Token};
use tracing::warn;

use crate::{
    codegen::{ty::{lifetime_as_generic_argument, lifetime_as_lifetime}, alias_of},
    doc::Documentation,
    expr::Expr,
    imports::Imports,
    source::{Field, Source, Struct},
    ty::{Mutability, Ty},
};

impl<'a> Struct<'a> {
    /// Generates the code for the raw C-compatible struct
    pub fn generate_raw_code(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        imports: &Imports,
        mut out: &mut TokenStream,
    ) {
        // get the name as an identifier
        let name = self.as_ident();

        // generate the derives
        let copy = self.is_copy(source).then(|| quote! { #[derive(Clone, Copy)] });
        let partial_eq_ord = self
            .is_copy(source)
            .then(|| quote! { #[derive(PartialEq, PartialOrd)] });
        let eq_ord = self.is_eq(source).then(|| quote! { #[derive(Eq, Ord)] });
        let hash = self.is_hash(source).then(|| quote! { #[derive(Hash)] });
        let serde = self
            .is_serde(source)
            .then(|| quote! { #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))] });

        // create the lifetime generic argument
        let lt = lifetime_as_generic_argument();
        let lifetime = self.has_lifetime(source).then(|| {
            quote! {
                <#lt>
            }
        });

        // create a transparent, zero-sized field if there is a lifetime
        let lifetime_field = self.has_lifetime(source).then(|| {
            imports.push("std::marker::PhantomData");

            let lt = lifetime_as_lifetime();
            quote! {
                pub _lifetime: PhantomData<&#lt ()>,
            }
        });

        let lifetime_default = self.has_lifetime(source).then(|| {
            quote! {
                _lifetime: PhantomData,
            }
        });

        // get the documentation and the documentation of each field
        let field_doc = self.generate_doc(source, doc, out).unwrap_or_default();

        // creates the field list
        let fields = self
            .fields()
            .iter()
            .map(|field| field.generate_raw_code(source, imports, &field_doc));

        let raw_getters = self
            .fields()
            .iter()
            .filter_map(|field| field.generate_raw_getter(source, imports));
        let raw_setters = self
            .fields()
            .iter()
            .filter_map(|field| field.generate_raw_setter(source, imports));
        let pretty_getters = self
            .fields()
            .iter()
            .map(|field| field.generate_getter(self, source, imports));
        let pretty_mut_getters = self
            .fields()
            .iter()
            .filter_map(|field| field.generate_mut_getter(source, imports, self));
        let pretty_setters = self.fields().iter().map(|field| field.generate_setter(source, self));
        let field_defaults = self
            .fields()
            .iter()
            .map(|field| field.generate_default(source, imports));

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        quote_each_token! {
            out

            #[derive(Debug)]
            #copy
            #eq_ord
            #partial_eq_ord
            #hash
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #serde
            #[repr(C)]
            pub struct #name #lifetime {
                #lifetime_field
                #(#fields),*
            }

            impl #lifetime Default for #name #lifetime {
                fn default() -> Self {
                    Self {
                        #lifetime_default
                        #(#field_defaults),*
                    }
                }
            }

            impl #lifetime #name #lifetime {
                #(#raw_getters)*
                #(#raw_setters)*
                #(#pretty_getters)*
                #(#pretty_mut_getters)*
                #(#pretty_setters)*
            }
        }
    }

    /// Generates the documentation for a raw struct
    fn generate_doc(
        &self,
        source: &Source<'a>,
        doc: &mut Documentation,
        out: &mut TokenStream,
    ) -> Option<AHashMap<String, String>> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the description section
            let mut fields = AHashMap::with_capacity(self.fields().len());

            // parse the members and write them out
            doc.members(source, self, out, Some(&mut fields));

            // parse the descirption and write it out
            // if we did not find the fields in `members`, try again here, this happens in some man pages
            doc.description(source, self, out, fields.is_empty().then(|| &mut fields));

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(fields)
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }
}

impl<'a> Field<'a> {
    /// Generates the code for the raw C-compatible struct
    pub fn generate_raw_code(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        doc: &AHashMap<String, String>,
    ) -> TokenStream {
        // get the name as an identifier of the field
        let name = self.as_ident();

        // get the type of the field
        let ty = self.ty().as_raw_ty(source, Some(imports), true).0;

        // get the doc of the field
        let doc = doc.get(self.name()).map_or_else(
            || quote! { #[doc = "No documentation found"]},
            |t| quote! { #[doc = #t] },
        );

        quote! {
            #doc
            pub #name: #ty
        }
    }

    /// Generates the code for a getter that returns the raw C-compatible value
    pub(super) fn generate_raw_getter(&self, source: &Source<'a>, imports: &Imports) -> Option<TokenStream> {
        // if we don't require conversion, there is no need to have a `raw` function
        if !self.ty().requires_conversion(source) {
            return None;
        }
        // get the name as an identifier of the field
        let name = self.as_ident();

        // create the function name
        let getter_name = format!("{}_raw", self.name());
        let ident = Ident::new(&getter_name, Span::call_site());

        // get the type of the field
        let ty = self.ty().as_raw_ty(source, Some(imports), true).0;

        // generate the ref or not
        let is_not_copy = !self.ty().is_copy(source);
        let ref_ = is_not_copy.then(|| quote! { & });

        // generate the documentation
        let doc = format!("Gets the raw value of [`Self::{}`]", self.name());

        Some(quote! {
            #[doc = #doc]
            pub fn #ident(&self) -> #ref_ #ty {
                #ref_ self.#name
            }
        })
    }

    /// Generates the prettified getter for this field
    pub(super) fn generate_getter(&self, owner: &Struct<'a>, source: &Source<'a>, imports: &Imports) -> TokenStream {
        // get the name as an identifier of the field
        let name = self.as_ident();

        // get the type of the field
        let ty = self.ty().as_ty(source, Some(imports)).0;

        // generate the documentation
        let doc = format!("Gets the value of [`Self::{}`]", self.name());

        // unsafe keyword if needed
        let is_unsafe = !self.ty().is_safe_conversion();
        let unsafe_ = is_unsafe.then(|| quote! { unsafe });

        // generate some documentation in case the function is unsafe
        let safety_doc = is_unsafe.then(|| {
            quote! {
                #[doc = "# Safety"]
                #[doc = "This function converts a pointer into a value which may be invalid, make sure"]
                #[doc = "that the pointer is valid before dereferencing."]
            }
        });

        // get the length expression if needed
        let len = {
            if self.ty().length().map(Expr::variables).map_or(true, |v| v.is_empty()) {
                None
            } else {
                Some(self.ty().length().unwrap().as_expr(
                    source,
                    &|name| {
                        let ident = owner.get_field(name).expect("unknown field").as_ident();
                        quote! {
                            self.#ident
                        }
                    },
                    Some(imports),
                ))
            }
        };

        // create the converter
        let (converter, ref_) = self
            .ty()
            .c_to_rust_converter(source, Mutability::Const, quote! { self.#name }, len)
            .expect("failed to created const converter");

        // if the output is a reference, get it
        let ref_ = ref_.then(|| Token![&](Span::call_site()));

        quote! {
            #[doc = #doc]
            #safety_doc
            pub #unsafe_ fn #name(&self) -> #ref_ #ty {
                #converter
            }
        }
    }

    /// Generates the prettified getter for this field
    pub(super) fn generate_mut_getter(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        owner: &Struct<'a>,
    ) -> Option<TokenStream> {
        // get the name as an identifier of the field
        let fn_name = format!("{}_mut", self.name());
        let ident = Ident::new(&fn_name, Span::call_site());

        let name = self.as_ident();

        // get the type of the field
        let ty = self.ty().as_ty(source, Some(imports)).0;

        // generate the documentation
        let doc = format!("Gets a mutable reference to the value of [`Self::{}`]", self.name());

        // unsafe keyword if needed
        let is_unsafe = !self.ty().is_safe_conversion();
        let unsafe_ = is_unsafe.then(|| quote! { unsafe });

        // generate some documentation in case the function is unsafe
        let safety_doc = is_unsafe.then(|| {
            quote! {
                #[doc = "# Safety"]
                #[doc = "This function converts a pointer into a value which may be invalid, make sure"]
                #[doc = "that the pointer is valid before dereferencing."]
            }
        });

        // get the length expression if needed
        let len = {
            if self.ty().length().map(Expr::variables).map_or(true, |v| v.is_empty()) {
                None
            } else {
                Some(self.ty().length().unwrap().as_expr(
                    source,
                    &|name| {
                        let ident = owner.get_field(name).expect("unknown field").as_ident();
                        quote! {
                            self.#ident
                        }
                    },
                    Some(imports),
                ))
            }
        };

        // create the converter
        let (converter, ref_) =
            self.ty()
                .c_to_rust_converter(source, Mutability::Mutable, quote! { self.#name }, len)?;

        // if the output is a reference, get it
        let ref_ = ref_.then(|| quote! { &mut });

        Some(quote! {
            #[doc = #doc]
            #safety_doc
            pub #unsafe_ fn #ident(&mut self) -> #ref_ #ty {
                #converter
            }
        })
    }

    /// Generates the code for a getter that returns the raw C-compatible value
    pub(super) fn generate_raw_setter(&self, source: &Source<'a>, imports: &Imports) -> Option<TokenStream> {
        // if we don't require conversion, there is no need to have a `raw` function
        if !self.ty().requires_conversion(source) {
            return None;
        }

        // get the name as an identifier of the field
        let name = self.as_ident();

        // create the function name
        let setter_name = format!("set_{}_raw", self.name());
        let ident = Ident::new(&setter_name, Span::call_site());

        // get the type of the field
        let ty = self.ty().as_raw_ty(source, Some(imports), true).0;

        // generate the documentation
        let doc = format!("Sets the raw value of [`Self::{}`]", self.name());

        Some(quote! {
            #[doc = #doc]
            pub fn #ident(&mut self, value: #ty) -> &mut Self {
                self.#name = value;

                self
            }
        })
    }

    /// Generates the code for a getter that returns the raw C-compatible value
    pub(super) fn generate_setter(&self, source: &Source<'a>, owner: &Struct<'a>) -> Option<TokenStream> {
        // create the function name
        let setter_name = format!("set_{}", self.name());
        let ident = Ident::new(&setter_name, Span::call_site());

        // generate the documentation
        let doc = format!("Sets the raw value of [`Self::{}`]", self.name());

        let len_field: Option<Cow<'a, str>> = if let Ty::Slice(_, _, len) = self.ty() {
            let vars = len.variables();
            vars.get(0).cloned()
        } else {
            None
        };

        let (fields, tokens) = self.ty().rust_to_c_converter(
            source,
            &|field, ident| {
                let field = owner.get_field(field).unwrap().as_ident();
                quote! {
                    self.#field = #ident;
                }
            },
            self.name(),
            len_field.as_ref().map(|s| s as &str),
        );

        Some(quote! {
            #[doc = #doc]
            pub fn #ident(&mut self, #(#fields),*) -> &mut Self {
                #tokens

                self
            }
        })
    }

    /// Generate the default value
    pub(super) fn generate_default(&self, source: &Source<'a>, imports: &Imports) -> TokenStream {
        let name = self.as_ident();

        let default = self.ty().default_tokens(source, Some(imports));

        quote! {
            #name: #default
        }
    }
}
