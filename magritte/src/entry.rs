//! # Entry
//! The entry is the entry-point into Vulkan, it contains the most basic functions
//! required to load all other Vulkan functions. It can optionally be used to dynamically
//! load Vulkan.

use std::ffi::CStr;

use crate::{
    vulkan1_0::{
        AllocationCallbacks, ExtensionProperties, FNCreateInstance, FNEnumerateInstanceExtensionProperties,
        FNEnumerateInstanceLayerProperties, FNGetInstanceProcAddr, Instance, InstanceCreateInfo, LayerProperties,
        PFNVoidFunction, VulkanResultCodes,
    },
    vulkan1_1::FNEnumerateInstanceVersion,
    Extensions, Unique, Version,
};

#[derive(Clone, Copy, Default)]
pub struct Entry(pub EntryVTable);

impl Entry {
    /// Gets the V-Table
    #[inline(always)]
    pub fn vtable(&self) -> &EntryVTable {
        &self.0
    }

    /// Creates a Vulkan instance.
    /// See [`FNCreateInstance`] for more information.
    pub unsafe fn create_instance<'lt>(
        &self,
        instance_create_info: &InstanceCreateInfo<'lt>,
        allocation_callback: Option<&AllocationCallbacks<'lt>>,
        extensions: Extensions,
    ) -> Result<Unique<'_, Instance>, VulkanResultCodes> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let fn_ = self.vtable().create_instance().unwrap();

        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let fn_ = self.vtable().create_instance().unwrap_unchecked();

        let mut instance = Instance::default();

        match fn_(
            instance_create_info,
            allocation_callback
                .map(|v| v as *const _)
                .unwrap_or_else(std::ptr::null),
            &mut instance,
        ) {
            VulkanResultCodes::SUCCESS => Ok(Unique::new(self, instance, extensions)),
            other => Err(other),
        }
    }

    /// Gets the function pointer from an instance and a given name.
    /// See [`FNGetInstanceProcAddr`] for more information.
    pub unsafe fn get_instance_proc_addr<'lt>(&self, instance: Instance, name: &CStr) -> PFNVoidFunction {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let fn_ = self.vtable().get_instance_proc_addr().unwrap();

        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let fn_ = self.vtable().get_instance_proc_addr().unwrap_unchecked();

        fn_(instance, name.as_ptr())
    }

    /// Gets the maximum supported Vulkan version.
    /// If the function is not available in the current Vulkan entry, will default to returning
    /// [`Version::VULKAN1_0`]. See [`FNEnumerateInstanceVersion`] for more information.
    #[inline]
    #[track_caller]
    #[doc(alias = "vkEnumerateInstanceVersion")]
    pub unsafe fn enumerate_instance_version<'lt>(&self) -> Result<Version, VulkanResultCodes> {
        if let Some(fn_) = self.vtable().enumerate_instance_version() {
            let mut out: u32 = 0;

            match fn_(&mut out) {
                VulkanResultCodes::SUCCESS => Ok(Version::from(out)),
                other => Err(other),
            }
        } else {
            Ok(Version::VULKAN1_0)
        }
    }

    /// Gets the list of instance layers and their properties.
    /// `count` provides the maximum number of layers you want.
    /// See [`FNEnumerateInstanceLayerProperties`] for more information.
    pub unsafe fn enumerate_instance_layer_properties<'lt>(
        &self,
        count: Option<u32>,
    ) -> Result<Vec<LayerProperties>, VulkanResultCodes> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let fn_ = self.vtable().enumerate_instance_layer_properties().unwrap();

        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let fn_ = self.vtable().enumerate_instance_layer_properties().unwrap_unchecked();

        let mut len = if let Some(count) = count {
            count
        } else {
            let mut len = 0;
            match fn_(&mut len, std::ptr::null_mut()) {
                VulkanResultCodes::SUCCESS => len,
                other => return Err(other),
            }
        };

        let mut out = vec![Default::default(); len as usize];

        match fn_(&mut len, out.as_mut_ptr()) {
            VulkanResultCodes::SUCCESS => {
                out.truncate(len as usize);
                Ok(out)
            },
            other => Err(other),
        }
    }

    /// Gets the list of instance extensions and their properties.
    /// `count` provides the maximum number of extensions you want.
    /// If `layer_name` is specified, will only return the extensions added by that layer.
    /// See [`FNEnumerateInstanceExtensionProperties`] for more information.
    pub unsafe fn enumerate_instance_extension_properties<'lt>(
        &self,
        count: Option<u32>,
        layer_name: Option<&CStr>,
    ) -> Result<Vec<ExtensionProperties>, VulkanResultCodes> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let fn_ = self.vtable().enumerate_instance_extension_properties().unwrap();

        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let fn_ = self
            .vtable()
            .enumerate_instance_extension_properties()
            .unwrap_unchecked();

        let mut len = if let Some(count) = count {
            count
        } else {
            let mut len = 0;
            match fn_(
                layer_name.map(CStr::as_ptr).unwrap_or_else(std::ptr::null),
                &mut len,
                std::ptr::null_mut(),
            ) {
                VulkanResultCodes::SUCCESS => len,
                other => return Err(other),
            }
        };

        let mut out = vec![Default::default(); len as usize];

        match fn_(
            layer_name.map(CStr::as_ptr).unwrap_or_else(std::ptr::null),
            &mut len,
            out.as_mut_ptr(),
        ) {
            VulkanResultCodes::SUCCESS => {
                out.truncate(len as usize);
                Ok(out)
            },
            other => Err(other),
        }
    }
}

