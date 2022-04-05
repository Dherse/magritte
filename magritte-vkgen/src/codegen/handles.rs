pub mod loader;

use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token, ToTokens};
use tracing::warn;

use crate::{
    codegen::{alias_of, functions::wrapped::StatefulFunctionGeneratorState},
    doc::Documentation,
    imports::Imports,
    source::{Function, Handle, Source},
};

impl<'a> Handle<'a> {
    /// Generates the code for a handle
    pub(super) fn generate_code(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        doc: &mut Documentation,
        mut out: &mut TokenStream,
    ) {
        // the name as an identifier
        let name = self.as_ident();

        // the type of the constant
        let ty = if self.dispatchable() {
            quote! { *mut () }
        } else {
            quote! { u64 }
        };

        let default = if self.dispatchable() {
            quote! { std::ptr::null_mut() }
        } else {
            quote! { 0 }
        };

        // append the doc first
        self.generate_doc(source, doc, out);

        // creates a doc alias if the name has been changed
        alias_of(self.original_name(), self.name(), out);

        // TODO: fns_

        imports.push("crate::Handle");

        let parent = if let Some(parent) = self.parent() {
            let handle = source.handles.get_by_name(parent).expect("unknown parent");

            imports.push_origin(handle.origin(), handle.name());
            imports.push("crate::Unique");

            let ident = handle.as_ident();

            quote! {
                Unique<'a, #ident>
            }
        } else {
            imports.push("crate::entry::Entry");
            quote! { Entry }
        };

        let mut ancestors = Vec::new();

        let ancestor_fn = |count, name| {
            let doc_str = format!("Gets the reference to the [`{}`]", name);
            let ident = Ident::new(name, Span::call_site());
            let func_name = Ident::new(&name.to_case(Case::Snake), Span::call_site());

            let origin = source.handles.get_by_either(name).unwrap().origin();

            imports.push_origin(origin, name);

            let ancestors = (0..count).into_iter().map(|_| quote! { parent });

            quote! {
                #[doc = #doc_str]
                #[inline]
                pub fn #func_name(&self) -> &'a Unique<'a, #ident> {
                    self #(
                        .#ancestors()
                    )*
                }
            }
        };

        let entry_count = self.find_entry_ancestors(source);
        let entry_ancestors = (0..entry_count).into_iter().map(|_| quote! { parent });
        imports.push("crate::entry::Entry");
        ancestors.push(quote! {
            #[doc = "Gets the reference to the [`Entry`]"]
            #[inline]
            pub fn entry(&self) -> &'a Entry {
                self #(
                    .#entry_ancestors()
                )*
            }
        });

        if let Some(instance_ancestors) = self.find_instance_ancestors(source) {
            ancestors.push(ancestor_fn(instance_ancestors, "Instance"));
        }

        if let Some(physical_device_ancestors) = self.find_physical_device_ancestors(source) {
            ancestors.push(ancestor_fn(physical_device_ancestors, "PhysicalDevice"));
        }

        if let Some(device_ancestors) = self.find_device_ancestors(source) {
            ancestors.push(ancestor_fn(device_ancestors, "Device"));
        }

        if let Some(command_pool_ancestors) = self.find_command_pool_ancestors(source) {
            ancestors.push(ancestor_fn(command_pool_ancestors, "CommandPool"));
        }

        if let Some(descriptor_pool_ancestors) = self.find_descriptor_pool_ancestors(source) {
            ancestors.push(ancestor_fn(descriptor_pool_ancestors, "DescriptorPool"));
        }

        if let Some(display_ancestors) = self.find_display_ancestors(source) {
            ancestors.push(ancestor_fn(display_ancestors, "DisplayKHR"));
        }

        if let Some(surface_ancestors) = self.find_surface_ancestors(source) {
            ancestors.push(ancestor_fn(surface_ancestors, "SurfaceKHR"));
        }

        if let Some(video_session_ancestors) = self.find_video_session_ancestors(source) {
            ancestors.push(ancestor_fn(video_session_ancestors, "VideoSessionKHR"));
        }

        let vtable = if self.is_loader() {
            self.this_vtable_ident().to_token_stream()
        } else {
            quote! { () }
        };

        let metadata = if self.name() == "Instance" {
            imports.push("crate::extensions::Extensions");
            quote! { Extensions }
        } else {
            quote! { () }
        };

        let load_vtable = if self.is_loader() {
            let ident = self.this_vtable_ident();

            let loader_fn = match self.name() {
                "Device" => quote! { parent.instance().vtable().vulkan1_0().get_device_proc_addr().unwrap() },
                "Instance" => quote! { parent.vtable().get_instance_proc_addr().unwrap() },
                _ => unreachable!(),
            };

            let extensions = match self.name() {
                "Device" => quote! { parent.instance().metadata() },
                "Instance" => quote! { metadata },
                _ => unreachable!(),
            };

            quote! {
                #ident::load(
                    #loader_fn,
                    *self,
                    #extensions,
                )
            }
        } else {
            quote! { () }
        };

        let destroy = self
            .destroyer()
            .map(|name| source.functions.get_by_name(name).unwrap())
            .map(|func| {
                if self.name() == "Instance" {
                    return quote! {
                        #[cfg(any(debug_assertions, feature = "assertions"))]
                        let _function = self.vtable().vulkan1_0().destroy_instance().expect("not loaded");

                        #[cfg(not(any(debug_assertions, feature = "assertions")))]
                        let _function = self.vtable().vulkan1_0().destroy_instance().unwrap_unchecked();

                        _function(
                            **self,
                            std::ptr::null()
                        );
                    };
                }

                let destroyer = func.as_ident();

                let owner = source
                    .handles
                    .get_by_name(func.arguments()[0].ty().as_named())
                    .expect("unknown handle");

                let owner_getter = match owner.name() {
                    "Device" => quote! { device },
                    "Instance" => quote! { instance },
                    "PhysicalDevice" => quote! { physical_device },
                    other => unreachable!("unsupported owner: {}", other),
                };

                match func.arguments().len() {
                    2 => {
                        if func.original_name() == "vkReleaseDisplayEXT" {
                            quote! {
                                #[cfg(feature = "VK_EXT_direct_mode_display")]
                                self.parent().#destroyer(
                                    self.as_raw()
                                );
                            }
                        } else {
                            quote! {
                                self.#destroyer(
                                    None
                                );
                            }
                        }
                    },
                    3 => {
                        quote! {
                            self.#owner_getter().#destroyer(
                                Some(self.as_raw()),
                                None
                            );
                        }
                    },
                    4 => {
                        quote! {
                            self.#owner_getter().#destroyer(
                                self.parent().as_raw(),
                                &[
                                    self.as_raw()
                                ],
                            );
                        }
                    },
                    len => unreachable!("unsupported number of arguments: {}", len),
                }
            });

        quote_each_token! {
            out

            #[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
            #[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
            #[repr(transparent)]
            pub struct #name(pub #ty);

            impl #name {
                #[doc = "Creates a new null handle"]
                #[inline]
                pub const fn null() -> Self {
                    Self(#default)
                }

                #[doc = "Checks if this is a null handle"]
                #[inline]
                pub fn is_null(&self) -> bool {
                    self == &Self::null()
                }

                #[doc = "Gets the raw value"]
                #[inline]
                pub fn raw(&self) -> #ty {
                    self.0
                }

                // #fns
            }

            unsafe impl Send for #name {}

            impl Default for #name {
                fn default() -> Self {
                    Self::null()
                }
            }

            impl Handle for #name {
                type Parent<'a> = #parent;

                type VTable = #vtable;

                type Metadata = #metadata;

                #[inline]
                #[track_caller]
                unsafe fn destroy<'a>(self: &mut Unique<'a, Self>) {
                    #destroy
                }

                #[inline]
                unsafe fn load_vtable<'a>(&self, parent: &Self::Parent<'a>, metadata: &Self::Metadata) -> Self::VTable {
                    #load_vtable
                }
            }

            impl<'a> Unique<'a, #name> {
                #(#ancestors)*
            }
        }
    }

    /// Generates the documentation for a handle
    fn generate_doc(&self, source: &Source<'a>, doc: &mut Documentation, out: &mut TokenStream) -> Option<()> {
        if let Some(mut doc) = doc.find(self.original_name()) {
            // parse the name section and write it out
            doc.name(source, self, out);

            // parse the c spec section and write it out
            doc.specification(source, self, out);

            // parse the related elements and write them out
            doc.related(source, out);

            // adds the copyright of the Vulkan docs
            doc.copyright(out);

            Some(())
        } else {
            warn!("No documentation for {}", self.original_name());

            // add the default no doc comment
            Documentation::no_doc(out);

            None
        }
    }

    /// Generate the code for a function
    pub fn generate_fn_code(
        &self,
        source: &Source<'a>,
        imports: &Imports,
        doc: &mut Documentation,
        fn_: &Function<'a>,
        mut out: &mut TokenStream,
    ) {
        imports.push_origin(self.origin(), self.name());
        imports.push("crate::Unique");
        imports.push("crate::AsRaw");

        let handle_ident = self.as_ident();

        let mut function_code = TokenStream::default();
        fn_.generate_code(source, imports, doc, self, &mut function_code);

        quote_each_token! {
            out

            impl #handle_ident {
                #function_code
            }
        }
    }
}
