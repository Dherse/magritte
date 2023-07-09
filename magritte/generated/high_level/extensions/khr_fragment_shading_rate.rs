pub use crate::common::extensions::khr_fragment_shading_rate::{
    FragmentShadingRateCombinerOpKHR, KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME, KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{Extent2D, SampleCountFlagBits, SampleCountFlags, StructureType},
    vulkan1_2::AttachmentReference2,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkFragmentShadingRateAttachmentInfoKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FragmentShadingRateAttachmentInfoKHR {
    #[doc(alias = "pFragmentShadingRateAttachment")]
    pub fragment_shading_rate_attachment: Option<AttachmentReference2>,
    #[doc(alias = "shadingRateAttachmentTexelSize")]
    pub shading_rate_attachment_texel_size: Extent2D,
}
impl FragmentShadingRateAttachmentInfoKHR {
    ///Get a reference to the `fragment_shading_rate_attachment` field.
    pub fn fragment_shading_rate_attachment(&self) -> &Option<AttachmentReference2> {
        &self.fragment_shading_rate_attachment
    }
    ///Get a reference to the `shading_rate_attachment_texel_size` field.
    pub fn shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.shading_rate_attachment_texel_size
    }
    ///Get a mutable reference to the `fragment_shading_rate_attachment` field.
    pub fn fragment_shading_rate_attachment_mut(&mut self) -> &mut Option<AttachmentReference2> {
        &mut self.fragment_shading_rate_attachment
    }
    ///Get a mutable reference to the `shading_rate_attachment_texel_size` field.
    pub fn shading_rate_attachment_texel_size_mut(&mut self) -> &mut Extent2D {
        &mut self.shading_rate_attachment_texel_size
    }
    ///Sets the `fragment_shading_rate_attachment` field.
    pub fn set_fragment_shading_rate_attachment(
        &mut self,
        fragment_shading_rate_attachment: Option<AttachmentReference2>,
    ) -> &mut Self {
        self.fragment_shading_rate_attachment = fragment_shading_rate_attachment;
        self
    }
    ///Sets the `shading_rate_attachment_texel_size` field.
    pub fn set_shading_rate_attachment_texel_size(
        &mut self,
        shading_rate_attachment_texel_size: Extent2D,
    ) -> &mut Self {
        self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
    ///Sets the `fragment_shading_rate_attachment` field in a builder way.
    pub fn with_fragment_shading_rate_attachment(
        mut self,
        fragment_shading_rate_attachment: Option<AttachmentReference2>,
    ) -> Self {
        self.fragment_shading_rate_attachment = fragment_shading_rate_attachment;
        self
    }
    ///Sets the `shading_rate_attachment_texel_size` field in a builder way.
    pub fn with_shading_rate_attachment_texel_size(mut self, shading_rate_attachment_texel_size: Extent2D) -> Self {
        self.shading_rate_attachment_texel_size = shading_rate_attachment_texel_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for FragmentShadingRateAttachmentInfoKHR {
    type LowLevel = crate::native::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_fragment_shading_rate::FragmentShadingRateAttachmentInfoKHR {
            s_type: StructureType::FragmentShadingRateAttachmentInfoKhr,
            p_next: std::ptr::null(),
            fragment_shading_rate_attachment: self
                .fragment_shading_rate_attachment
                .as_ref()
                .map(|v| bump.alloc(v.into_low_level(context, bump)) as *const _)
                .unwrap_or_else(std::ptr::null),
            shading_rate_attachment_texel_size: self.shading_rate_attachment_texel_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for FragmentShadingRateAttachmentInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_shading_rate_attachment: crate::conv::FromLowLevel::from_low_level(
                context,
                *value.fragment_shading_rate_attachment,
            ),
            shading_rate_attachment_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shading_rate_attachment_texel_size,
            ),
        }
    }
}
#[doc(alias = "VkPipelineFragmentShadingRateStateCreateInfoKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineFragmentShadingRateStateCreateInfoKHR {
    #[doc(alias = "fragmentSize")]
    pub fragment_size: Extent2D,
    #[doc(alias = "combinerOps")]
    pub combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize],
}
impl PipelineFragmentShadingRateStateCreateInfoKHR {
    ///Get a reference to the `fragment_size` field.
    pub fn fragment_size(&self) -> Extent2D {
        self.fragment_size
    }
    ///Get a reference to the `combiner_ops` field.
    pub fn combiner_ops(&self) -> [FragmentShadingRateCombinerOpKHR; 2 as usize] {
        self.combiner_ops
    }
    ///Get a mutable reference to the `fragment_size` field.
    pub fn fragment_size_mut(&mut self) -> &mut Extent2D {
        &mut self.fragment_size
    }
    ///Get a mutable reference to the `combiner_ops` field.
    pub fn combiner_ops_mut(&mut self) -> &mut [FragmentShadingRateCombinerOpKHR; 2 as usize] {
        &mut self.combiner_ops
    }
    ///Sets the `fragment_size` field.
    pub fn set_fragment_size(&mut self, fragment_size: Extent2D) -> &mut Self {
        self.fragment_size = fragment_size;
        self
    }
    ///Sets the `combiner_ops` field.
    pub fn set_combiner_ops(&mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize]) -> &mut Self {
        self.combiner_ops = combiner_ops;
        self
    }
    ///Sets the `fragment_size` field in a builder way.
    pub fn with_fragment_size(mut self, fragment_size: Extent2D) -> Self {
        self.fragment_size = fragment_size;
        self
    }
    ///Sets the `combiner_ops` field in a builder way.
    pub fn with_combiner_ops(mut self, combiner_ops: [FragmentShadingRateCombinerOpKHR; 2 as usize]) -> Self {
        self.combiner_ops = combiner_ops;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineFragmentShadingRateStateCreateInfoKHR {
    type LowLevel = crate::native::extensions::khr_fragment_shading_rate::PipelineFragmentShadingRateStateCreateInfoKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_fragment_shading_rate::PipelineFragmentShadingRateStateCreateInfoKHR {
            s_type: StructureType::PipelineFragmentShadingRateStateCreateInfoKhr,
            p_next: std::ptr::null(),
            fragment_size: self.fragment_size.into_low_level(context, bump),
            combiner_ops: self.combiner_ops.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineFragmentShadingRateStateCreateInfoKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            fragment_size: crate::conv::FromLowLevel::from_low_level(context, value.fragment_size),
            combiner_ops: crate::conv::FromLowLevel::from_low_level(context, value.combiner_ops),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShadingRateFeaturesKHR {
    #[doc(alias = "pipelineFragmentShadingRate")]
    pub pipeline_fragment_shading_rate: bool,
    #[doc(alias = "primitiveFragmentShadingRate")]
    pub primitive_fragment_shading_rate: bool,
    #[doc(alias = "attachmentFragmentShadingRate")]
    pub attachment_fragment_shading_rate: bool,
}
impl PhysicalDeviceFragmentShadingRateFeaturesKHR {
    ///Get a reference to the `pipeline_fragment_shading_rate` field.
    pub fn pipeline_fragment_shading_rate(&self) -> &bool {
        &self.pipeline_fragment_shading_rate
    }
    ///Get a reference to the `primitive_fragment_shading_rate` field.
    pub fn primitive_fragment_shading_rate(&self) -> &bool {
        &self.primitive_fragment_shading_rate
    }
    ///Get a reference to the `attachment_fragment_shading_rate` field.
    pub fn attachment_fragment_shading_rate(&self) -> &bool {
        &self.attachment_fragment_shading_rate
    }
    ///Get a mutable reference to the `pipeline_fragment_shading_rate` field.
    pub fn pipeline_fragment_shading_rate_mut(&mut self) -> &mut bool {
        &mut self.pipeline_fragment_shading_rate
    }
    ///Get a mutable reference to the `primitive_fragment_shading_rate` field.
    pub fn primitive_fragment_shading_rate_mut(&mut self) -> &mut bool {
        &mut self.primitive_fragment_shading_rate
    }
    ///Get a mutable reference to the `attachment_fragment_shading_rate` field.
    pub fn attachment_fragment_shading_rate_mut(&mut self) -> &mut bool {
        &mut self.attachment_fragment_shading_rate
    }
    ///Sets the `pipeline_fragment_shading_rate` field.
    pub fn set_pipeline_fragment_shading_rate(&mut self, pipeline_fragment_shading_rate: bool) -> &mut Self {
        self.pipeline_fragment_shading_rate = pipeline_fragment_shading_rate;
        self
    }
    ///Sets the `primitive_fragment_shading_rate` field.
    pub fn set_primitive_fragment_shading_rate(&mut self, primitive_fragment_shading_rate: bool) -> &mut Self {
        self.primitive_fragment_shading_rate = primitive_fragment_shading_rate;
        self
    }
    ///Sets the `attachment_fragment_shading_rate` field.
    pub fn set_attachment_fragment_shading_rate(&mut self, attachment_fragment_shading_rate: bool) -> &mut Self {
        self.attachment_fragment_shading_rate = attachment_fragment_shading_rate;
        self
    }
    ///Sets the `pipeline_fragment_shading_rate` field in a builder way.
    pub fn with_pipeline_fragment_shading_rate(mut self, pipeline_fragment_shading_rate: bool) -> Self {
        self.pipeline_fragment_shading_rate = pipeline_fragment_shading_rate;
        self
    }
    ///Sets the `primitive_fragment_shading_rate` field in a builder way.
    pub fn with_primitive_fragment_shading_rate(mut self, primitive_fragment_shading_rate: bool) -> Self {
        self.primitive_fragment_shading_rate = primitive_fragment_shading_rate;
        self
    }
    ///Sets the `attachment_fragment_shading_rate` field in a builder way.
    pub fn with_attachment_fragment_shading_rate(mut self, attachment_fragment_shading_rate: bool) -> Self {
        self.attachment_fragment_shading_rate = attachment_fragment_shading_rate;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateFeaturesKHR {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateFeaturesKhr,
            p_next: std::ptr::null_mut(),
            pipeline_fragment_shading_rate: self.pipeline_fragment_shading_rate.into_low_level(context, bump),
            primitive_fragment_shading_rate: self.primitive_fragment_shading_rate.into_low_level(context, bump),
            attachment_fragment_shading_rate: self.attachment_fragment_shading_rate.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShadingRateFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            pipeline_fragment_shading_rate: crate::conv::FromLowLevel::from_low_level(
                context,
                value.pipeline_fragment_shading_rate,
            ),
            primitive_fragment_shading_rate: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_fragment_shading_rate,
            ),
            attachment_fragment_shading_rate: crate::conv::FromLowLevel::from_low_level(
                context,
                value.attachment_fragment_shading_rate,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRatePropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShadingRatePropertiesKHR {
    #[doc(alias = "minFragmentShadingRateAttachmentTexelSize")]
    pub min_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSize")]
    pub max_fragment_shading_rate_attachment_texel_size: Extent2D,
    #[doc(alias = "maxFragmentShadingRateAttachmentTexelSizeAspectRatio")]
    pub max_fragment_shading_rate_attachment_texel_size_aspect_ratio: u32,
    #[doc(alias = "primitiveFragmentShadingRateWithMultipleViewports")]
    pub primitive_fragment_shading_rate_with_multiple_viewports: bool,
    #[doc(alias = "layeredShadingRateAttachments")]
    pub layered_shading_rate_attachments: bool,
    #[doc(alias = "fragmentShadingRateNonTrivialCombinerOps")]
    pub fragment_shading_rate_non_trivial_combiner_ops: bool,
    #[doc(alias = "maxFragmentSize")]
    pub max_fragment_size: Extent2D,
    #[doc(alias = "maxFragmentSizeAspectRatio")]
    pub max_fragment_size_aspect_ratio: u32,
    #[doc(alias = "maxFragmentShadingRateCoverageSamples")]
    pub max_fragment_shading_rate_coverage_samples: u32,
    #[doc(alias = "maxFragmentShadingRateRasterizationSamples")]
    pub max_fragment_shading_rate_rasterization_samples: SampleCountFlagBits,
    #[doc(alias = "fragmentShadingRateWithShaderDepthStencilWrites")]
    pub fragment_shading_rate_with_shader_depth_stencil_writes: bool,
    #[doc(alias = "fragmentShadingRateWithSampleMask")]
    pub fragment_shading_rate_with_sample_mask: bool,
    #[doc(alias = "fragmentShadingRateWithShaderSampleMask")]
    pub fragment_shading_rate_with_shader_sample_mask: bool,
    #[doc(alias = "fragmentShadingRateWithConservativeRasterization")]
    pub fragment_shading_rate_with_conservative_rasterization: bool,
    #[doc(alias = "fragmentShadingRateWithFragmentShaderInterlock")]
    pub fragment_shading_rate_with_fragment_shader_interlock: bool,
    #[doc(alias = "fragmentShadingRateWithCustomSampleLocations")]
    pub fragment_shading_rate_with_custom_sample_locations: bool,
    #[doc(alias = "fragmentShadingRateStrictMultiplyCombiner")]
    pub fragment_shading_rate_strict_multiply_combiner: bool,
}
impl PhysicalDeviceFragmentShadingRatePropertiesKHR {
    ///Get a reference to the `min_fragment_shading_rate_attachment_texel_size` field.
    pub fn min_fragment_shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.min_fragment_shading_rate_attachment_texel_size
    }
    ///Get a reference to the `max_fragment_shading_rate_attachment_texel_size` field.
    pub fn max_fragment_shading_rate_attachment_texel_size(&self) -> Extent2D {
        self.max_fragment_shading_rate_attachment_texel_size
    }
    ///Get a reference to the `max_fragment_shading_rate_attachment_texel_size_aspect_ratio` field.
    pub fn max_fragment_shading_rate_attachment_texel_size_aspect_ratio(&self) -> u32 {
        self.max_fragment_shading_rate_attachment_texel_size_aspect_ratio
    }
    ///Get a reference to the `primitive_fragment_shading_rate_with_multiple_viewports` field.
    pub fn primitive_fragment_shading_rate_with_multiple_viewports(&self) -> &bool {
        &self.primitive_fragment_shading_rate_with_multiple_viewports
    }
    ///Get a reference to the `layered_shading_rate_attachments` field.
    pub fn layered_shading_rate_attachments(&self) -> &bool {
        &self.layered_shading_rate_attachments
    }
    ///Get a reference to the `fragment_shading_rate_non_trivial_combiner_ops` field.
    pub fn fragment_shading_rate_non_trivial_combiner_ops(&self) -> &bool {
        &self.fragment_shading_rate_non_trivial_combiner_ops
    }
    ///Get a reference to the `max_fragment_size` field.
    pub fn max_fragment_size(&self) -> Extent2D {
        self.max_fragment_size
    }
    ///Get a reference to the `max_fragment_size_aspect_ratio` field.
    pub fn max_fragment_size_aspect_ratio(&self) -> u32 {
        self.max_fragment_size_aspect_ratio
    }
    ///Get a reference to the `max_fragment_shading_rate_coverage_samples` field.
    pub fn max_fragment_shading_rate_coverage_samples(&self) -> u32 {
        self.max_fragment_shading_rate_coverage_samples
    }
    ///Get a reference to the `max_fragment_shading_rate_rasterization_samples` field.
    pub fn max_fragment_shading_rate_rasterization_samples(&self) -> SampleCountFlagBits {
        self.max_fragment_shading_rate_rasterization_samples
    }
    ///Get a reference to the `fragment_shading_rate_with_shader_depth_stencil_writes` field.
    pub fn fragment_shading_rate_with_shader_depth_stencil_writes(&self) -> &bool {
        &self.fragment_shading_rate_with_shader_depth_stencil_writes
    }
    ///Get a reference to the `fragment_shading_rate_with_sample_mask` field.
    pub fn fragment_shading_rate_with_sample_mask(&self) -> &bool {
        &self.fragment_shading_rate_with_sample_mask
    }
    ///Get a reference to the `fragment_shading_rate_with_shader_sample_mask` field.
    pub fn fragment_shading_rate_with_shader_sample_mask(&self) -> &bool {
        &self.fragment_shading_rate_with_shader_sample_mask
    }
    ///Get a reference to the `fragment_shading_rate_with_conservative_rasterization` field.
    pub fn fragment_shading_rate_with_conservative_rasterization(&self) -> &bool {
        &self.fragment_shading_rate_with_conservative_rasterization
    }
    ///Get a reference to the `fragment_shading_rate_with_fragment_shader_interlock` field.
    pub fn fragment_shading_rate_with_fragment_shader_interlock(&self) -> &bool {
        &self.fragment_shading_rate_with_fragment_shader_interlock
    }
    ///Get a reference to the `fragment_shading_rate_with_custom_sample_locations` field.
    pub fn fragment_shading_rate_with_custom_sample_locations(&self) -> &bool {
        &self.fragment_shading_rate_with_custom_sample_locations
    }
    ///Get a reference to the `fragment_shading_rate_strict_multiply_combiner` field.
    pub fn fragment_shading_rate_strict_multiply_combiner(&self) -> &bool {
        &self.fragment_shading_rate_strict_multiply_combiner
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    type LowLevel =
        crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRatePropertiesKHR {
            s_type: StructureType::PhysicalDeviceFragmentShadingRatePropertiesKhr,
            p_next: std::ptr::null_mut(),
            min_fragment_shading_rate_attachment_texel_size: self
                .min_fragment_shading_rate_attachment_texel_size
                .into_low_level(context, bump),
            max_fragment_shading_rate_attachment_texel_size: self
                .max_fragment_shading_rate_attachment_texel_size
                .into_low_level(context, bump),
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: self
                .max_fragment_shading_rate_attachment_texel_size_aspect_ratio
                .into_low_level(context, bump),
            primitive_fragment_shading_rate_with_multiple_viewports: self
                .primitive_fragment_shading_rate_with_multiple_viewports
                .into_low_level(context, bump),
            layered_shading_rate_attachments: self.layered_shading_rate_attachments.into_low_level(context, bump),
            fragment_shading_rate_non_trivial_combiner_ops: self
                .fragment_shading_rate_non_trivial_combiner_ops
                .into_low_level(context, bump),
            max_fragment_size: self.max_fragment_size.into_low_level(context, bump),
            max_fragment_size_aspect_ratio: self.max_fragment_size_aspect_ratio.into_low_level(context, bump),
            max_fragment_shading_rate_coverage_samples: self
                .max_fragment_shading_rate_coverage_samples
                .into_low_level(context, bump),
            max_fragment_shading_rate_rasterization_samples: self
                .max_fragment_shading_rate_rasterization_samples
                .into_low_level(context, bump),
            fragment_shading_rate_with_shader_depth_stencil_writes: self
                .fragment_shading_rate_with_shader_depth_stencil_writes
                .into_low_level(context, bump),
            fragment_shading_rate_with_sample_mask: self
                .fragment_shading_rate_with_sample_mask
                .into_low_level(context, bump),
            fragment_shading_rate_with_shader_sample_mask: self
                .fragment_shading_rate_with_shader_sample_mask
                .into_low_level(context, bump),
            fragment_shading_rate_with_conservative_rasterization: self
                .fragment_shading_rate_with_conservative_rasterization
                .into_low_level(context, bump),
            fragment_shading_rate_with_fragment_shader_interlock: self
                .fragment_shading_rate_with_fragment_shader_interlock
                .into_low_level(context, bump),
            fragment_shading_rate_with_custom_sample_locations: self
                .fragment_shading_rate_with_custom_sample_locations
                .into_low_level(context, bump),
            fragment_shading_rate_strict_multiply_combiner: self
                .fragment_shading_rate_strict_multiply_combiner
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShadingRatePropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_fragment_shading_rate_attachment_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_fragment_shading_rate_attachment_texel_size,
            ),
            max_fragment_shading_rate_attachment_texel_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_shading_rate_attachment_texel_size,
            ),
            max_fragment_shading_rate_attachment_texel_size_aspect_ratio: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_shading_rate_attachment_texel_size_aspect_ratio,
            ),
            primitive_fragment_shading_rate_with_multiple_viewports: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_fragment_shading_rate_with_multiple_viewports,
            ),
            layered_shading_rate_attachments: crate::conv::FromLowLevel::from_low_level(
                context,
                value.layered_shading_rate_attachments,
            ),
            fragment_shading_rate_non_trivial_combiner_ops: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_non_trivial_combiner_ops,
            ),
            max_fragment_size: crate::conv::FromLowLevel::from_low_level(context, value.max_fragment_size),
            max_fragment_size_aspect_ratio: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_size_aspect_ratio,
            ),
            max_fragment_shading_rate_coverage_samples: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_shading_rate_coverage_samples,
            ),
            max_fragment_shading_rate_rasterization_samples: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_fragment_shading_rate_rasterization_samples,
            ),
            fragment_shading_rate_with_shader_depth_stencil_writes: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_shader_depth_stencil_writes,
            ),
            fragment_shading_rate_with_sample_mask: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_sample_mask,
            ),
            fragment_shading_rate_with_shader_sample_mask: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_shader_sample_mask,
            ),
            fragment_shading_rate_with_conservative_rasterization: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_conservative_rasterization,
            ),
            fragment_shading_rate_with_fragment_shader_interlock: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_fragment_shader_interlock,
            ),
            fragment_shading_rate_with_custom_sample_locations: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_with_custom_sample_locations,
            ),
            fragment_shading_rate_strict_multiply_combiner: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fragment_shading_rate_strict_multiply_combiner,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceFragmentShadingRateKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceFragmentShadingRateKHR {
    #[doc(alias = "sampleCounts")]
    pub sample_counts: SampleCountFlags,
    #[doc(alias = "fragmentSize")]
    pub fragment_size: Extent2D,
}
impl PhysicalDeviceFragmentShadingRateKHR {
    ///Get a reference to the `sample_counts` field.
    pub fn sample_counts(&self) -> SampleCountFlags {
        self.sample_counts
    }
    ///Get a reference to the `fragment_size` field.
    pub fn fragment_size(&self) -> Extent2D {
        self.fragment_size
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceFragmentShadingRateKHR {
    type LowLevel = crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_fragment_shading_rate::PhysicalDeviceFragmentShadingRateKHR {
            s_type: StructureType::PhysicalDeviceFragmentShadingRateKhr,
            p_next: std::ptr::null_mut(),
            sample_counts: self.sample_counts.into_low_level(context, bump),
            fragment_size: self.fragment_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceFragmentShadingRateKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            sample_counts: crate::conv::FromLowLevel::from_low_level(context, value.sample_counts),
            fragment_size: crate::conv::FromLowLevel::from_low_level(context, value.fragment_size),
        }
    }
}
