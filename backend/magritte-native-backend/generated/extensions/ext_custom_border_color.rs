//!# [VK_EXT_custom_border_color](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_custom_border_color.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_custom_border_color/VK_EXT_custom_border_color.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ClearColorValue, Format, StructureType},
};
use std::ffi::CStr;
///# [VkSamplerCustomBorderColorCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSamplerCustomBorderColorCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_custom_border_color/VkSamplerCustomBorderColorCreateInfoEXT.md")]
#[doc(alias = "VkSamplerCustomBorderColorCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SamplerCustomBorderColorCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "customBorderColor")]
    custom_border_color: ClearColorValue,
    format: Format,
}
///# [VkPhysicalDeviceCustomBorderColorPropertiesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorPropertiesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_custom_border_color/VkPhysicalDeviceCustomBorderColorPropertiesEXT.md")]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxCustomBorderColorSamplers")]
    max_custom_border_color_samplers: u32,
}
///# [VkPhysicalDeviceCustomBorderColorFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceCustomBorderColorFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_custom_border_color/VkPhysicalDeviceCustomBorderColorFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceCustomBorderColorFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceCustomBorderColorFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "customBorderColors")]
    custom_border_colors: Bool32,
    #[doc(alias = "customBorderColorWithoutFormat")]
    custom_border_color_without_format: Bool32,
}
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION")]
pub const EXT_CUSTOM_BORDER_COLOR_SPEC_VERSION: u32 = 12;
#[doc(alias = "VK_EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME")]
pub const EXT_CUSTOM_BORDER_COLOR_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_custom_border_color");
