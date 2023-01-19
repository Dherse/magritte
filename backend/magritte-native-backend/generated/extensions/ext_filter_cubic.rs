//!# [VK_EXT_filter_cubic](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_filter_cubic.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_filter_cubic/VK_EXT_filter_cubic.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, ImageViewType, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceImageViewImageFormatInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceImageViewImageFormatInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_filter_cubic/VkPhysicalDeviceImageViewImageFormatInfoEXT.md")]
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
///# [VkFilterCubicImageViewImageFormatPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFilterCubicImageViewImageFormatPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_filter_cubic/VkFilterCubicImageViewImageFormatPropertiesEXT.md")]
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
