pub use crate::common::extensions::khr_separate_depth_stencil_layouts::{
    KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_EXTENSION_NAME, KHR_SEPARATE_DEPTH_STENCIL_LAYOUTS_SPEC_VERSION,
};
use crate::vulkan1_2::{
    AttachmentDescriptionStencilLayout, AttachmentReferenceStencilLayout,
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures,
};
#[doc(alias = "VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR")]
pub type PhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR = PhysicalDeviceSeparateDepthStencilLayoutsFeatures;
#[doc(alias = "VkAttachmentReferenceStencilLayoutKHR")]
pub type AttachmentReferenceStencilLayoutKHR = AttachmentReferenceStencilLayout;
#[doc(alias = "VkAttachmentDescriptionStencilLayoutKHR")]
pub type AttachmentDescriptionStencilLayoutKHR = AttachmentDescriptionStencilLayout;
