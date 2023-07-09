use crate::native::vulkan1_2::{
    AttachmentDescriptionStencilLayout, AttachmentReferenceStencilLayout,
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
};
///See [`PhysicalDeviceSeparateDepthStencilLayoutsFeatures`]
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR")]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
///See [`AttachmentReferenceStencilLayout`]
#[doc(alias = "VkAttachmentReferenceStencilLayoutKHR")]
pub type AttachmentReferenceStencilLayoutKHR = AttachmentReferenceStencilLayout;
///See [`AttachmentDescriptionStencilLayout`]
#[doc(alias = "VkAttachmentDescriptionStencilLayoutKHR")]
pub type AttachmentDescriptionStencilLayoutKHR = AttachmentDescriptionStencilLayout;
pub use crate::common::extensions::khr_separate_depth_stencil_layouts::{
    KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME, KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION,
};
