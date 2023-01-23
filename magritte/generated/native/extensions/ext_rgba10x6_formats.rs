//!# [VK_EXT_rgba10x6_formats](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_rgba10x6_formats.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_rgba10x6_formats/VK_EXT_rgba10x6_formats.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_rgba10x6_formats/VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceRgba10x6FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "formatRgba10x6WithoutYCbCrSampler")]
    format_rgba10x6_without_y_cb_cr_sampler: Bool32,
}
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_SPEC_VERSION")]
pub const EXT_RGBA10X6_FORMATS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_RGBA10X6_FORMATS_EXTENSION_NAME")]
pub const EXT_RGBA10X6_FORMATS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_rgba10x6_formats");
