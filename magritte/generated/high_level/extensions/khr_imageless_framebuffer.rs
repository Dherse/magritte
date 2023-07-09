pub use crate::common::extensions::khr_imageless_framebuffer::{
    KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME, KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION,
};
use crate::vulkan1_2::{
    FramebufferAttachmentImageInfo, FramebufferAttachmentsCreateInfo, PhysicalDeviceImagelessFramebufferFeatures,
    RenderPassAttachmentBeginInfo,
};
#[doc(alias = "VkPhysicalDeviceImagelessFramebufferFeaturesKHR")]
pub type PhysicalDeviceImagelessFramebufferFeaturesKHR = PhysicalDeviceImagelessFramebufferFeatures;
#[doc(alias = "VkFramebufferAttachmentsCreateInfoKHR")]
pub type FramebufferAttachmentsCreateInfoKHR = FramebufferAttachmentsCreateInfo;
#[doc(alias = "VkFramebufferAttachmentImageInfoKHR")]
pub type FramebufferAttachmentImageInfoKHR = FramebufferAttachmentImageInfo;
#[doc(alias = "VkRenderPassAttachmentBeginInfoKHR")]
pub type RenderPassAttachmentBeginInfoKHR = RenderPassAttachmentBeginInfo;
