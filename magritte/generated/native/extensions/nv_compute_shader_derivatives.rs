use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "computeDerivativeGroupQuads")]
    pub compute_derivative_group_quads: Bool32,
    #[doc(alias = "computeDerivativeGroupLinear")]
    pub compute_derivative_group_linear: Bool32,
}
impl Default for PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceComputeShaderDerivativesFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            compute_derivative_group_quads: unsafe { std::mem::zeroed() },
            compute_derivative_group_linear: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_compute_shader_derivatives::{
    NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME, NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION,
};
