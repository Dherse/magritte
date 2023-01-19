//!# [VK_EXT_depth_clip_enable](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_depth_clip_enable.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_enable/VK_EXT_depth_clip_enable.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceDepthClipEnableFeaturesEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceDepthClipEnableFeaturesEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_enable/VkPhysicalDeviceDepthClipEnableFeaturesEXT.md")]
#[doc(alias = "VkPhysicalDeviceDepthClipEnableFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceDepthClipEnableFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "depthClipEnable")]
    depth_clip_enable: Bool32,
}
///# [VkPipelineRasterizationDepthClipStateCreateInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationDepthClipStateCreateInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_depth_clip_enable/VkPipelineRasterizationDepthClipStateCreateInfoEXT.md")]
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationDepthClipStateCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineRasterizationDepthClipStateCreateFlagsEXT,
    #[doc(alias = "depthClipEnable")]
    depth_clip_enable: Bool32,
}
#[doc(alias = "VkPipelineRasterizationDepthClipStateCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationDepthClipStateCreateFlagsEXT(u32);
impl PipelineRasterizationDepthClipStateCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION")]
pub const EXT_DEPTH_CLIP_ENABLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME")]
pub const EXT_DEPTH_CLIP_ENABLE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_depth_clip_enable");
