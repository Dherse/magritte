//!# [VK_NV_linear_color_attachment](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_linear_color_attachment.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_linear_color_attachment/VK_NV_linear_color_attachment.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceLinearColorAttachmentFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_linear_color_attachment/VkPhysicalDeviceLinearColorAttachmentFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceLinearColorAttachmentFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceLinearColorAttachmentFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "linearColorAttachment")]
    linear_color_attachment: Bool32,
}
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION")]
pub const NV_LINEAR_COLOR_ATTACHMENT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME")]
pub const NV_LINEAR_COLOR_ATTACHMENT_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_linear_color_attachment");
