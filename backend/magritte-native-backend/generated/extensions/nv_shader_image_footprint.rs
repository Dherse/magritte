use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceShaderImageFootprintFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceShaderImageFootprintFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "imageFootprint")]
    image_footprint: Bool32,
}
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION")]
pub const NV_SHADER_IMAGE_FOOTPRINT_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME")]
pub const NV_SHADER_IMAGE_FOOTPRINT_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_shader_image_footprint");
