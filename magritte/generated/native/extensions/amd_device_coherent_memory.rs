use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceCoherentMemoryFeaturesAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCoherentMemoryFeaturesAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "deviceCoherentMemory")]
    pub device_coherent_memory: Bool32,
}
impl Default for PhysicalDeviceCoherentMemoryFeaturesAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceCoherentMemoryFeaturesAmd,
            p_next: unsafe { std::mem::zeroed() },
            device_coherent_memory: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_device_coherent_memory::{
    AMD_DEVICE_COHERENT_MEMORY_EXTENSION_NAME, AMD_DEVICE_COHERENT_MEMORY_SPEC_VERSION,
};
