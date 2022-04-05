use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    imports::Imports,
    origin::Origin,
    source::{CommandAlias, Function, Source},
};

impl<'a> Function<'a> {
    /// Gets the conditional compilation tokens for this function
    pub fn conditional_compilation(&self, parent: &Origin<'a>, source: &Source<'a>) -> Option<TokenStream> {
        if self.origin() == parent || parent.requires(source, self.origin()) {
            None
        } else if self.aliases().is_empty() {
            self.origin().condition()
        } else {
            Origin::or(
                std::iter::once(self.origin()).chain(
                    self.aliases()
                        .iter()
                        .map(|alias| source.command_aliases.get_by_name(alias).expect("unknown alias"))
                        .map(CommandAlias::origin),
                ),
            )
        }
    }

    /// Gets the conditional boolean tokens for this function
    pub fn boolean_condition(
        &self,
        source: &Source<'a>,
        parent: &Origin<'a>,
        variant: &TokenStream,
        imports: &Imports,
    ) -> Option<TokenStream> {
        if self.origin() == parent || parent.requires(source, self.origin()) {
            None
        } else if self.aliases().is_empty() {
            self.origin().as_bool_tokens(Some(imports), variant)
        } else {
            Origin::or_bool(
                std::iter::once(self.origin()).chain(
                    self.aliases()
                        .iter()
                        .map(|alias| source.command_aliases.get_by_name(alias).expect("unknown alias"))
                        .map(CommandAlias::origin),
                ),
                variant,
                Some(imports),
            )
        }
    }

    /// Generate the code of this function as a field for a V-table
    pub fn generate_field_code(&self) -> TokenStream {
        // the identifier of the field
        let name = self.as_ident();

        // the type name identifier
        let ty = self.as_fn_pointer_ident();

        // the short documentation
        let doc = format!("See [`{}`] for more information.", self.as_fn_pointer_name());

        quote! {
            #[doc = #doc]
            pub #name: #ty
        }
    }

    /// Generate the code for initializing the field
    pub fn generate_field_init_code(
        &self,
        parent: &Origin<'a>,
        source: &Source<'a>,
        owner: &TokenStream,
        variant: Option<&TokenStream>,
        rename: Option<&str>,
        imports: &Imports,
    ) -> TokenStream {
        // the identifier of the field
        let ident = self.as_ident();

        // the name of the field
        let name = rename.unwrap_or_else(|| self.original_name());

        // conditional compilation
        let conditional_compilation = self.conditional_compilation(parent, source);

        let init = quote! {
            unsafe {
                std::mem::transmute(loader_fn(#owner, crate::cstr!(#name).as_ptr()))
            }
        };

        // shorthand to avoid unnecessary code generation
        if self.origin() == parent || parent.requires(source, self.origin()) || variant.is_none() {
            return quote! {
                #ident: #init
            };
        }

        let value = if let Some(check) = self.boolean_condition(source, parent, variant.unwrap(), imports) {
            quote! {
                if #check { #init } else { None }
            }
        } else {
            quote! {
                #init
            }
        };

        quote! {
            #conditional_compilation
            #ident: #value
        }
    }

    /// Generates the code for a getter of this function
    pub fn generate_field_getter_code(&self, parent: &Origin<'a>, source: &Source<'a>) -> TokenStream {
        // the identifier of the field
        let name = self.as_ident();

        // the type name identifier
        let ty = self.as_fn_pointer_ident();

        // the short documentation
        let doc = format!(
            "Gets [`Self::{}`]. See [`{}`] for more information.",
            self.name(),
            self.as_fn_pointer_name()
        );

        // conditional compilation
        let conditional_compilation = self.conditional_compilation(parent, source);

        quote! {
            #[doc = #doc]
            #conditional_compilation
            pub fn #name(&self) -> #ty {
                self.#name
            }
        }
    }
}
