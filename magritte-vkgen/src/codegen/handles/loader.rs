use std::ops::Deref;

use ahash::AHashMap;
use heck::ToUpperCamelCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};

use crate::{
    imports::Imports,
    origin::Origin,
    source::{CommandAlias, ExtensionType, Function, Handle, Source},
    symbols::SymbolName,
    ty::Ty,
};

impl<'a> Handle<'a> {
    /// Finds the functions this handle loads
    pub fn loader_functions<'b, 'this>(
        &'this self,
        source: &'b Source<'a>,
    ) -> impl Iterator<Item = &'b Function<'a>> + 'this
    where
        'a: 'b,
        'b: 'this,
    {
        source
            .functions
            .iter()
            .chain(source.commands.iter().map(Deref::deref))
            .filter_map(|fn_| {
                // handles the fact that `vkGetInstanceProcAddr` is actually loaded
                // by the entry
                if fn_.original_name() == "vkGetInstanceProcAddr" {
                    return None;
                }

                // handle the fact that `vkGetDeviceProcAddr` is actually loaded
                // by the instance
                if fn_.original_name() == "vkGetDeviceProcAddr" {
                    if self.original_name() == "VkInstance" {
                        return Some(fn_);
                    } else {
                        return None;
                    }
                }

                match fn_.arguments.get(0).unwrap().ty() {
                    Ty::Named(name) if &**name == self.original_name() => Some(fn_),
                    Ty::Named(name) => {
                        let handle = source.handles.get_by_either(name)?;

                        if handle.original_name() == self.original_name() {
                            Some(fn_)
                        } else if handle.ancestor_loader(source)? == self.original_name() {
                            Some(fn_)
                        } else {
                            None
                        }
                    },
                    _ => None,
                }
            })
    }

