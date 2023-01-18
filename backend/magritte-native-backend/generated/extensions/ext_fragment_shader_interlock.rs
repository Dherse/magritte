use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "fragmentShaderSampleInterlock")]
    fragment_shader_sample_interlock: Bool32,
    #[doc(alias = "fragmentShaderPixelInterlock")]
    fragment_shader_pixel_interlock: Bool32,
    #[doc(alias = "fragmentShaderShadingRateInterlock")]
    fragment_shader_shading_rate_interlock: Bool32,
}
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME")]
pub const EXT_FRAGMENT_SHADER_INTERLOCK_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_fragment_shader_interlock");
