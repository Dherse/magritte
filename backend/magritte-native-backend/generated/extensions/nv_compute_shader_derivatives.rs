use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceComputeShaderDerivativesFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceComputeShaderDerivativesFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "computeDerivativeGroupQuads")]
    compute_derivative_group_quads: Bool32,
    #[doc(alias = "computeDerivativeGroupLinear")]
    compute_derivative_group_linear: Bool32,
}
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME")]
pub const NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_compute_shader_derivatives");
