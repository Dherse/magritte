pub use crate::common::extensions::khr_maintenance2::{
    KHR_MAINTENANCE2_EXTENSION_NAME, KHR_MAINTENANCE2_SPEC_VERSION, KHR_MAINTENANCE_2_EXTENSION_NAME,
    KHR_MAINTENANCE_2_SPEC_VERSION,
};
use crate::vulkan1_1::{
    ImageViewUsageCreateInfo, InputAttachmentAspectReference, PhysicalDevicePointClippingProperties,
    PipelineTessellationDomainOriginStateCreateInfo, PointClippingBehavior, RenderPassInputAttachmentAspectCreateInfo,
    TessellationDomainOrigin,
};
#[doc(alias = "VkPointClippingBehaviorKHR")]
pub type PointClippingBehaviorKHR = PointClippingBehavior;
#[doc(alias = "VkTessellationDomainOriginKHR")]
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
#[doc(alias = "VkInputAttachmentAspectReferenceKHR")]
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfoKHR")]
pub type RenderPassInputAttachmentAspectCreateInfoKHR = RenderPassInputAttachmentAspectCreateInfo;
#[doc(alias = "VkPhysicalDevicePointClippingPropertiesKHR")]
pub type PhysicalDevicePointClippingPropertiesKHR = PhysicalDevicePointClippingProperties;
#[doc(alias = "VkImageViewUsageCreateInfoKHR")]
pub type ImageViewUsageCreateInfoKHR = ImageViewUsageCreateInfo;
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfoKHR")]
pub type PipelineTessellationDomainOriginStateCreateInfoKHR = PipelineTessellationDomainOriginStateCreateInfo;
