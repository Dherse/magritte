use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSMCount")]
    shader_sm_count: u32,
    #[doc(alias = "shaderWarpsPerSM")]
    shader_warps_per_sm: u32,
}
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSMBuiltins")]
    shader_sm_builtins: Bool32,
}
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_SPEC_VERSION")]
pub const NV_SHADER_SM_BUILTINS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_SHADER_SM_BUILTINS_EXTENSION_NAME")]
pub const NV_SHADER_SM_BUILTINS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_shader_sm_builtins");
