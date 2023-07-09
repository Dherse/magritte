use crate::native::{
    vulkan1_0::{BaseInStructure, Bool32, Extent2D, ImageLayout, ImageView, SampleCountFlagBits, StructureType},
    vulkan1_3::{
        CommandBufferInheritanceRenderingInfo, FNCmdBeginRendering, FNCmdEndRendering,
        PhysicalDeviceDynamicRenderingFeatures, PipelineRenderingCreateInfo, RenderingAttachmentInfo,
        RenderingFlagBits, RenderingFlags, RenderingInfo,
    },
};
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
#[doc(alias = "VkRenderingFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    pub shading_rate_attachment_texel_size: Extent2D,
}
impl Default for RenderingFragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderingFragmentShadingRateAttachmentInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            image_view: unsafe { std::mem::zeroed() },
            image_layout: unsafe { std::mem::zeroed() },
            shading_rate_attachment_texel_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkRenderingFragmentDensityMapAttachmentInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RenderingFragmentDensityMapAttachmentInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "imageView")]
    pub image_view: ImageView,
    #[doc(alias = "imageLayout")]
    pub image_layout: ImageLayout,
}
impl Default for RenderingFragmentDensityMapAttachmentInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::RenderingFragmentDensityMapAttachmentInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            image_view: unsafe { std::mem::zeroed() },
            image_layout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAttachmentSampleCountInfoAMD")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AttachmentSampleCountInfoAMD {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "colorAttachmentCount")]
    pub color_attachment_count: u32,
    #[doc(alias = "pColorAttachmentSamples")]
    pub color_attachment_samples: *const SampleCountFlagBits,
    #[doc(alias = "depthStencilAttachmentSamples")]
    pub depth_stencil_attachment_samples: SampleCountFlagBits,
}
impl Default for AttachmentSampleCountInfoAMD {
    fn default() -> Self {
        Self {
            s_type: StructureType::AttachmentSampleCountInfoAmd,
            p_next: unsafe { std::mem::zeroed() },
            color_attachment_count: unsafe { std::mem::zeroed() },
            color_attachment_samples: unsafe { std::mem::zeroed() },
            depth_stencil_attachment_samples: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMultiviewPerViewAttributesInfoNVX")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultiviewPerViewAttributesInfoNVX {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "perViewAttributes")]
    pub per_view_attributes: Bool32,
    #[doc(alias = "perViewAttributesPositionXOnly")]
    pub per_view_attributes_position_x_only: Bool32,
}
impl Default for MultiviewPerViewAttributesInfoNVX {
    fn default() -> Self {
        Self {
            s_type: StructureType::MultiviewPerViewAttributesInfoNvx,
            p_next: unsafe { std::mem::zeroed() },
            per_view_attributes: unsafe { std::mem::zeroed() },
            per_view_attributes_position_x_only: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_dynamic_rendering::{
    KHR_DYNAMIC_RENDERING_EXTENSION_NAME, KHR_DYNAMIC_RENDERING_SPEC_VERSION,
};
///See [`cmd_begin_rendering`]
#[doc(alias = "vkCmdBeginRenderingKHR")]
pub type FNCmdBeginRenderingKhr = FNCmdBeginRendering;
///See [`cmd_end_rendering`]
#[doc(alias = "vkCmdEndRenderingKHR")]
pub type FNCmdEndRenderingKhr = FNCmdEndRendering;
