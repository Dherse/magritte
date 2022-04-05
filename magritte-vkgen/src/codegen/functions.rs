use std::{borrow::Cow, ops::Not};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};

use crate::{
    codegen::functions::wrapped::{ExtraCallInfo, StatefulFunctionGeneratorState},
    doc::Documentation,
    imports::Imports,
    origin::Origin,
    source::{Function, Handle, Source},
    ty::Ty,
};

pub mod field;
pub mod raw;
pub mod wrapped;

impl<'a> Function<'a> {
    /// generates the code for the function
    pub fn generate_code(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        doc: &mut Documentation,
        handle: &Handle<'a>,
        mut out: &mut TokenStream,
    ) {
        let mut gen = StatefulFunctionGeneratorState::new(source, imports, handle, self.clone(), None);

        let name = self.as_ident();
        let original_name = self.original_name();

        self.as_fn_pointer().generate_doc(source, doc, out);

        let loader = match self.name() {
            "get_instance_proc_addr" => Some(quote! { .entry() }),
            "get_device_proc_addr" => Some(quote! { .instance() }),
            _ => match handle.name() {
                "Device" | "Instance" => None,
                _ => match &*handle.ancestor_loader(source).unwrap() {
                    "VkDevice" => Some(quote! { .device() }),
                    "VkInstance" => Some(quote! { .instance() }),
                    other => unreachable!("not a loader: {}", other),
                },
            },
        };

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

        let alias = if !self.original_name().eq(self.name()) {
            Some(quote! {
                #[doc(alias = #original_name)]
            })
        } else {
            None
        };

        // let this = gen.this.not().then(|| quote! { self: &Unique<'a, #handle_ident>, });

        let origin = if self.name() == "get_instance_proc_addr" {
            None
        } else {
            let ident = Ident::new(&self.origin().as_name(), Span::call_site());

            Some(quote! {
                .#ident()
            })
        };

        let origin_expect =
            (self.origin() != &Origin::Vulkan1_0 && self.name() != "get_instance_proc_addr").then(|| {
                quote! {
                    .expect("extension/version not loaded")
                }
            });

        let origin_unwrap =
            (self.origin() != &Origin::Vulkan1_0 && self.name() != "get_instance_proc_addr").then(|| {
                quote! {
                    .unwrap_unchecked()
                }
            });

        let lifetime = gen.lifetime.not().then(|| quote! { 'parent, });

        quote_each_token! {
            out

            #alias
            #[track_caller]
            #[inline]
            pub unsafe fn #name <'a: 'this, #lifetime 'this, #generics>(
                // #this
                #(#params),*
            ) -> #return_ty {
                #[cfg(any(debug_assertions, feature = "assertions"))]
                let _function = self #loader .vtable() #origin #origin_expect.#name().expect("function not loaded");

                #[cfg(not(any(debug_assertions, feature = "assertions")))]
                let _function = self #loader .vtable() #origin #origin_unwrap.#name().unwrap_unchecked();

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
    }
}
