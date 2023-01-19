//!# [VK_EXT_depth_clip_control](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_control.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_control/VK_EXT_depth_clip_control.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDepthClipControlFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipControlFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_control/VkPhysicalDeviceDepthClipControlFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceDepthClipControlFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipControlFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "depthClipControl")]
    depth_clip_control: Bool32,
}
///# [VkPipelineViewportDepthClipControlCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportDepthClipControlCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_control/VkPipelineViewportDepthClipControlCreateInfoEXT.md")]
#[doc(alias = "VkPipelineViewportDepthClipControlCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportDepthClipControlCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "negativeOneToOne")]
    negative_one_to_one: Bool32,
}
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_CONTROL_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_CONTROL_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_depth_clip_control");
