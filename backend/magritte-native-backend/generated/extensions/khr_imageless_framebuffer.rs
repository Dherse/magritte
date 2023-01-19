//!# [VK_KHR_imageless_framebuffer](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_imageless_framebuffer.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_imageless_framebuffer/VK_KHR_imageless_framebuffer.md")]
use crate::{
    cstr,
    vulkan1_2::{
        FramebufferAttachmentImageInfo, FramebufferAttachmentsCreateInfo, PhysicalDeviceImagelessFramebufferFeatures,
        RenderPassAttachmentBeginInfo,
    },
};
use std::ffi::CStr;
///See [`PhysicalDeviceImagelessFramebufferFeatures`]
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeaturesKHR")]
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR = PhysicalDeviceImagelessFramebufferFeatures;
///See [`FramebufferAttachmentsCreateInfo`]
#[doc(alias = "VkFramebufferAttachmentsCreateInfoKHR")]
pub type FramebufferAttachmentsCreateInfoKHR = FramebufferAttachmentsCreateInfo;
///See [`FramebufferAttachmentImageInfo`]
#[doc(alias = "VkFramebufferAttachmentImageInfoKHR")]
pub type FramebufferAttachmentImageInfoKHR = FramebufferAttachmentImageInfo;
///See [`RenderPassAttachmentBeginInfo`]
#[doc(alias = "VkRenderPassAttachmentBeginInfoKHR")]
pub type RenderPassAttachmentBeginInfoKHR = RenderPassAttachmentBeginInfo;
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION")]
pub const KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME")]
pub const KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_imageless_framebuffer");
