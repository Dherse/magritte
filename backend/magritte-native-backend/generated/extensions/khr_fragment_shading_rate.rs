use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, CommandBuffer, Extent2D, PhysicalDevice, SampleCountFlagBits,
        SampleCountFlags, StructureType, VulkanResultCodes,
    },
    vulkan1_2::AttachmentReference2,
};
use std::ffi::CStr;
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pFragmentShadingRateAttachment")]
    fragment_shading_rate_attachment: *const AttachmentReference2,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    shading_rate_attachment_texel_size: Extent2D,
}
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "fragmentSize")]
    fragment_size: Extent2D,
    #[doc(alias = "combinerOps")]
    combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "pipelineFragmentShadingRate")]
    pipeline_fragment_shading_rate: Bool32,
    #[doc(alias = "primitiveFragmentShadingRate")]
    primitive_fragment_shading_rate: Bool32,
    #[doc(alias = "attachmentFragmentShadingRate")]
    attachment_fragment_shading_rate: Bool32,
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "minFragmentShadingRateAttachmentTexelSize")]
    min_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSize")]
    max_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSizeAspectRatio")]
    max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    #[doc(alias = "primitiveFragmentShadingRateWithMultipleViewports")]
    primitive_fragment_shading_rate_with_multiple_viewports: Bool32,
    #[doc(alias = "layeredShadingRateAttachments")]
    layered_shading_rate_attachments: Bool32,
    #[doc(alias = "fragmentShadingRateNonTrivialCombinerOps")]
    fragment_shading_rate_non_trivial_combiner_ops: Bool32,
    #[doc(alias = "maxFragmentSize")]
    max_fragment_size: Extent2D,
    #[doc(alias = "maxFragmentSizeAspectRatio")]
    max_fragment_size_aspect_ratio: u32,
    #[doc(alias = "maxFragmentShadingRateCoverageSamples")]
    max_fragment_shading_rate_coverage_samples: u32,
    #[doc(alias = "maxFragmentShadingRateRasterizationSamples")]
    max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "fragmentShadingRateWithShaderDepthStencilWrites")]
    fragment_shading_rate_with_shader_depth_stencil_writes: Bool32,
    #[doc(alias = "fragmentShadingRateWithSampleMask")]
    fragment_shading_rate_with_sample_mask: Bool32,
    #[doc(alias = "fragmentShadingRateWithShaderSampleMask")]
    fragment_shading_rate_with_shader_sample_mask: Bool32,
    #[doc(alias = "fragmentShadingRateWithConservativeRasterization")]
    fragment_shading_rate_with_conservative_rasterization: Bool32,
    #[doc(alias = "fragmentShadingRateWithFragmentShaderInterlock")]
    fragment_shading_rate_with_fragment_shader_interlock: Bool32,
    #[doc(alias = "fragmentShadingRateWithCustomSampleLocations")]
    fragment_shading_rate_with_custom_sample_locations: Bool32,
    #[doc(alias = "fragmentShadingRateStrictMultiplyCombiner")]
    fragment_shading_rate_strict_multiply_combiner: Bool32,
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "sampleCounts")]
    sample_counts: SampleCountFlags,
    #[doc(alias = "fragmentSize")]
    fragment_size: Extent2D,
}
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_fragment_shading_rate");
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct FragmentShadingRateCombinerOpKHR(i32);
impl FragmentShadingRateCombinerOpKHR {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR")]
    pub const KEEP: Self = Self(0);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR")]
    pub const REPLACE: Self = Self(1);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR")]
    pub const MIN: Self = Self(2);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR")]
    pub const MAX: Self = Self(3);
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR")]
    pub const MUL: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::KEEP.bits() => Some(Self(x)),
            x if x == Self::REPLACE.bits() => Some(Self(x)),
            x if x == Self::MIN.bits() => Some(Self(x)),
            x if x == Self::MAX.bits() => Some(Self(x)),
            x if x == Self::MUL.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
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
