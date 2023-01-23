//!# [VK_EXT_memory_budget](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_memory_budget.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_memory_budget/VK_EXT_memory_budget.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, DeviceSize, StructureType, MAX_MEMORY_HEAPS},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceMemoryBudgetPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMemoryBudgetPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_memory_budget/VkPhysicalDeviceMemoryBudgetPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "heapBudget")]
    heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    #[doc(alias = "heapUsage")]
    heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
}
#[doc(alias = "VK_EXT_MEMORY_BUDGET_SPEC_VERSION")]
pub const EXT_MEMORY_BUDGET_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_MEMORY_BUDGET_EXTENSION_NAME")]
pub const EXT_MEMORY_BUDGET_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_memory_budget");