    /// Finds the functions this handle loads
    pub fn loader_aliases<'b, 'this>(
        &'this self,
        source: &'b Source<'a>,
    ) -> impl Iterator<Item = &'b CommandAlias<'a>> + 'this
    where
        'a: 'b,
        'b: 'this,
    {
        source.command_aliases.iter().filter_map(|fn_| {
            let of = source
                .functions
                .get_by_either(fn_.of())
                .or_else(|| source.commands.get_by_either(fn_.of()).map(Deref::deref))
                .expect("unknown function");

            match of.arguments.get(0).unwrap().ty() {
                Ty::Named(name) if &**name == self.original_name() => Some(fn_),
                Ty::Named(name) => {
                    let handle = source.handles.get_by_either(name)?;

                    if handle.original_name() == self.original_name() {
                        Some(fn_)
                    } else if handle.ancestor_loader(source)? == self.original_name() {
                        Some(fn_)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        })
    }

    /// Gets the functions grouped by origin
    pub fn functions_by_origin<'b>(
        &self,
        source: &'b Source<'a>,
    ) -> AHashMap<&'b Origin<'a>, Vec<HandleFunction<'a, 'b>>>
    where
        'a: 'b,
    {
        let mut out = AHashMap::<&'b Origin<'a>, Vec<HandleFunction<'a, 'b>>>::default();
        for fun in self
            .loader_functions(source)
            .map(|fun| HandleFunction::Function(fun))
            .chain(self.loader_aliases(source).map(|fun| HandleFunction::Alias(fun)))
            .filter(|f| !f.origin().is_disabled())
        {
            if let Some(out) = out.get_mut(&fun.origin()) {
                out.push(fun);
            } else {
                out.insert(fun.origin(), vec![fun]);
            }
        }
        out
    }

    /// Generates the V-Table code for a loader
    pub fn generate_vtable_code<'b>(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        origins: impl Iterator<Item = &'b Origin<'a>>,
        mut out: &mut TokenStream,
    ) where
        'a: 'b,
    {
        imports.push("crate::extensions::InstanceExtensions");

        // the name of the vtable
        let name = self.this_vtable_ident();

        // the loader type (ourself)
        let loader = self.as_ident();

        // the documentation of the V-Table
        let doc = format!("The V-Table for [`{}`]", self.name());

        let origins = origins.collect::<Vec<_>>();

        for origin in &origins {
            if *origin != self.origin() && !self.origin().requires(source, *origin) {
                if let Some(condition) = origin.feature_gate(source) {
                    imports.push_str(&format!(
                        r##"
                            {}
                            use {}::{};
                        "##,
                        condition,
                        origin.as_path_str(),
                        self.vtable_name(*origin),
                    ));
                } else if *origin != self.origin() {
                    imports.push_origin(*origin, self.vtable_name(*origin));
                }
            } else if *origin != self.origin() {
                imports.push_origin(*origin, self.vtable_name(*origin));
            }
        }

        let conditional_compilations = origins.iter().map(|o| o.condition(source)).collect::<Vec<_>>();

        let idents = origins
            .iter()
            .map(|o| Ident::new(&o.as_name(), Span::call_site()))
            .collect::<Vec<_>>();

        let docs = origins
            .iter()
            .map(|o| format!("See [`{}`] for more information", self.vtable_name(o)));

        let fields = origins.iter().map(|o| {
            let ident = Ident::new(&o.as_name(), Span::call_site());
            let ty = self.vtable_ident(*o);

            if o.always() {
                quote! {
                    pub #ident: #ty
                }
            } else {
                quote! {
                    pub #ident: Option<#ty>
                }
            }
        });

        let values = origins.iter().map(|o| {
            let getter = match o {
                Origin::Extension(name, _, false) => match source.extensions.get_by_name(&name).unwrap().ty() {
                    ExtensionType::Device => quote! { device_extensions},
                    ExtensionType::Instance => quote! { instance_extensions },
                },
                _ => quote! { instance_extensions },
            };

            let ty = self.vtable_ident(*o);
            if let Some(tokens) = o.as_bool_tokens(Some(imports), &getter) {
                quote! {
                    #tokens.then(|| #ty :: load(loader_fn, loader))
                }
            } else {
                quote! {
                    #ty :: load(loader_fn, loader)
                }
            }
        });

        let opt_types = origins.iter().map(|o| {
            let ty = self.vtable_ident(*o);
            if o.as_bool_tokens(Some(imports), &quote! {}).is_some() {
                quote! {
                    Option<&#ty>
                }
            } else {
                quote! {
                    &#ty
                }
            }
        });

        let as_refs = origins.iter().map(|o| {
            if o.as_bool_tokens(Some(imports), &quote! {}).is_some() {
                Some(quote! {
                    .as_ref()
                })
            } else {
                None
            }
        });

        let refs = origins.iter().map(|o| {
            if o.as_bool_tokens(Some(imports), &quote! {}).is_some() {
                None
            } else {
                Some(quote! {
                    &
                })
            }
        });

        let device_extensions = origins
            .iter()
            .filter_map(|o| match o {
                Origin::Extension(name, _, false) => source.extensions.get_by_name(&name),
                _ => None,
            })
            .any(|o| o.ty() == ExtensionType::Device)
            .then(|| {
                imports.push("crate::extensions::DeviceExtensions");
                quote! {
                    device_extensions: &DeviceExtensions,
                }
            });

        quote_each_token! {
            out

            #[doc = #doc]
            pub struct #name {
                #(
                    #[doc = #docs]
                    #conditional_compilations
                    #fields
                ),*
            }

            impl #name {
                #[doc = "Loads the VTable from the owner and the names"]
                #[track_caller]
                pub fn load(
                    loader_fn: unsafe extern "system" fn(#loader, *const std::os::raw::c_char) -> Option<unsafe extern "system" fn()>,
                    loader: #loader,
                    instance_extensions: &InstanceExtensions,
                    #device_extensions
                ) -> Self
                {
                    Self {
                        #(
                            #conditional_compilations
                            #idents: #values
                        ),*
                    }
                }

                #(
                    #[inline(always)]
                    #[track_caller]
                    #conditional_compilations
                    pub const fn #idents(&self) -> #opt_types {
                        #refs self.#idents #as_refs
                    }
                )*
            }
        }
    }

    /// Get the name of a vtable of a certain origin
    pub fn this_vtable_name(&self) -> String {
        format!("{}_VTable", self.name()).to_upper_camel_case()
    }

    /// Get the ident of a vtable of a certain origin
    pub fn this_vtable_ident(&self) -> Ident {
        Ident::new(&self.this_vtable_name(), Span::call_site())
    }

    /// Get the name of a vtable of a certain origin
    pub fn vtable_name(&self, origin: &Origin<'a>) -> String {
        format!("{}_{}_VTable", self.name(), origin.as_short_name()).to_upper_camel_case()
    }

    /// Get the ident of a vtable of a certain origin
    pub fn vtable_ident(&self, origin: &Origin<'a>) -> Ident {
        Ident::new(&self.vtable_name(origin), Span::call_site())
    }

    /// Generates the V-Table code for part of a laoder
    pub fn generate_sub_vtable_code<'b>(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        origin: &Origin<'a>,
        functions: &[HandleFunction<'a, 'b>],
        mut out: &mut TokenStream,
    ) where
        'a: 'b,
    {
        imports.push_origin(&Origin::Vulkan1_0, self.name());

        // the name of the vtable
        let name = self.vtable_ident(origin);

        // the documentation string
        let doc = format!(
            "The V-table of [`{}`] for functions from `{}`",
            self.name(),
            origin.name()
        );

        for fun in functions {
            match fun {
                HandleFunction::Function(fun) => imports.push_origin(fun.origin(), fun.as_fn_pointer_name()),
                HandleFunction::Alias(alias) => alias.import_function_pointer_ty(source, imports),
            }
        }

        // the field definitions
        let fields = functions.iter().map(|fun| match fun {
            HandleFunction::Function(fun) => fun.generate_field_code(),
            HandleFunction::Alias(alias) => alias.generate_field_code(source),
        });

        // the field initialization
        let owner = quote! { loader };

        let field_inits = functions.iter().map(|fun| match fun {
            HandleFunction::Function(fun) => fun.generate_field_init_code(origin, source, &owner, None, imports),
            HandleFunction::Alias(alias) => alias.generate_field_init_code(origin, source, &owner, None, imports),
        });

        let field_getters = functions.iter().map(|fun| match fun {
            HandleFunction::Function(fun) => fun.generate_field_getter_code(origin, source),
            HandleFunction::Alias(alias) => alias.generate_field_getter_code(origin, source),
        });

        let loader = self.as_ident();

        quote_each_token! {
            out

            #[doc = #doc]
            pub struct #name {
                #(#fields),*
            }

            impl #name {
                #[doc = "Loads the VTable from the owner and the names"]
                #[track_caller]
                pub fn load(
                    loader_fn: unsafe extern "system" fn(#loader, *const std::os::raw::c_char) -> Option<unsafe extern "system" fn()>,
                    loader: #loader,
                ) -> Self
                {
                    Self {
                        #(#field_inits),*
                    }
                }

                #(#field_getters)*
            }
        }
    }
}

/// A function that is part of a handle
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum HandleFunction<'a: 'b, 'b> {
    /// A function
    Function(&'b Function<'a>),

    /// An alias of a function
    Alias(&'b CommandAlias<'a>),
}

impl<'a: 'b, 'b> HandleFunction<'a, 'b> {
    /// Gets the origin of the function
    pub fn origin(&self) -> &'b Origin<'a> {
        match self {
            HandleFunction::Function(fn_) => fn_.origin(),
            HandleFunction::Alias(alias) => alias.origin(),
        }
    }
}
