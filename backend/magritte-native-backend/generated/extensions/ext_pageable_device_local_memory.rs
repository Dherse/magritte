//!# [VK_EXT_pageable_device_local_memory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_pageable_device_local_memory.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_pageable_device_local_memory/VK_EXT_pageable_device_local_memory.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, Device, DeviceMemory, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_pageable_device_local_memory/VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "pageableDeviceLocalMemory")]
    pageable_device_local_memory: Bool32,
}
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME")]
pub const EXT_PAGEABLE_DEVICE_LOCAL_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_pageable_device_local_memory");
///# [vkSetDeviceMemoryPriorityEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkSetDeviceMemoryPriorityEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_pageable_device_local_memory/vkSetDeviceMemoryPriorityEXT.md")]
#[doc(alias = "vkSetDeviceMemoryPriorityEXT")]
pub type FNSetDeviceMemoryPriorityExt = unsafe extern "system" fn(device: Device, memory: DeviceMemory, priority: f32);
