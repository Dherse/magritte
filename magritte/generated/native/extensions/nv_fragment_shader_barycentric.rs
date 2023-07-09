use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShaderBarycentric")]
    pub fragment_shader_barycentric: Bool32,
}
impl Default for PhysicalDeviceFragmentShaderBarycentricFeaturesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShaderBarycentricFeaturesNv,
            p_next: unsafe { std::mem::zeroed() },
            fragment_shader_barycentric: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_fragment_shader_barycentric::{
    NV_FRAGMENT_SHADER_BARYCENTRIC_EXTENSION_NAME, NV_FRAGMENT_SHADER_BARYCENTRIC_SPEC_VERSION,
};
