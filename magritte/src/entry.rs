use std::ffi::CString;

use crate::{
    native::{
        self,
        vulkan1_0::{
            FNCreateInstance, FNEnumerateInstanceExtensionProperties, FNEnumerateInstanceLayerProperties,
            FNGetInstanceProcAddr, StructureType,
        },
        vulkan1_1::FNEnumerateInstanceVersion,
    },
    vulkan1_0::{Instance, InstanceCreateInfo},
};

pub struct Entry {
    vtable: EntryVTable,

    #[cfg(feature = "dynamic")]
    library: Option<libloading::Library>,
}

unsafe impl Send for Entry {}
unsafe impl Sync for Entry {}

impl Entry {
    /// Creates a new entry from its V-table.
    pub fn from_vtable(vtable: EntryVTable) -> Self {
        Self {
            vtable,

            #[cfg(feature = "dynamic")]
            library: None,
        }
    }

    /// Gets the V-Table
    #[inline(always)]
    pub fn vtable(&self) -> &EntryVTable {
        &self.vtable
    }

    pub fn create_instance(
        &self,
        create_info: &InstanceCreateInfo,
        layers: Vec<String>,
        extensions: Vec<String>,
    ) -> anyhow::Result<Instance> {
        let mut instance = native::vulkan1_0::Instance::null();

        let fn_ = self.vtable().create_instance();

        let application_info = create_info.application_info().map_or_else::<anyhow::Result<_>, _, _>(
            || {
                Ok(native::vulkan1_0::ApplicationInfo {
                    s_type: StructureType::ApplicationInfo,
                    p_next: std::ptr::null(),
                    application_name: std::ptr::null(),
                    application_version: 0,
                    engine_name: std::ptr::null(),
                    engine_version: 0,
                    api_version: todo!(),
                })
            },
            |info| {
                let application_name = CString::new(info.application_name().as_deref().unwrap_or(""))?;
                let application_version = info.application_version();

                let engine_name = CString::new(info.engine_name().as_deref().unwrap_or(""))?;
                let engine_version = info.engine_version();

                let api_version = info.api_version();

                Ok(native::vulkan1_0::ApplicationInfo {
                    s_type: StructureType::ApplicationInfo,
                    p_next: std::ptr::null(),
                    application_name: application_name.as_ptr(),
                    application_version: application_version,
                    engine_name: engine_name.as_ptr(),
                    engine_version: engine_version,
                    api_version: api_version,
                })
            },
        )?;

        let layers = layers
            .into_iter()
            .map(|layer| CString::new(layer))
            .collect::<Result<Vec<_>, _>>()?;

        let layer_ptrs = layers.iter().map(|layer| layer.as_ptr()).collect::<Vec<_>>();

        let extensions = extensions
            .into_iter()
            .map(|ext| CString::new(ext))
            .collect::<Result<Vec<_>, _>>()?;

        let extension_ptrs = layers.iter().map(|ext| ext.as_ptr()).collect::<Vec<_>>();

        let mut raw_create_info = native::vulkan1_0::InstanceCreateInfo {
            s_type: StructureType::InstanceCreateInfo,
            p_next: std::ptr::null(),
            flags: create_info.flags(),
            application_info: &application_info,
            enabled_layer_count: layers.len() as _,
            pp_enabled_layer_names: layer_ptrs.as_ptr(),
            enabled_extension_count: extensions.len() as _,
            pp_enabled_extension_names: extension_ptrs.as_ptr(),
        };

        let next = &mut raw_create_info.p_next;
        for ext in create_info.extensions() {
            /*match ext {
                #[cfg(feature = "VK_EXT_debug_report")]
                ///Contains a type [`DebugReportCallbackCreateInfoEXT`] for extending [`InstanceCreateInfo`]
                DebugReportCallbackCreateInfoEXT(DebugReportCallbackCreateInfoEXT),
                #[cfg(feature = "VK_EXT_validation_flags")]
                ///Contains a type [`ValidationFlagsEXT`] for extending [`InstanceCreateInfo`]
                ValidationFlagsEXT(ValidationFlagsEXT),
                #[cfg(feature = "VK_EXT_validation_features")]
                ///Contains a type [`ValidationFeaturesEXT`] for extending [`InstanceCreateInfo`]
                ValidationFeaturesEXT(ValidationFeaturesEXT),
                #[cfg(feature = "VK_EXT_debug_utils")]
                ///Contains a type [`DebugUtilsMessengerCreateInfoEXT`] for extending [`InstanceCreateInfo`]
                DebugUtilsMessengerCreateInfoEXT(DebugUtilsMessengerCreateInfoEXT),
                _ => panic!("Unsupported element in pointer chain")
            }*/
        }

        let result = unsafe { fn_(&raw_create_info, std::ptr::null(), &mut instance) };

        todo!()
    }
}

/// The V-Table of functions loaded dynamically or statically before an instance is created
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
