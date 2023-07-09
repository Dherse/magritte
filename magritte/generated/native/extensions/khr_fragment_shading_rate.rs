use crate::native::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, PhysicalDevice, SampleCountFlagBits,
        SampleCountFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_2::AttachmentReference2,
};
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pFragmentShadingRateAttachment")]
    pub fragment_shading_rate_attachment: *const AttachmentReference2,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    pub shading_rate_attachment_texel_size: Extent2D,
}
impl Default for FragmentShadingRateAttachmentInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::FragmentShadingRateAttachmentInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fragment_shading_rate_attachment: unsafe { std::mem::zeroed() },
            shading_rate_attachment_texel_size: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "fragmentSize")]
    pub fragment_size: Extent2D,
    #[doc(alias = "combinerOps")]
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
impl Default for PipelineFragmentShadingRateStateCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineFragmentShadingRateStateCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fragment_size: unsafe { std::mem::zeroed() },
            combiner_ops: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "pipelineFragmentShadingRate")]
    pub pipeline_fragment_shading_rate: Bool32,
    #[doc(alias = "primitiveFragmentShadingRate")]
    pub primitive_fragment_shading_rate: Bool32,
    #[doc(alias = "attachmentFragmentShadingRate")]
    pub attachment_fragment_shading_rate: Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            pipeline_fragment_shading_rate: unsafe { std::mem::zeroed() },
            primitive_fragment_shading_rate: unsafe { std::mem::zeroed() },
            attachment_fragment_shading_rate: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "minFragmentShadingRateAttachmentTexelSize")]
    pub min_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSize")]
    pub max_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSizeAspectRatio")]
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    #[doc(alias = "primitiveFragmentShadingRateWithMultipleViewports")]
    pub primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    #[doc(alias = "layeredShadingRateAttachments")]
    pub layered_shading_rate_attachments: Bool32,
    #[doc(alias = "fragmentShadingRateNonTrivialCombinerOps")]
    pub fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    #[doc(alias = "maxFragmentSize")]
    pub max_fragment_size: Extent2D,
    #[doc(alias = "maxFragmentSizeAspectRatio")]
    pub max_fragment_size_aspect_ratio: u32,
    #[doc(alias = "maxFragmentShadingRateCoverageSamples")]
    pub max_fragment_shading_rate_coverage_samples: u32,
    #[doc(alias = "maxFragmentShadingRateRasterizationSamples")]
    pub max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "fragmentShadingRateWithShaderDepthStencilWrites")]
    pub fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    #[doc(alias = "fragmentShadingRateWithSampleMask")]
    pub fragment_shading_rate_with_sample_mask: Bool32,
    #[doc(alias = "fragmentShadingRateWithShaderSampleMask")]
    pub fragment_shading_rate_with_shader_sample_mask: Bool32,
    #[doc(alias = "fragmentShadingRateWithConservativeRasterization")]
    pub fragment_shading_rate_with_conservative_rasterization: Bool32,
    #[doc(alias = "fragmentShadingRateWithFragmentShaderInterlock")]
    pub fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    #[doc(alias = "fragmentShadingRateWithCustomSampleLocations")]
    pub fragment_shading_rate_with_custom_sample_locations: Bool32,
    #[doc(alias = "fragmentShadingRateStrictMultiplyCombiner")]
    pub fragment_shading_rate_strict_multiply_combiner: Bool32,
}
impl Default for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShadingRatePropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            min_fragment_shading_rate_attachment_texel_size: unsafe { std::mem::zeroed() },
            max_fragment_shading_rate_attachment_texel_size: unsafe { std::mem::zeroed() },
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: unsafe { std::mem::zeroed() },
            primitive_fragment_shading_rate_with_multiple_viewports: unsafe { std::mem::zeroed() },
            layered_shading_rate_attachments: unsafe { std::mem::zeroed() },
            fragment_shading_rate_non_trivial_combiner_ops: unsafe { std::mem::zeroed() },
            max_fragment_size: unsafe { std::mem::zeroed() },
            max_fragment_size_aspect_ratio: unsafe { std::mem::zeroed() },
            max_fragment_shading_rate_coverage_samples: unsafe { std::mem::zeroed() },
            max_fragment_shading_rate_rasterization_samples: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_shader_depth_stencil_writes: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_sample_mask: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_shader_sample_mask: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_conservative_rasterization: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_fragment_shader_interlock: unsafe { std::mem::zeroed() },
            fragment_shading_rate_with_custom_sample_locations: unsafe { std::mem::zeroed() },
            fragment_shading_rate_strict_multiply_combiner: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "sampleCounts")]
    pub sample_counts: SampleCountFlags,
    #[doc(alias = "fragmentSize")]
    pub fragment_size: Extent2D,
}
impl Default for PhysicalDeviceFragmentShadingRateKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateKhr,
            p_next: unsafe { std::mem::zeroed() },
            sample_counts: unsafe { std::mem::zeroed() },
            fragment_size: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_fragment_shading_rate::{
    FragmentShadingRateCombinerOpKHR, KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME, KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION,
};
#[doc(alias = "vkGetPhysicalDeviceFragmentShadingRatesKHR")]
pub type FNGetPhysicalDeviceFragmentShadingRatesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_fragment_shading_rate_count: *mut u32,
    p_fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSetFragmentShadingRateKHR")]
pub type FNCmdSetFragmentShadingRateKhr = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_fragment_size: *const Extent2D,
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
);
