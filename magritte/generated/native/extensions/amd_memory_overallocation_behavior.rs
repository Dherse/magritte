use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkDeviceMemoryOverallocationCreateInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceMemoryOverallocationCreateInfoAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "overallocationBehavior")]
    pub overallocation_behavior: MemoryOverallocationBehaviorAMD,
}
impl Default for DeviceMemoryOverallocationCreateInfoAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceMemoryOverallocationCreateInfoAmd,
            p_next: unsafe { std::mem::zeroed() },
            overallocation_behavior: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_memory_overallocation_behavior::{
    MemoryOverallocationBehaviorAMD, AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME,
    AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION,
};
