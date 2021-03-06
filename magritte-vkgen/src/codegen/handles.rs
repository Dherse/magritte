pub mod loader;

use heck::{ToShoutySnakeCase, ToSnakeCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_each_token, ToTokens};
use tracing::warn;

use crate::{
    codegen::alias_of,
    doc::Documentation,
    imports::Imports,
    source::{Alias, Bit, Function, Handle, Source},
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

        imports.push("crate::Handle");

        let parent = if let Some(parent) = self.parent() {
            let handle = source.handles.get_by_name(parent).expect("unknown parent");

            imports.push_origin(handle.origin(), handle.name());
            imports.push("crate::Unique");

            let ident = handle.as_ident();

            quote! {
                Unique<#ident>
            }
        } else {
            imports.push("crate::entry::Entry");
            imports.push("std::sync::Arc");
            quote! { Arc<Entry> }
        };

        let mut ancestors = Vec::new();

        let ancestor_fn = |count, name| {
            let doc_str = format!("Gets the reference to the [`{}`]", name);
            let ident = Ident::new(name, Span::call_site());
            let func_name = Ident::new(&name.to_snake_case(), Span::call_site());

            let origin = source.handles.get_by_either(name).unwrap().origin();

            imports.push_origin(origin, name);

            let ancestors = (0..count).into_iter().map(|_| quote! { parent });

            quote! {
                #[doc = #doc_str]
                #[inline]
                pub fn #func_name(&self) -> &Unique<#ident> {
                    self #(
                        .#ancestors()
                    )*
                }
            }
        };

        let entry_count = self.find_entry_ancestors(source);
        let entry_ancestors = (0..entry_count).into_iter().map(|_| quote! { parent });
        imports.push("crate::entry::Entry");
        imports.push("std::sync::Arc");
        ancestors.push(quote! {
            #[doc = "Gets the reference to the [`Entry`]"]
            #[inline]
            pub fn entry(&self) -> &Arc<Entry> {
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

        let metadata = if self.original_name() == "VkInstance" {
            imports.push("crate::extensions::InstanceExtensions");
            quote! { InstanceExtensions }
        } else if self.original_name() == "VkDevice" {
            imports.push("crate::extensions::DeviceExtensions");
            quote! { DeviceExtensions }
        } else {
            imports.push("std::sync::atomic::AtomicBool");
            quote! { AtomicBool }
        };

        let load_vtable = if self.is_loader() {
            let ident = self.this_vtable_ident();

            let loader_fn = match self.name() {
                "Device" => quote! { parent.instance().vtable().vulkan1_0().get_device_proc_addr().unwrap() },
                "Instance" => quote! { parent.vtable().get_instance_proc_addr().unwrap() },
                _ => unreachable!(),
            };

            let extensions = match self.name() {
                "Device" => quote! { &parent.instance().metadata(), metadata },
                "Instance" => quote! { &metadata, &Default::default() },
                _ => unreachable!(),
            };

            Some(quote! {
                #ident::load(
                    #loader_fn,
                    *self,
                    #extensions,
                )
            })
        } else {
            None
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
                                if !self.metadata().load(Ordering::Acquire) {
                                    self.parent().#destroyer(
                                        self.as_raw().coerce()
                                    );
                                }
                            }
                        } else if self.original_name() == "VkPerformanceConfigurationINTEL" {
                            quote! {
                                if !self.metadata().load(Ordering::Acquire) {
                                    self.device().#destroyer(
                                        Some(self.as_raw().coerce())
                                    );
                                }
                            }
                        } else if self.original_name() == "VkDevice" {
                            quote! {
                                self.#destroyer(
                                    None
                                );
                            }
                        } else {
                            quote! {
                                if !self.metadata().load(Ordering::Acquire) {
                                    self.#destroyer(
                                        None
                                    );
                                }
                            }
                        }
                    },
                    3 => {
                        if [
                            "VkCuModuleNVX",
                            "VkCuFunctionNVX",
                            "VkVideoSessionParametersKHR",
                            "VkVideoSessionKHR",
                            "VkBufferCollectionFUCHSIA",
                        ]
                        .contains(&self.original_name())
                        {
                            quote! {
                                if !self.metadata().load(Ordering::Acquire) {
                                    self.#owner_getter().#destroyer(
                                        self.as_raw().coerce(),
                                        None
                                    );
                                }
                            }
                        } else {
                            quote! {
                                if !self.metadata().load(Ordering::Acquire) {
                                    self.#owner_getter().#destroyer(
                                        Some(self.as_raw().coerce()),
                                        None
                                    );
                                }
                            }
                        }
                    },
                    4 => {
                        quote! {
                            if !self.metadata().load(Ordering::Acquire) {
                                self.#owner_getter().#destroyer(
                                    self.parent().as_raw(),
                                    &[
                                        self.as_raw().coerce()
                                    ],
                                );
                            }
                        }
                    },
                    len => unreachable!("unsupported number of arguments: {}", len),
                }
            });

        let parent_ident = if load_vtable.is_some() {
            quote! { parent }
        } else {
            quote! { _ }
        };

        let metadata_ident =
            if load_vtable.is_some() && (self.original_name() == "VkInstance" || self.original_name() == "VkDevice") {
                quote! { metadata }
            } else {
                quote! { _ }
            };

        if self.original_name() != "VkInstance" && self.original_name() != "VkDevice" {
            imports.push("std::sync::atomic::Ordering");

            ancestors.push(quote! {
                #[doc = "Disables the base dropping behaviour of this handle"]
                #[inline]
                pub fn disable_drop(&self) {
                    self.metadata().store(true, Ordering::Relaxed);
                }
            });
        } else if self.original_name() == "VkInstance" {
            imports.push("crate::extensions::DeviceExtensions");
            let ident = self.this_vtable_ident();

            ancestors.push(quote! {
                #[doc = "Update the vtable with the list of device extensions"]
                #[doc(hidden)]
                #[inline]
                pub unsafe fn update_vtable(&self, device_extensions: &DeviceExtensions) {
                    let this = self.this;
                    let inner = self.inner_mut_unchecked();
                    inner.vtable = #ident::load(
                        inner.parent.vtable().get_instance_proc_addr().unwrap(),
                        this,
                        &inner.metadata,
                        device_extensions
                    );
                }
            });
        }

        quote_each_token! {
            out

            #[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
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
            }

            unsafe impl Send for #name {}

            impl Default for #name {
                fn default() -> Self {
                    Self::null()
                }
            }

            impl Handle for #name {
                type Parent = #parent;

                type VTable = #vtable;

                type Metadata = #metadata;

                type Storage = #ty;

                #[inline]
                fn as_stored(self) -> Self::Storage {
                    self.0
                }

                #[inline]
                unsafe fn from_stored(this: Self::Storage) -> Self {
                    Self(this)
                }

                #[inline]
                #[track_caller]
                unsafe fn destroy(self: &mut Unique<Self>) {
                    #destroy
                }

                #[inline]
                unsafe fn load_vtable(&self, #parent_ident: &Self::Parent, #metadata_ident: &Self::Metadata) -> Self::VTable {
                    #load_vtable
                }
            }

            impl Unique<#name> {
                #(#ancestors)*
            }
        }

        self.generate_debug_marker_code(source, out);
    }

    fn generate_debug_marker_code(&self, source: &Source<'a>, mut out: &mut TokenStream) {
        if self.find_device_ancestors(source).is_none() {
            return;
        }

        let shouty_name = if self.rename.is_some() {
            source
                .handles
                .get_by_name(&self.original_name)
                .expect("unknown alias")
                .name()
                .to_shouty_snake_case()
        } else {
            self.name().to_shouty_snake_case()
        };

        let enum_ = source.enums.get_by_name("VkObjectType").expect("VkObjectType missing");

        if let Some(variant) = enum_
            .variants()
            .get_by_name(&format!("VK_OBJECT_TYPE_{}", shouty_name))
            .map(Bit::as_ident)
            .or_else(|| {
                enum_
                    .aliases()
                    .get_by_name(&format!("VK_OBJECT_TYPE_{}", shouty_name))
                    .map(Alias::as_ident)
            })
        {
            let name = self.as_ident();

            let device_access = (self.original_name() != "VkDevice").then(|| quote! { .device() });

            quote_each_token! {
                out

                #[cfg(feature = "VK_EXT_debug_utils")]
                impl #name {
                    #[doc = "Give a user-friendly name to an object"]
                    pub fn set_name(self: &Unique<Self>, name: &std::ffi::CStr) {
                        assert!(self.strong_count() == 1, "`set_name` requires that the object be synchronized");
                        if !self #device_access .instance().metadata().ext_debug_utils() {
                            return;
                        }

                        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectNameInfoEXT::default()
                            .with_object_type(crate::generated::vulkan1_0::ObjectType::#variant)
                            .with_object_handle(self.as_stored() as u64)
                            .with_object_name(name.as_ptr());

                        unsafe {
                            self #device_access .set_debug_utils_object_name_ext(&info).unwrap();
                        }
                    }

                    #[doc = "Attach arbitrary data to an object"]
                    #[doc = "In addition to setting a name for an object, debugging and validation layers may have uses for additional"]
                    #[doc = "binary data on a per-object basis that has no other place in the Vulkan API. For example, a VkShaderModule"]
                    #[doc = "could have additional debugging data attached to it to aid in offline shader tracing."]
                    pub fn set_tag(self: &Unique<Self>, tag: u64, data: &[u8]) {
                        assert!(self.strong_count() == 1, "`set_name` requires that the object be synchronized");
                        if !self #device_access .instance().metadata().ext_debug_utils() {
                            return;
                        }

                        let info = crate::generated::extensions::ext_debug_utils::DebugUtilsObjectTagInfoEXT::default()
                            .with_object_type(crate::generated::vulkan1_0::ObjectType::#variant)
                            .with_object_handle(self.as_stored() as u64)
                            .with_tag_name(tag)
                            .with_tag_size(data.len() as _)
                            .with_tag_raw(data.as_ptr().cast());

                        unsafe {
                            self #device_access .set_debug_utils_object_tag_ext(&info).unwrap();
                        }
                    }
                }
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
        imports.push("crate::Unique");
        imports.push("crate::AsRaw");

        let mut handle_ident = self.as_ident();

        let mut function_code = TokenStream::default();

        // this is a dirty hack, but it really makes my life a lot easier, sorry
        // TODO: improve this
        if let Some(change) = fn_.generate_code(source, imports, doc, self, &mut function_code) {
            let handle = source.handles.get_by_name(&change).unwrap();
            handle_ident = handle.as_ident();

            imports.push_origin(handle.origin(), handle.name());
        } else {
            imports.push_origin(self.origin(), self.name());
        }

        quote_each_token! {
            out

            impl #handle_ident {
                #function_code
            }
        }
    }
}
