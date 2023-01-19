//!# [VK_NV_scissor_exclusive](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_scissor_exclusive.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_scissor_exclusive/VK_NV_scissor_exclusive.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Rect2D, StructureType},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceExclusiveScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceExclusiveScissorFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_scissor_exclusive/VkPhysicalDeviceExclusiveScissorFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceExclusiveScissorFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceExclusiveScissorFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "exclusiveScissor")]
    exclusive_scissor: Bool32,
}
///# [VkPipelineViewportExclusiveScissorStateCreateInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportExclusiveScissorStateCreateInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_scissor_exclusive/VkPipelineViewportExclusiveScissorStateCreateInfoNV.md")]
#[doc(alias = "VkPipelineViewportExclusiveScissorStateCreateInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineViewportExclusiveScissorStateCreateInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "exclusiveScissorCount")]
    exclusive_scissor_count: u32,
    #[doc(alias = "pExclusiveScissors")]
    exclusive_scissors: *const Rect2D,
}
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_SPEC_VERSION")]
pub const NV_SCISSOR_EXCLUSIVE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME")]
pub const NV_SCISSOR_EXCLUSIVE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_scissor_exclusive");
///# [vkCmdSetExclusiveScissorNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetExclusiveScissorNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_scissor_exclusive/vkCmdSetExclusiveScissorNV.md")]
#[doc(alias = "vkCmdSetExclusiveScissorNV")]
pub type FNCmdSetExclusiveScissorNv = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    p_exclusive_scissors: *const Rect2D,
);
