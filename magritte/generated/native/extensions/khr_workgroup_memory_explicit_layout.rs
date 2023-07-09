use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "workgroupMemoryExplicitLayout")]
    pub workgroup_memory_explicit_layout: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayoutScalarBlockLayout")]
    pub workgroup_memory_explicit_layout_scalar_block_layout: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayout8BitAccess")]
    pub workgroup_memory_explicit_layout8_bit_access: Bool32,
    #[doc(alias = "workgroupMemoryExplicitLayout16BitAccess")]
    pub workgroup_memory_explicit_layout16_bit_access: Bool32,
}
impl Default for PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            workgroup_memory_explicit_layout: unsafe { std::mem::zeroed() },
            workgroup_memory_explicit_layout_scalar_block_layout: unsafe { std::mem::zeroed() },
            workgroup_memory_explicit_layout8_bit_access: unsafe { std::mem::zeroed() },
            workgroup_memory_explicit_layout16_bit_access: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_workgroup_memory_explicit_layout::{
    KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_EXTENSION_NAME, KHR_WORKGROUP_MEMORY_EXPLICIT_LAYOUT_SPEC_VERSION,
};
