use crate::native::vulkan1_0::{BaseOutStructure, Bool32, StructureType};
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShaderSampleInterlock")]
    pub fragment_shader_sample_interlock: Bool32,
    #[doc(alias = "fragmentShaderPixelInterlock")]
    pub fragment_shader_pixel_interlock: Bool32,
    #[doc(alias = "fragmentShaderShadingRateInterlock")]
    pub fragment_shader_shading_rate_interlock: Bool32,
}
impl Default for PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShaderInterlockFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            fragment_shader_sample_interlock: unsafe { std::mem::zeroed() },
            fragment_shader_pixel_interlock: unsafe { std::mem::zeroed() },
            fragment_shader_shading_rate_interlock: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_fragment_shader_interlock::{
    EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME, EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION,
};
