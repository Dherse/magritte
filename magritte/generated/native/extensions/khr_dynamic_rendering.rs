//!# [VK_KHR_dynamic_rendering](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_dynamic_rendering.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_dynamic_rendering/VK_KHR_dynamic_rendering.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, Extent2D, ImageLayout, ImageView, SampleCountFlagBits, StructureType},
    vulkan1_3::{
        CommandBufferInheritanceRenderingInfo, FNCmdBeginRendering, FNCmdEndRendering,
        PhysicalDeviceDynamicRenderingFeatures, PipelineRenderingCreateInfo, RenderingAttachmentInfo,
        RenderingFlagBits, RenderingFlags, RenderingInfo,
    },
};
use std::ffi::CStr;
///See [`RenderingFlags`]
#[doc(alias = "VkRenderingFlagsKHR")]
pub type RenderingFlagsKHR = RenderingFlags;
///See [`RenderingFlagBits`]
#[doc(alias = "VkRenderingFlagBitsKHR")]
pub type RenderingFlagBitsKHR = RenderingFlagBits;
///See [`PipelineRenderingCreateInfo`]
#[doc(alias = "VkPipelineRenderingCreateInfoKHR")]
pub type PipelineRenderingCreateInfoKHR = PipelineRenderingCreateInfo;
///See [`RenderingInfo`]
#[doc(alias = "VkRenderingInfoKHR")]
pub type RenderingInfoKHR = RenderingInfo;
///See [`RenderingAttachmentInfo`]
#[doc(alias = "VkRenderingAttachmentInfoKHR")]
pub type RenderingAttachmentInfoKHR = RenderingAttachmentInfo;
///See [`PhysicalDeviceDynamicRenderingFeatures`]
#[doc(alias = "VkPhysicalDeviceDynamicRenderingFeaturesKHR")]
pub type PhysicalDeviceDynamicRenderingFeaturesKHR = PhysicalDeviceDynamicRenderingFeatures;
///See [`CommandBufferInheritanceRenderingInfo`]
#[doc(alias = "VkCommandBufferInheritanceRenderingInfoKHR")]
pub type CommandBufferInheritanceRenderingInfoKHR = CommandBufferInheritanceRenderingInfo;
///See [`AttachmentSampleCountInfoAMD`]
#[doc(alias = "VkAttachmentSampleCountInfoNV")]
pub type AttachmentSampleCountInfoNV = AttachmentSampleCountInfoAMD;
///# [VkRenderingFragmentShadingRateAttachmentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentShadingRateAttachmentInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_dynamic_rendering/VkRenderingFragmentShadingRateAttachmentInfoKHR.md")]
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    image_view: ImageView,
    #[doc(alias = "imageLayout")]
    image_layout: ImageLayout,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    shading_rate_attachment_texel_size: Extent2D,
}
///# [VkRenderingFragmentDensityMapAttachmentInfoEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkRenderingFragmentDensityMapAttachmentInfoEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_dynamic_rendering/VkRenderingFragmentDensityMapAttachmentInfoEXT.md")]
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    image_view: ImageView,
    #[doc(alias = "imageLayout")]
    image_layout: ImageLayout,
}
///# [VkAttachmentSampleCountInfoAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkAttachmentSampleCountInfoAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_dynamic_rendering/VkAttachmentSampleCountInfoAMD.md")]
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "colorAttachmentCount")]
    color_attachment_count: u32,
    #[doc(alias = "pColorAttachmentSamples")]
    color_attachment_samples: *const SampleCountFlagBits,
    #[doc(alias = "depthStencilAttachmentSamples")]
    depth_stencil_attachment_samples: SampleCountFlagBits,
}
///# [VkMultiviewPerViewAttributesInfoNVX](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMultiviewPerViewAttributesInfoNVX.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_dynamic_rendering/VkMultiviewPerViewAttributesInfoNVX.md")]
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "perViewAttributes")]
    per_view_attributes: Bool32,
    #[doc(alias = "perViewAttributesPositionXOnly")]
    per_view_attributes_position_x_only: Bool32,
}
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_SPEC_VERSION")]
pub const KHR_DYNAMIC_RENDERING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_DYNAMIC_RENDERING_EXTENSION_NAME")]
pub const KHR_DYNAMIC_RENDERING_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_dynamic_rendering");
///See [`cmd_begin_rendering`]
#[doc(alias = "vkCmdBeginRenderingKHR")]
pub type FNCmdBeginRenderingKhr = FNCmdBeginRendering;
///See [`cmd_end_rendering`]
#[doc(alias = "vkCmdEndRenderingKHR")]
pub type FNCmdEndRenderingKhr = FNCmdEndRendering;
