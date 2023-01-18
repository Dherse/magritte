use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, ImageViewType, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceImageViewImageFormatInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceImageViewImageFormatInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "imageViewType")]
    image_view_type: ImageViewType,
}
#[doc(alias = "VkFilterCubicImageViewImageFormatPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FilterCubicImageViewImageFormatPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "filterCubic")]
    filter_cubic: Bool32,
    #[doc(alias = "filterCubicMinmax")]
    filter_cubic_minmax: Bool32,
}
#[doc(alias = "VK_EXT_FILTER_CUBIC_SPEC_VERSION")]
pub const EXT_FILTER_CUBIC_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_EXT_FILTER_CUBIC_EXTENSION_NAME")]
pub const EXT_FILTER_CUBIC_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_filter_cubic");
