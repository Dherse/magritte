use std::borrow::Cow;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};

use crate::{
    codegen::functions::wrapped::{ExtraCallInfo, StatefulFunctionGeneratorState},
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Function, Handle, Source, CommandAlias},
    ty::{Ty, Mutability},
};

pub mod field;
pub mod raw;
pub mod wrapped;

impl<'a> CommandAlias<'a> {
    fn function_getter(
        &self,
        source: &Source<'a>,
        handle: &Handle<'a>
    ) -> TokenStream {
        let name = self.as_ident();
        let loader = match handle.name() {
            "Device" | "Instance" => None,
            _ => match &*handle.ancestor_loader(source).unwrap() {
                "VkDevice" => Some(quote! { .device() }),
                "VkInstance" => Some(quote! { .instance() }),
                other => unreachable!("not a loader: {}", other),
            },
        };

        let origin_ident = Ident::new(&self.origin().as_name(), Span::call_site());

        if self.origin() == &Origin::Vulkan1_0 {
            quote! {
                self #loader .vtable().vulkan1_0().#name()
            }
        } else {
            quote! {
                self #loader .vtable().#origin_ident().and_then(|vtable| vtable.#name())
            }
        }
    }
}

impl<'a> Function<'a> {
    fn function_getter(
        &self,
        parent: &Origin<'a>,
        source: &Source<'a>,
        handle: &Handle<'a>
    ) -> TokenStream {
        let name = self.as_ident();
        let loader = match self.original_name() {
            "vkGetInstanceProcAddr" => Some(quote! { .entry() }),
            "vkGetDeviceProcAddr" => Some(quote! { .instance() }),
            _ => match handle.name() {
                "Device" | "Instance" => None,
                _ => match &*handle.ancestor_loader(source).unwrap() {
                    "VkDevice" => Some(quote! { .device() }),
                    "VkInstance" => Some(quote! { .instance() }),
                    other => unreachable!("not a loader: {}", other),
                },
            },
        };

        let origin_ident = Ident::new(&self.origin().as_name(), Span::call_site());

        let aliases = self
            .aliases()
            .iter()
            .map(|alias| source.command_aliases.get_by_name(&**alias).unwrap())
            .map(|alias| alias.function_getter(source, handle));

        let conditions = self
            .aliases()
            .iter()
            .map(|alias| source.command_aliases.get_by_name(&**alias).unwrap())
            .map(|alias| alias.conditional_compilation(parent, source));


        let condition_nots = self
            .aliases()
            .iter()
            .map(|alias| source.command_aliases.get_by_name(&**alias).unwrap())
            .map(|alias| alias.conditional_compilation_not(parent, source));
        
        if self.original_name() == "vkGetInstanceProcAddr" {
            quote! {
                self
                    .entry()
                    .vtable()
                    .get_instance_proc_addr()#(
                        .or_else(|| {
                            #conditions
                            return #aliases;
                            #condition_nots
                            return None;
                        })
                    )*
            }
        } else if self.origin() == &Origin::Vulkan1_0 {
            quote! {
                self #loader .vtable().vulkan1_0().#name()#(
                    .or_else(|| {
                        #conditions
                        return #aliases;
                        #condition_nots
                        return None;
                    })
                )*
            }
        } else {
            quote! {
                self #loader .vtable().#origin_ident().and_then(|vtable| vtable.#name())#(
                    .or_else(|| {
                        #conditions
                        return #aliases;
                        #condition_nots
                        return None;
                    })
                )*
            }
        }
    }

    /// generates the code for the function
    pub fn generate_code<'b>(
        &self,
        source: &'b Source<'a>,
        imports: &Imports,
        doc: &mut Documentation,
        mut handle: &'b Handle<'a>,
        mut out: &mut TokenStream,
    ) -> Option<String> {
        let mut gen = StatefulFunctionGeneratorState::new(source, imports, handle, self.clone(), None);

        let mut change = None;
        if let Some(new_handle) = gen.change_handle {
            handle = source.handles.get_by_name(&new_handle).unwrap();
            // gen = StatefulFunctionGeneratorState::new(source, imports, handle, self.clone(), None);

            change = Some(new_handle);
        }


        let name = self.as_ident();

        self.as_fn_pointer().generate_doc(source, doc, out);

        let extra_call = gen.extra_call_info.map(|info| {
            let ExtraCallInfo {
                arguments,
                length_index,
                param_name,
                ..
            } = info;

            let call_args = gen.call_args.iter().enumerate().map(|(i, arg)| {
                if i == length_index {
                    quote! { &mut v }
                } else if arguments.contains(&i) {
                    quote! { std::ptr::null_mut() }
                } else {
                    arg(None)
                }
            });

            quote! {
                let mut #param_name = match #param_name {
                    Some(v) => v as _,
                    None => {
                        let mut v = 0;
                        _function(#(#call_args),*);
                        v
                    }
                };
            }
        });

        let inner_return_ty = if gen.return_types.len() == 1 {
            gen.return_types.pop().unwrap()
        } else {
            let return_types = gen.return_types.into_iter();

            quote! {
                (#(#return_types),*)
            }
        };

        let return_ty = match self.return_type() {
            Some(Ty::Named(Cow::Borrowed("VkResult"))) => {
                imports.push("crate::VulkanResult");
                quote! {
                    VulkanResult<#inner_return_ty>
                }
            },
            _ => inner_return_ty,
        };

        let inner_return_value = if gen.return_values.len() == 1 {
            gen.return_values.pop().unwrap()
        } else {
            let return_values = gen.return_values.into_iter();

            quote! {
                (#(#return_values),*)
            }
        };

        let return_expr = match self.return_type() {
            Some(Ty::Named(Cow::Borrowed("VkResult"))) => {
                imports.push_origin(&Origin::Vulkan1_0, "VulkanResultCodes");
                imports.push("crate::VulkanResult");

                let result = source.enums.get_by_either("VulkanResultCodes").unwrap();
                let successes = self.success_codes.iter().map(|name| {
                    if let Some(variant) = result.variants().get_by_either(name) {
                        variant.as_ident()
                    } else if let Some(alias) = result.aliases().get_by_either(name) {
                        result.variants.get_by_name(alias.of()).unwrap().as_ident()
                    } else {
                        unreachable!("unknown result variant: {}", name);
                    }
                });

                quote! {
                    match _return {
                        #(VulkanResultCodes::#successes) |* => VulkanResult::Success(_return, #inner_return_value),
                        e => VulkanResult::Err(e)
                    }
                }
            },
            _ => inner_return_value,
        };

        let params = gen.function_args.into_iter();
        let states = gen.states.into_iter();
        let return_init = gen.return_initializations.into_iter();

        let call_args = gen.call_args.into_iter().map(|fun| fun(None));

        let is_lifetime = self.has_lifetime(source);

        let generics = if is_lifetime {
            Some(quote! {
                'lt,
            })
        } else {
            None
        };

        // println!("{}: {:#?}", self.original_name(), gen);

        // let unsafe_ = if gen.unsafe_ { Some(quote! { unsafe }) } else { None };
        let function_getter = self.function_getter(handle.origin(), source, handle);

        let mut aliases = vec![ self.original_name() ];
        aliases.extend(self.aliases().iter().map(|s| &**s));

        let ident = handle.as_ident();
        let this = match gen.this {
            Mutability::Mutable => quote! { self: &'this mut Unique<'a, #ident> },
            Mutability::Const => quote! { self: &'this Unique<'a, #ident> },
        };

        quote_each_token! {
            out

            #(
                #[doc(alias = #aliases)]
            )*
            #[track_caller]
            #[inline]
            pub unsafe fn #name <'a: 'this, 'this, #generics>(
                #this,
                #(#params),*
            ) -> #return_ty {
                #[cfg(any(debug_assertions, feature = "assertions"))]
                let _function = #function_getter.expect("function not loaded");

                #[cfg(not(any(debug_assertions, feature = "assertions")))]
                let _function = #function_getter.unwrap_unchecked();

                #(
                    #states
                )*

                #extra_call

                #(
                    #return_init
                )*

                let _return = _function(#(#call_args),*);

                #return_expr
            }
        }

        change
    }
}
