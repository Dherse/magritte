use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "ycbcr2plane444Formats")]
    ycbcr2plane444_formats: Bool32,
}
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME")]
pub const EXT_YCBCR_2PLANE_444_FORMATS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_ycbcr_2plane_444_formats");
