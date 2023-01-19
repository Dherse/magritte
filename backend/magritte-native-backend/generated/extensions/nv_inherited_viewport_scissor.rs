//!# [VK_NV_inherited_viewport_scissor](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_inherited_viewport_scissor.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_inherited_viewport_scissor/VK_NV_inherited_viewport_scissor.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Bool32, StructureType, Viewport},
};
use std::ffi::CStr;
///# [VkPhysicalDeviceInheritedViewportScissorFeaturesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_inherited_viewport_scissor/VkPhysicalDeviceInheritedViewportScissorFeaturesNV.md")]
#[doc(alias = "VkPhysicalDeviceInheritedViewportScissorFeaturesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceInheritedViewportScissorFeaturesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "inheritedViewportScissor2D")]
    inherited_viewport_scissor2_d: Bool32,
}
///# [VkCommandBufferInheritanceViewportScissorInfoNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCommandBufferInheritanceViewportScissorInfoNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_inherited_viewport_scissor/VkCommandBufferInheritanceViewportScissorInfoNV.md")]
#[doc(alias = "VkCommandBufferInheritanceViewportScissorInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CommandBufferInheritanceViewportScissorInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "viewportScissor2D")]
    viewport_scissor2_d: Bool32,
    #[doc(alias = "viewportDepthCount")]
    viewport_depth_count: u32,
    #[doc(alias = "pViewportDepths")]
    viewport_depths: *const Viewport,
}
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME")]
pub const NV_INHERITED_VIEWPORT_SCISSOR_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_inherited_viewport_scissor");
