use crate::native::vulkan1_0::{BaseOutStructure, DeviceSize, StructureType, MAX_MEMORY_HEAPS};
#[doc(alias = "VkPhysicalDeviceMemoryBudgetPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryBudgetPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "heapBudget")]
    pub heap_budget: [DeviceSize; MAX_MEMORY_HEAPS as usize],
    #[doc(alias = "heapUsage")]
    pub heap_usage: [DeviceSize; MAX_MEMORY_HEAPS as usize],
}
impl Default for PhysicalDeviceMemoryBudgetPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMemoryBudgetPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            heap_budget: unsafe { std::mem::zeroed() },
            heap_usage: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_memory_budget::{
    EXT_MEMORY_BUDGET_EXTENSION_NAME, EXT_MEMORY_BUDGET_SPEC_VERSION,
};
