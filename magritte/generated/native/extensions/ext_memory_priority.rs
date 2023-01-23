//!# [VK_EXT_memory_priority](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_priority.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_memory_priority/VK_EXT_memory_priority.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceMemoryPriorityFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryPriorityFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_memory_priority/VkPhysicalDeviceMemoryPriorityFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryPriority")]
    memory_priority: Bool32,
}
///# [VkMemoryPriorityAllocateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryPriorityAllocateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_memory_priority/VkMemoryPriorityAllocateInfoEXT.md")]
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    priority: f32,
}
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_SPEC_VERSION")]
pub const EXT_MEMORY_PRIORITY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_MEMORY_PRIORITY_EXTENSION_NAME")]
pub const EXT_MEMORY_PRIORITY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_memory_priority");
