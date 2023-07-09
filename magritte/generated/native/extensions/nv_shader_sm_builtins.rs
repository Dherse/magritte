use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSMCount")]
    pub shader_sm_count: u32,
    #[doc(alias = "shaderWarpsPerSM")]
    pub shader_warps_per_sm: u32,
}
impl Default for PhysicalDeviceShaderSmBuiltinsPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderSmBuiltinsPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            shader_sm_count: unsafe { std::mem::zeroed() },
            shader_warps_per_sm: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceShaderSMBuiltinsFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "shaderSMBuiltins")]
    pub shader_sm_builtins: Bool32,
}
impl Default for PhysicalDeviceShaderSmBuiltinsFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceShaderSmBuiltinsFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            shader_sm_builtins: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_shader_sm_builtins::{
    NV_SHADER_SM_BUILTINS_EXTENSION_NAME, NV_SHADER_SM_BUILTINS_SPEC_VERSION,
};
