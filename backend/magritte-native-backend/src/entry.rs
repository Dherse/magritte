use crate::{vulkan1_0::{FNCreateInstance, FNGetInstanceProcAddr, FNEnumerateInstanceLayerProperties, FNEnumerateInstanceExtensionProperties}, vulkan1_1::FNEnumerateInstanceVersion};

#[derive(Clone, Copy)]
pub struct EntryVTable {
    /// the function `vkCreateInstance`.
    pub create_instance: FNCreateInstance,

    /// the function `vkGetInstanceProcAddr`.
    pub get_instance_proc_addr: FNGetInstanceProcAddr,

    /// the function `vkEnumerateInstanceVersion`.
    pub enumerate_instance_version: Option<FNEnumerateInstanceVersion>,

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
    pub const fn enumerate_instance_version(&self) -> Option<FNEnumerateInstanceVersion> {
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