/// The V-Table of functions loaded dynamically or statically before an instance is created
#[derive(Clone, Copy, Default)]
pub struct EntryVTable {
    /// the function `vkCreateInstance`.
    pub create_instance: FNCreateInstance,

    /// the function `vkGetInstanceProcAddr`.
    pub get_instance_proc_addr: FNGetInstanceProcAddr,

    /// the function `vkEnumerateInstanceVersion`.
    pub enumerate_instance_version: FNEnumerateInstanceVersion,

    /// the function `vkEnumerateInstanceLayerProperties`.
    pub enumerate_instance_layer_properties: FNEnumerateInstanceLayerProperties,

    /// the function `vkEnumerateInstanceExtensionProperties`.
    pub enumerate_instance_extension_properties: FNEnumerateInstanceExtensionProperties,
}

impl EntryVTable {
    /// Gets the function `vkCreateInstance`.
    /// See [`FNCreateInstance`] for more information.
    #[inline(always)]
    pub const fn create_instance(&self) -> FNCreateInstance {
        self.create_instance
    }

    /// Gets the function `vkGetInstanceProcAddr`.
    /// See [`FNGetInstanceProcAddr`] for more information.
    #[inline(always)]
    pub const fn get_instance_proc_addr(&self) -> FNGetInstanceProcAddr {
        self.get_instance_proc_addr
    }

    /// Gets the function `vkEnumerateInstanceVersion`.
    /// See [`FNEnumerateInstanceVersion`] for more information.
    #[inline(always)]
    pub const fn enumerate_instance_version(&self) -> FNEnumerateInstanceVersion {
        self.enumerate_instance_version
    }

    /// Gets the function `vkEnumerateInstanceLayerProperties`.
    /// See [`FNEnumerateInstanceLayerProperties`] for more information.
    #[inline(always)]
    pub const fn enumerate_instance_layer_properties(&self) -> FNEnumerateInstanceLayerProperties {
        self.enumerate_instance_layer_properties
    }

    /// Gets the function `vkEnumerateInstanceExtensionProperties`.
    /// See [`FNEnumerateInstanceExtensionProperties`] for more information.
    #[inline(always)]
    pub const fn enumerate_instance_extension_properties(&self) -> FNEnumerateInstanceExtensionProperties {
        self.enumerate_instance_extension_properties
    }
}
