use crate::native::vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceMemoryPriorityFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMemoryPriorityFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryPriority")]
    pub memory_priority: Bool32,
}
impl Default for PhysicalDeviceMemoryPriorityFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceMemoryPriorityFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            memory_priority: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryPriorityAllocateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryPriorityAllocateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub priority: f32,
}
impl Default for MemoryPriorityAllocateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryPriorityAllocateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            priority: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_memory_priority::{
    EXT_MEMORY_PRIORITY_EXTENSION_NAME, EXT_MEMORY_PRIORITY_SPEC_VERSION,
};
