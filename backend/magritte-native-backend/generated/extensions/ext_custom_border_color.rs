use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, ClearColorValue, Format, StructureType},
};
use std::ffi::CStr;
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
