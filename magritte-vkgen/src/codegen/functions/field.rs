use std::ops::Deref;

use proc_macro2::{TokenStream, Span, Ident};
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
        imports: &Imports,
    ) -> TokenStream {
        // the identifier of the field
        let ident = self.as_ident();

        // the name of the field
        let name = self.original_name();

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


impl<'a> CommandAlias<'a> {
    /// Turns this function into an equivalent function pointer name
    pub fn as_fn_pointer_name(&self, source: &Source<'a>) -> String {
        source
            .functions
            .get_by_name(self.of())
            .or_else(|| source.commands.get_by_name(self.of()).map(Deref::deref))
            .map(Function::as_fn_pointer_name)
            .or_else(|| source.command_aliases.get_by_name(self.of()).map(|alias| alias.as_fn_pointer_name(source)))
            .unwrap()
    }

    /// Turns this function into an equivalent function pointer identifier
    pub fn as_fn_pointer_ident(&self, source: &Source<'a>) -> Ident {
        Ident::new(&self.as_fn_pointer_name(source), Span::call_site())
    }


    /// Gets the conditional compilation tokens for this function
    pub fn conditional_compilation(&self, parent: &Origin<'a>, source: &Source<'a>) -> Option<TokenStream> {
        if self.origin() == parent || parent.requires(source, self.origin()) {
            None
        } else {
            self.origin().condition()
        }
    }


    /// Gets the conditional compilation tokens for this function
    pub fn conditional_compilation_not(&self, parent: &Origin<'a>, source: &Source<'a>) -> Option<TokenStream> {
        if self.origin() == parent || parent.requires(source, self.origin()) {
            None
        } else {
            self.origin().condition_not()
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
        } else {
            self.origin().as_bool_tokens(Some(imports), variant)
        }
    }

    /// Imports the function pointer type
    pub fn import_function_pointer_ty(&self, source: &Source<'a>, imports: &Imports) {
        if let Some(fun) = source.functions.get_by_name(self.of()).or_else(|| source.commands.get_by_name(self.of()).map(Deref::deref)) {
            imports.push_origin(fun.origin(), fun.as_fn_pointer_ident());
        } else if let Some(alias) = source.command_aliases.get_by_name(self.of()) {
            alias.import_function_pointer_ty(source, imports);
        }
    }

    /// Generate the code of this function as a field for a V-table
    pub fn generate_field_code(&self, source: &Source<'a>) -> TokenStream {
        // the identifier of the field
        let name = self.as_ident();

        // the type name identifier
        let ty = self.as_fn_pointer_ident(source);

        // the short documentation
        let doc = format!("See [`{}`] for more information.", self.as_fn_pointer_name(source));

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
        imports: &Imports,
    ) -> TokenStream {
        // the identifier of the field
        let ident = self.as_ident();

        // the name of the field
        let name = self.original_name();

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
        let ty = self.as_fn_pointer_ident(source);

        // the short documentation
        let doc = format!(
            "Gets [`Self::{}`]. See [`{}`] for more information.",
            self.name(),
            self.as_fn_pointer_name(source)
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