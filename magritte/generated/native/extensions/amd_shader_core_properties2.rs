use crate::native::vulkan1_0::{BaseOutStructure, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderCoreProperties2AMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderCoreProperties2AMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderCoreFeatures")]
    pub shader_core_features: ShaderCorePropertiesFlagsAMD,
    #[doc(alias = "activeComputeUnitCount")]
    pub active_compute_unit_count: u32,
}
impl Default for PhysicalDeviceShaderCoreProperties2AMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderCoreProperties2Amd,
            p_next: unsafe { std::mem::zeroed() },
            shader_core_features: unsafe { std::mem::zeroed() },
            active_compute_unit_count: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::amd_shader_core_properties2::{
    ShaderCorePropertiesFlagBitsAMD, ShaderCorePropertiesFlagsAMD, AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME,
    AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION,
};
