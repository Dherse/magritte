//!# [VK_KHR_separate_depth_stencil_layouts](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_separate_depth_stencil_layouts.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_separate_depth_stencil_layouts/VK_KHR_separate_depth_stencil_layouts.md")]
use crate::{
    cstr,
    vulkan1_2::{
        AttachmentDescriptionStencilLayout, AttachmentReferenceStencilLayout,
        PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
    },
};
use std::ffi::CStr;
///See [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR")]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
///See [`AttachmentReferenceStencilLayout`]
#[doc(alias = "VkAttachmentReferenceStencilLayoutKHR")]
pub type AttachmentReferenceStencilLayoutKHR = AttachmentReferenceStencilLayout;
///See [`AttachmentDescriptionStencilLayout`]
#[doc(alias = "VkAttachmentDescriptionStencilLayoutKHR")]
pub type AttachmentDescriptionStencilLayoutKHR = AttachmentDescriptionStencilLayout;
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME")]
pub const KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME: &'static CStr =
    cstr!("VK_KHR_separate_depth_stencil_layouts");
