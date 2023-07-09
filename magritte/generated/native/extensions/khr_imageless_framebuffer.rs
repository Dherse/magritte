use crate::native::vulkan1_2::{
    FramebufferAttachmentImageInfo, FramebufferAttachmentsCreateInfo, PhysicalDeviceImagelessFramebufferFeatures,
    RenderPassAttachmentBeginInfo,
};
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
pub use crate::common::extensions::khr_imageless_framebuffer::{
    KHR_IMAGELESS_FRAMEBUFFER_EXTENSION_NAME, KHR_IMAGELESS_FRAMEBUFFER_SPEC_VERSION,
};
