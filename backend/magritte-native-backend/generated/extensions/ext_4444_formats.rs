//!# [VK_EXT_4444_formats](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_4444_formats.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_4444_formats/VK_EXT_4444_formats.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDevice4444FormatsFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDevice4444FormatsFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_4444_formats/VkPhysicalDevice4444FormatsFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDevice4444FormatsFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevice4444FormatsFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "formatA4R4G4B4")]
    format_a4r4g4b4: Bool32,
    #[doc(alias = "formatA4B4G4R4")]
    format_a4b4g4r4: Bool32,
}
#[doc(alias = "VK_EXT_4444_FORMATS_SPEC_VERSION")]
pub const EXT_4444_FORMATS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_4444_FORMATS_EXTENSION_NAME")]
pub const EXT_4444_FORMATS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_4444_formats");
