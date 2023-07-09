use crate::native::vulkan1_0::{BaseOutStructure, Bool32, Device, DeviceMemory, StructureType};
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pageableDeviceLocalMemory")]
    pub pageable_device_local_memory: Bool32,
}
impl Default for PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePageableDeviceLocalMemoryFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            pageable_device_local_memory: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_pageable_device_local_memory::{
    EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME, EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION,
};
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
pub type FNSetDeviceMemoryPriorityExt = unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
