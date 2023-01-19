//!# [VK_NVX_multiview_per_view_attributes](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NVX_multiview_per_view_attributes.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nvx_multiview_per_view_attributes/VK_NVX_multiview_per_view_attributes.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/nvx_multiview_per_view_attributes/VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX.md")]
#[doc(alias = "VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "perViewPositionAllComponents")]
    per_view_position_all_components: Bool32,
}
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME")]
pub const NVX_MULTIVIEW_PER_VIEW_ATTRIBUTES_EXTENSION_NAME: &'static CStr =
    cstr!("VK_NVX_multiview_per_view_attributes");
