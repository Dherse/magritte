use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceCornerSampledImageFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCornerSampledImageFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "cornerSampledImage")]
    corner_sampled_image: Bool32,
}
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION")]
pub const NV_CORNER_SAMPLED_IMAGE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME")]
pub const NV_CORNER_SAMPLED_IMAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_corner_sampled_image");
