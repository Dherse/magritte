//!# [VK_EXT_astc_decode_mode](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_astc_decode_mode.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_astc_decode_mode/VK_EXT_astc_decode_mode.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, Format, StructureType},
};
use std::ffi::CStr;
///# [VkImageViewASTCDecodeModeEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImageViewASTCDecodeModeEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_astc_decode_mode/VkImageViewASTCDecodeModeEXT.md")]
#[doc(alias = "VkImageViewASTCDecodeModeEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImageViewAstcDecodeModeEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "decodeMode")]
    decode_mode: Format,
}
///# [VkPhysicalDeviceASTCDecodeFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceASTCDecodeFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_astc_decode_mode/VkPhysicalDeviceASTCDecodeFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceASTCDecodeFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceAstcDecodeFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "decodeModeSharedExponent")]
    decode_mode_shared_exponent: Bool32,
}
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_SPEC_VERSION")]
pub const EXT_ASTC_DECODE_MODE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_ASTC_DECODE_MODE_EXTENSION_NAME")]
pub const EXT_ASTC_DECODE_MODE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_astc_decode_mode");
