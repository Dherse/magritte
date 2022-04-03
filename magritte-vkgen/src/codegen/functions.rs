use std::{borrow::Cow, ops::Not};

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token};

use crate::{
    codegen::functions::wrapped::{ExtraCallInfo, StatefulFunctionGeneratorState},
    doc::Documentation,
    imports::Imports,
    name::enum_name,
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
        let mut gen = StatefulFunctionGeneratorState::new(source, imports, handle, self.clone(), Some(quote! { 'lt }));

        let name = self.as_ident();
        let original_name = self.original_name();

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

                let successes = self
                    .success_codes
                    .iter()
                    .map(|name| Ident::new(&enum_name(name, None, Some("VkResult")), Span::call_site()));

                quote! {
                    match _result {
                        #(VulkanResultCodes::#successes) |* => VulkanResult::Success(_result, #inner_return_value),
                        e => VulkanResult::Err(e)
                    }
                }
            },
            _ => inner_return_value,
        };

        let params = gen.function_args.into_iter();
        let state_idents = gen.state_idents.into_iter();
        let state_ty = gen.state_types.into_iter();
        let state_init = gen.state_initializations.into_iter();
        let state_set = gen.state_set.into_iter().map(|fun| fun(None));

        let return_init = gen.return_initializations.into_iter();

        let call_args = gen.call_args.into_iter().map(|fun| fun(None));

        let is_lifetime = self.has_lifetime(source);

        let generics = if gen.generic_idents.is_empty() {
            if is_lifetime {
                Some(quote! {
                    <'a>
                })
            } else {
                None
            }
        } else {
            let generic = gen.generic_idents.iter();
            let lifetime = is_lifetime.then(|| quote! { 'a, });

            Some(quote! {
                <#lifetime #(#generic),*>
            })
        };

        let generic_bounds = if gen.generic_bounds.is_empty() {
            None
        } else {
            let generic = gen.generic_idents.iter();
            let bound = gen.generic_bounds.iter();

            Some(quote! {
                where #(#generic: #bound),*
            })
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

        let this = gen.this.not().then(|| quote! { &self, });

        let origin = Ident::new(&self.origin().as_name(), Span::call_site());

        quote_each_token! {
            out

            #alias
            #[track_caller]
            #[inline]
            pub unsafe fn #name #generics(
                #this
                #(#params),*
            ) -> #return_ty #generic_bounds {
                #[cfg(any(debug_assertions, feature = "assertions"))]
                let _function = self.vtable().#origin().#name().expect("not loaded");

                #[cfg(not(any(debug_assertions, feature = "assertions")))]
                let _function = self.vtable().#origin().#name().unwrap_unchecked();

                #(
                    let mut #state_idents: #state_ty = #state_init;
                )*

                #(
                    #state_set
                )*

                #(
                    #return_init
                )*

                #extra_call

                let _return = _function(#(#call_args),*);

                #return_expr
            }
        }
    }
}
