//!# [VK_KHR_maintenance2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_maintenance2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_maintenance2/VK_KHR_maintenance2.md")]
use crate::{
    cstr,
    vulkan1_1::{
        ImageViewUsageCreateInfo, InputAttachmentAspectReference, PhysicalDevicePointClippingProperties,
        PipelineTessellationDomainOriginStateCreateInfo, PointClippingBehavior,
        RenderPassInputAttachmentAspectCreateInfo, TessellationDomainOrigin,
    },
};
use std::ffi::CStr;
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
#[doc(alias = "VK_KHR_MAINTENANCE_2_SPEC_VERSION")]
pub const KHR_MAINTENANCE_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_MAINTENANCE_2_EXTENSION_NAME")]
pub const KHR_MAINTENANCE_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_maintenance2");
///See [`KHR_MAINTENANCE_2_SPEC_VERSION`]
#[doc(alias = "VK_KHR_MAINTENANCE2_SPEC_VERSION")]
pub const KHR_MAINTENANCE2_SPEC_VERSION: u32 = KHR_MAINTENANCE_2_SPEC_VERSION;
///See [`KHR_MAINTENANCE_2_EXTENSION_NAME`]
#[doc(alias = "VK_KHR_MAINTENANCE2_EXTENSION_NAME")]
pub const KHR_MAINTENANCE2_EXTENSION_NAME: &'static CStr = KHR_MAINTENANCE_2_EXTENSION_NAME;
