//!# [VK_EXT_border_color_swizzle](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_border_color_swizzle.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_border_color_swizzle/VK_EXT_border_color_swizzle.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ComponentMapping, StructureType},
};
use std::ffi::CStr;
///# [VkSamplerBorderColorComponentMappingCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerBorderColorComponentMappingCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_border_color_swizzle/VkSamplerBorderColorComponentMappingCreateInfoEXT.md")]
#[doc(alias = "VkSamplerBorderColorComponentMappingCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerBorderColorComponentMappingCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    components: ComponentMapping,
    srgb: Bool32,
}
///# [VkPhysicalDeviceBorderColorSwizzleFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_border_color_swizzle/VkPhysicalDeviceBorderColorSwizzleFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceBorderColorSwizzleFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceBorderColorSwizzleFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "borderColorSwizzle")]
    border_color_swizzle: Bool32,
    #[doc(alias = "borderColorSwizzleFromImage")]
    border_color_swizzle_from_image: Bool32,
}
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION")]
pub const EXT_BORDER_COLOR_SWIZZLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME")]
pub const EXT_BORDER_COLOR_SWIZZLE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_border_color_swizzle");
