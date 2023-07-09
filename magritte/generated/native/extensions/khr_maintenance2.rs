use crate::native::vulkan1_1::{
    ImageViewUsageCreateInfo, InputAttachmentAspectReference, PhysicalDevicePointClippingProperties,
    PipelineTessellationDomainOriginStateCreateInfo, PointClippingBehavior, RenderPassInputAttachmentAspectCreateInfo,
    TessellationDomainOrigin,
};
///See [`PointClippingBehavior`]
#[doc(alias = "VkPointClippingBehaviorKHR")]
pub type PointClippingBehaviorKHR = PointClippingBehavior;
///See [`TessellationDomainOrigin`]
#[doc(alias = "VkTessellationDomainOriginKHR")]
pub type TessellationDomainOriginKHR = TessellationDomainOrigin;
///See [`InputAttachmentAspectReference`]
#[doc(alias = "VkInputAttachmentAspectReferenceKHR")]
pub type InputAttachmentAspectReferenceKHR = InputAttachmentAspectReference;
///See [`RenderPassInputAttachmentAspectCreateInfo`]
#[doc(alias = "VkRenderPassInputAttachmentAspectCreateInfoKHR")]
pub type RenderPassInputAttachmentAspectCreateInfoKHR = RenderPassInputAttachmentAspectCreateInfo;
///See [`PhysicalDevicePointClippingProperties`]
#[doc(alias = "VkPhysicalDevicePointClippingPropertiesKHR")]
pub type PhysicalDevicePointClippingPropertiesKHR = PhysicalDevicePointClippingProperties;
///See [`ImageViewUsageCreateInfo`]
#[doc(alias = "VkImageViewUsageCreateInfoKHR")]
pub type ImageViewUsageCreateInfoKHR = ImageViewUsageCreateInfo;
///See [`PipelineTessellationDomainOriginStateCreateInfo`]
#[doc(alias = "VkPipelineTessellationDomainOriginStateCreateInfoKHR")]
pub type PipelineTessellationDomainOriginStateCreateInfoKHR = PipelineTessellationDomainOriginStateCreateInfo;
pub use crate::common::extensions::khr_maintenance2::{
    KHR_MAINTENANCE2_EXTENSION_NAME, KHR_MAINTENANCE2_SPEC_VERSION, KHR_MAINTENANCE_2_EXTENSION_NAME,
    KHR_MAINTENANCE_2_SPEC_VERSION,
};
