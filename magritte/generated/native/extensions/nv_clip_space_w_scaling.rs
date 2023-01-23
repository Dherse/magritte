//!# [VK_NV_clip_space_w_scaling](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_clip_space_w_scaling.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_clip_space_w_scaling/VK_NV_clip_space_w_scaling.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, CommandBuffer, StructureType},
};
use std::ffi::CStr;
///# [VkViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkViewportWScalingNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_clip_space_w_scaling/VkViewportWScalingNV.md")]
#[doc(alias = "VkViewportWScalingNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ViewportWScalingNV {
    xcoeff: f32,
    ycoeff: f32,
}
///# [VkPipelineViewportWScalingStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportWScalingStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_clip_space_w_scaling/VkPipelineViewportWScalingStateCreateInfoNV.md")]
#[doc(alias = "VkPipelineViewportWScalingStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportWScalingStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "viewportWScalingEnable")]
    viewport_w_scaling_enable: Bool32,
    #[doc(alias = "viewportCount")]
    viewport_count: u32,
    #[doc(alias = "pViewportWScalings")]
    viewport_w_scalings: *const ViewportWScalingNV,
}
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_SPEC_VERSION")]
pub const NV_CLIP_SPACE_W_SCALING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME")]
pub const NV_CLIP_SPACE_W_SCALING_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_clip_space_w_scaling");
///# [vkCmdSetViewportWScalingNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetViewportWScalingNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_clip_space_w_scaling/vkCmdSetViewportWScalingNV.md")]
#[doc(alias = "vkCmdSetViewportWScalingNV")]
pub type FNCmdSetViewportWScalingNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    p_viewport_w_scalings: *const ViewportWScalingNV,
);
