pub use crate::common::extensions::khr_portability_subset::{
    KHR_PORTABILITY_SUBSET_EXTENSION_NAME, KHR_PORTABILITY_SUBSET_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDevicePortabilitySubsetFeaturesKHR")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePortabilitySubsetFeaturesKHR {
    #[doc(alias = "constantAlphaColorBlendFactors")]
    pub constant_alpha_color_blend_factors: bool,
    pub events: bool,
    #[doc(alias = "imageViewFormatReinterpretation")]
    pub image_view_format_reinterpretation: bool,
    #[doc(alias = "imageViewFormatSwizzle")]
    pub image_view_format_swizzle: bool,
    #[doc(alias = "imageView2DOn3DImage")]
    pub image_view2_d_on3_d_image: bool,
    #[doc(alias = "multisampleArrayImage")]
    pub multisample_array_image: bool,
    #[doc(alias = "mutableComparisonSamplers")]
    pub mutable_comparison_samplers: bool,
    #[doc(alias = "pointPolygons")]
    pub point_polygons: bool,
    #[doc(alias = "samplerMipLodBias")]
    pub sampler_mip_lod_bias: bool,
    #[doc(alias = "separateStencilMaskRef")]
    pub separate_stencil_mask_ref: bool,
    #[doc(alias = "shaderSampleRateInterpolationFunctions")]
    pub shader_sample_rate_interpolation_functions: bool,
    #[doc(alias = "tessellationIsolines")]
    pub tessellation_isolines: bool,
    #[doc(alias = "tessellationPointMode")]
    pub tessellation_point_mode: bool,
    #[doc(alias = "triangleFans")]
    pub triangle_fans: bool,
    #[doc(alias = "vertexAttributeAccessBeyondStride")]
    pub vertex_attribute_access_beyond_stride: bool,
}
impl PhysicalDevicePortabilitySubsetFeaturesKHR {
    ///Get a reference to the `constant_alpha_color_blend_factors` field.
    pub fn constant_alpha_color_blend_factors(&self) -> &bool {
        &self.constant_alpha_color_blend_factors
    }
    ///Get a reference to the `events` field.
    pub fn events(&self) -> &bool {
        &self.events
    }
    ///Get a reference to the `image_view_format_reinterpretation` field.
    pub fn image_view_format_reinterpretation(&self) -> &bool {
        &self.image_view_format_reinterpretation
    }
    ///Get a reference to the `image_view_format_swizzle` field.
    pub fn image_view_format_swizzle(&self) -> &bool {
        &self.image_view_format_swizzle
    }
    ///Get a reference to the `image_view2_d_on3_d_image` field.
    pub fn image_view2_d_on3_d_image(&self) -> &bool {
        &self.image_view2_d_on3_d_image
    }
    ///Get a reference to the `multisample_array_image` field.
    pub fn multisample_array_image(&self) -> &bool {
        &self.multisample_array_image
    }
    ///Get a reference to the `mutable_comparison_samplers` field.
    pub fn mutable_comparison_samplers(&self) -> &bool {
        &self.mutable_comparison_samplers
    }
    ///Get a reference to the `point_polygons` field.
    pub fn point_polygons(&self) -> &bool {
        &self.point_polygons
    }
    ///Get a reference to the `sampler_mip_lod_bias` field.
    pub fn sampler_mip_lod_bias(&self) -> &bool {
        &self.sampler_mip_lod_bias
    }
    ///Get a reference to the `separate_stencil_mask_ref` field.
    pub fn separate_stencil_mask_ref(&self) -> &bool {
        &self.separate_stencil_mask_ref
    }
    ///Get a reference to the `shader_sample_rate_interpolation_functions` field.
    pub fn shader_sample_rate_interpolation_functions(&self) -> &bool {
        &self.shader_sample_rate_interpolation_functions
    }
    ///Get a reference to the `tessellation_isolines` field.
    pub fn tessellation_isolines(&self) -> &bool {
        &self.tessellation_isolines
    }
    ///Get a reference to the `tessellation_point_mode` field.
    pub fn tessellation_point_mode(&self) -> &bool {
        &self.tessellation_point_mode
    }
    ///Get a reference to the `triangle_fans` field.
    pub fn triangle_fans(&self) -> &bool {
        &self.triangle_fans
    }
    ///Get a reference to the `vertex_attribute_access_beyond_stride` field.
    pub fn vertex_attribute_access_beyond_stride(&self) -> &bool {
        &self.vertex_attribute_access_beyond_stride
    }
    ///Get a mutable reference to the `constant_alpha_color_blend_factors` field.
    pub fn constant_alpha_color_blend_factors_mut(&mut self) -> &mut bool {
        &mut self.constant_alpha_color_blend_factors
    }
    ///Get a mutable reference to the `events` field.
    pub fn events_mut(&mut self) -> &mut bool {
        &mut self.events
    }
    ///Get a mutable reference to the `image_view_format_reinterpretation` field.
    pub fn image_view_format_reinterpretation_mut(&mut self) -> &mut bool {
        &mut self.image_view_format_reinterpretation
    }
    ///Get a mutable reference to the `image_view_format_swizzle` field.
    pub fn image_view_format_swizzle_mut(&mut self) -> &mut bool {
        &mut self.image_view_format_swizzle
    }
    ///Get a mutable reference to the `image_view2_d_on3_d_image` field.
    pub fn image_view2_d_on3_d_image_mut(&mut self) -> &mut bool {
        &mut self.image_view2_d_on3_d_image
    }
    ///Get a mutable reference to the `multisample_array_image` field.
    pub fn multisample_array_image_mut(&mut self) -> &mut bool {
        &mut self.multisample_array_image
    }
    ///Get a mutable reference to the `mutable_comparison_samplers` field.
    pub fn mutable_comparison_samplers_mut(&mut self) -> &mut bool {
        &mut self.mutable_comparison_samplers
    }
    ///Get a mutable reference to the `point_polygons` field.
    pub fn point_polygons_mut(&mut self) -> &mut bool {
        &mut self.point_polygons
    }
    ///Get a mutable reference to the `sampler_mip_lod_bias` field.
    pub fn sampler_mip_lod_bias_mut(&mut self) -> &mut bool {
        &mut self.sampler_mip_lod_bias
    }
    ///Get a mutable reference to the `separate_stencil_mask_ref` field.
    pub fn separate_stencil_mask_ref_mut(&mut self) -> &mut bool {
        &mut self.separate_stencil_mask_ref
    }
    ///Get a mutable reference to the `shader_sample_rate_interpolation_functions` field.
    pub fn shader_sample_rate_interpolation_functions_mut(&mut self) -> &mut bool {
        &mut self.shader_sample_rate_interpolation_functions
    }
    ///Get a mutable reference to the `tessellation_isolines` field.
    pub fn tessellation_isolines_mut(&mut self) -> &mut bool {
        &mut self.tessellation_isolines
    }
    ///Get a mutable reference to the `tessellation_point_mode` field.
    pub fn tessellation_point_mode_mut(&mut self) -> &mut bool {
        &mut self.tessellation_point_mode
    }
    ///Get a mutable reference to the `triangle_fans` field.
    pub fn triangle_fans_mut(&mut self) -> &mut bool {
        &mut self.triangle_fans
    }
    ///Get a mutable reference to the `vertex_attribute_access_beyond_stride` field.
    pub fn vertex_attribute_access_beyond_stride_mut(&mut self) -> &mut bool {
        &mut self.vertex_attribute_access_beyond_stride
    }
    ///Sets the `constant_alpha_color_blend_factors` field.
    pub fn set_constant_alpha_color_blend_factors(&mut self, constant_alpha_color_blend_factors: bool) -> &mut Self {
        self.constant_alpha_color_blend_factors = constant_alpha_color_blend_factors;
        self
    }
    ///Sets the `events` field.
    pub fn set_events(&mut self, events: bool) -> &mut Self {
        self.events = events;
        self
    }
    ///Sets the `image_view_format_reinterpretation` field.
    pub fn set_image_view_format_reinterpretation(&mut self, image_view_format_reinterpretation: bool) -> &mut Self {
        self.image_view_format_reinterpretation = image_view_format_reinterpretation;
        self
    }
    ///Sets the `image_view_format_swizzle` field.
    pub fn set_image_view_format_swizzle(&mut self, image_view_format_swizzle: bool) -> &mut Self {
        self.image_view_format_swizzle = image_view_format_swizzle;
        self
    }
    ///Sets the `image_view2_d_on3_d_image` field.
    pub fn set_image_view2_d_on3_d_image(&mut self, image_view2_d_on3_d_image: bool) -> &mut Self {
        self.image_view2_d_on3_d_image = image_view2_d_on3_d_image;
        self
    }
    ///Sets the `multisample_array_image` field.
    pub fn set_multisample_array_image(&mut self, multisample_array_image: bool) -> &mut Self {
        self.multisample_array_image = multisample_array_image;
        self
    }
    ///Sets the `mutable_comparison_samplers` field.
    pub fn set_mutable_comparison_samplers(&mut self, mutable_comparison_samplers: bool) -> &mut Self {
        self.mutable_comparison_samplers = mutable_comparison_samplers;
        self
    }
    ///Sets the `point_polygons` field.
    pub fn set_point_polygons(&mut self, point_polygons: bool) -> &mut Self {
        self.point_polygons = point_polygons;
        self
    }
    ///Sets the `sampler_mip_lod_bias` field.
    pub fn set_sampler_mip_lod_bias(&mut self, sampler_mip_lod_bias: bool) -> &mut Self {
        self.sampler_mip_lod_bias = sampler_mip_lod_bias;
        self
    }
    ///Sets the `separate_stencil_mask_ref` field.
    pub fn set_separate_stencil_mask_ref(&mut self, separate_stencil_mask_ref: bool) -> &mut Self {
        self.separate_stencil_mask_ref = separate_stencil_mask_ref;
        self
    }
    ///Sets the `shader_sample_rate_interpolation_functions` field.
    pub fn set_shader_sample_rate_interpolation_functions(
        &mut self,
        shader_sample_rate_interpolation_functions: bool,
    ) -> &mut Self {
        self.shader_sample_rate_interpolation_functions = shader_sample_rate_interpolation_functions;
        self
    }
    ///Sets the `tessellation_isolines` field.
    pub fn set_tessellation_isolines(&mut self, tessellation_isolines: bool) -> &mut Self {
        self.tessellation_isolines = tessellation_isolines;
        self
    }
    ///Sets the `tessellation_point_mode` field.
    pub fn set_tessellation_point_mode(&mut self, tessellation_point_mode: bool) -> &mut Self {
        self.tessellation_point_mode = tessellation_point_mode;
        self
    }
    ///Sets the `triangle_fans` field.
    pub fn set_triangle_fans(&mut self, triangle_fans: bool) -> &mut Self {
        self.triangle_fans = triangle_fans;
        self
    }
    ///Sets the `vertex_attribute_access_beyond_stride` field.
    pub fn set_vertex_attribute_access_beyond_stride(
        &mut self,
        vertex_attribute_access_beyond_stride: bool,
    ) -> &mut Self {
        self.vertex_attribute_access_beyond_stride = vertex_attribute_access_beyond_stride;
        self
    }
    ///Sets the `constant_alpha_color_blend_factors` field in a builder way.
    pub fn with_constant_alpha_color_blend_factors(mut self, constant_alpha_color_blend_factors: bool) -> Self {
        self.constant_alpha_color_blend_factors = constant_alpha_color_blend_factors;
        self
    }
    ///Sets the `events` field in a builder way.
    pub fn with_events(mut self, events: bool) -> Self {
        self.events = events;
        self
    }
    ///Sets the `image_view_format_reinterpretation` field in a builder way.
    pub fn with_image_view_format_reinterpretation(mut self, image_view_format_reinterpretation: bool) -> Self {
        self.image_view_format_reinterpretation = image_view_format_reinterpretation;
        self
    }
    ///Sets the `image_view_format_swizzle` field in a builder way.
    pub fn with_image_view_format_swizzle(mut self, image_view_format_swizzle: bool) -> Self {
        self.image_view_format_swizzle = image_view_format_swizzle;
        self
    }
    ///Sets the `image_view2_d_on3_d_image` field in a builder way.
    pub fn with_image_view2_d_on3_d_image(mut self, image_view2_d_on3_d_image: bool) -> Self {
        self.image_view2_d_on3_d_image = image_view2_d_on3_d_image;
        self
    }
    ///Sets the `multisample_array_image` field in a builder way.
    pub fn with_multisample_array_image(mut self, multisample_array_image: bool) -> Self {
        self.multisample_array_image = multisample_array_image;
        self
    }
    ///Sets the `mutable_comparison_samplers` field in a builder way.
    pub fn with_mutable_comparison_samplers(mut self, mutable_comparison_samplers: bool) -> Self {
        self.mutable_comparison_samplers = mutable_comparison_samplers;
        self
    }
    ///Sets the `point_polygons` field in a builder way.
    pub fn with_point_polygons(mut self, point_polygons: bool) -> Self {
        self.point_polygons = point_polygons;
        self
    }
    ///Sets the `sampler_mip_lod_bias` field in a builder way.
    pub fn with_sampler_mip_lod_bias(mut self, sampler_mip_lod_bias: bool) -> Self {
        self.sampler_mip_lod_bias = sampler_mip_lod_bias;
        self
    }
    ///Sets the `separate_stencil_mask_ref` field in a builder way.
    pub fn with_separate_stencil_mask_ref(mut self, separate_stencil_mask_ref: bool) -> Self {
        self.separate_stencil_mask_ref = separate_stencil_mask_ref;
        self
    }
    ///Sets the `shader_sample_rate_interpolation_functions` field in a builder way.
    pub fn with_shader_sample_rate_interpolation_functions(
        mut self,
        shader_sample_rate_interpolation_functions: bool,
    ) -> Self {
        self.shader_sample_rate_interpolation_functions = shader_sample_rate_interpolation_functions;
        self
    }
    ///Sets the `tessellation_isolines` field in a builder way.
    pub fn with_tessellation_isolines(mut self, tessellation_isolines: bool) -> Self {
        self.tessellation_isolines = tessellation_isolines;
        self
    }
    ///Sets the `tessellation_point_mode` field in a builder way.
    pub fn with_tessellation_point_mode(mut self, tessellation_point_mode: bool) -> Self {
        self.tessellation_point_mode = tessellation_point_mode;
        self
    }
    ///Sets the `triangle_fans` field in a builder way.
    pub fn with_triangle_fans(mut self, triangle_fans: bool) -> Self {
        self.triangle_fans = triangle_fans;
        self
    }
    ///Sets the `vertex_attribute_access_beyond_stride` field in a builder way.
    pub fn with_vertex_attribute_access_beyond_stride(mut self, vertex_attribute_access_beyond_stride: bool) -> Self {
        self.vertex_attribute_access_beyond_stride = vertex_attribute_access_beyond_stride;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePortabilitySubsetFeaturesKHR {
    type LowLevel = crate::native::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetFeaturesKHR {
            s_type: StructureType::PhysicalDevicePortabilitySubsetFeaturesKhr,
            p_next: std::ptr::null_mut(),
            constant_alpha_color_blend_factors: self.constant_alpha_color_blend_factors.into_low_level(context, bump),
            events: self.events.into_low_level(context, bump),
            image_view_format_reinterpretation: self.image_view_format_reinterpretation.into_low_level(context, bump),
            image_view_format_swizzle: self.image_view_format_swizzle.into_low_level(context, bump),
            image_view2_d_on3_d_image: self.image_view2_d_on3_d_image.into_low_level(context, bump),
            multisample_array_image: self.multisample_array_image.into_low_level(context, bump),
            mutable_comparison_samplers: self.mutable_comparison_samplers.into_low_level(context, bump),
            point_polygons: self.point_polygons.into_low_level(context, bump),
            sampler_mip_lod_bias: self.sampler_mip_lod_bias.into_low_level(context, bump),
            separate_stencil_mask_ref: self.separate_stencil_mask_ref.into_low_level(context, bump),
            shader_sample_rate_interpolation_functions: self
                .shader_sample_rate_interpolation_functions
                .into_low_level(context, bump),
            tessellation_isolines: self.tessellation_isolines.into_low_level(context, bump),
            tessellation_point_mode: self.tessellation_point_mode.into_low_level(context, bump),
            triangle_fans: self.triangle_fans.into_low_level(context, bump),
            vertex_attribute_access_beyond_stride: self
                .vertex_attribute_access_beyond_stride
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePortabilitySubsetFeaturesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            constant_alpha_color_blend_factors: crate::conv::FromLowLevel::from_low_level(
                context,
                value.constant_alpha_color_blend_factors,
            ),
            events: crate::conv::FromLowLevel::from_low_level(context, value.events),
            image_view_format_reinterpretation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.image_view_format_reinterpretation,
            ),
            image_view_format_swizzle: crate::conv::FromLowLevel::from_low_level(
                context,
                value.image_view_format_swizzle,
            ),
            image_view2_d_on3_d_image: crate::conv::FromLowLevel::from_low_level(
                context,
                value.image_view2_d_on3_d_image,
            ),
            multisample_array_image: crate::conv::FromLowLevel::from_low_level(context, value.multisample_array_image),
            mutable_comparison_samplers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.mutable_comparison_samplers,
            ),
            point_polygons: crate::conv::FromLowLevel::from_low_level(context, value.point_polygons),
            sampler_mip_lod_bias: crate::conv::FromLowLevel::from_low_level(context, value.sampler_mip_lod_bias),
            separate_stencil_mask_ref: crate::conv::FromLowLevel::from_low_level(
                context,
                value.separate_stencil_mask_ref,
            ),
            shader_sample_rate_interpolation_functions: crate::conv::FromLowLevel::from_low_level(
                context,
                value.shader_sample_rate_interpolation_functions,
            ),
            tessellation_isolines: crate::conv::FromLowLevel::from_low_level(context, value.tessellation_isolines),
            tessellation_point_mode: crate::conv::FromLowLevel::from_low_level(context, value.tessellation_point_mode),
            triangle_fans: crate::conv::FromLowLevel::from_low_level(context, value.triangle_fans),
            vertex_attribute_access_beyond_stride: crate::conv::FromLowLevel::from_low_level(
                context,
                value.vertex_attribute_access_beyond_stride,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDevicePortabilitySubsetPropertiesKHR")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDevicePortabilitySubsetPropertiesKHR {
    #[doc(alias = "minVertexInputBindingStrideAlignment")]
    pub min_vertex_input_binding_stride_alignment: u32,
}
impl PhysicalDevicePortabilitySubsetPropertiesKHR {
    ///Get a reference to the `min_vertex_input_binding_stride_alignment` field.
    pub fn min_vertex_input_binding_stride_alignment(&self) -> u32 {
        self.min_vertex_input_binding_stride_alignment
    }
    ///Get a mutable reference to the `min_vertex_input_binding_stride_alignment` field.
    pub fn min_vertex_input_binding_stride_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_vertex_input_binding_stride_alignment
    }
    ///Sets the `min_vertex_input_binding_stride_alignment` field.
    pub fn set_min_vertex_input_binding_stride_alignment(
        &mut self,
        min_vertex_input_binding_stride_alignment: u32,
    ) -> &mut Self {
        self.min_vertex_input_binding_stride_alignment = min_vertex_input_binding_stride_alignment;
        self
    }
    ///Sets the `min_vertex_input_binding_stride_alignment` field in a builder way.
    pub fn with_min_vertex_input_binding_stride_alignment(
        mut self,
        min_vertex_input_binding_stride_alignment: u32,
    ) -> Self {
        self.min_vertex_input_binding_stride_alignment = min_vertex_input_binding_stride_alignment;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDevicePortabilitySubsetPropertiesKHR {
    type LowLevel = crate::native::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::khr_portability_subset::PhysicalDevicePortabilitySubsetPropertiesKHR {
            s_type: StructureType::PhysicalDevicePortabilitySubsetPropertiesKhr,
            p_next: std::ptr::null_mut(),
            min_vertex_input_binding_stride_alignment: self
                .min_vertex_input_binding_stride_alignment
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDevicePortabilitySubsetPropertiesKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            min_vertex_input_binding_stride_alignment: crate::conv::FromLowLevel::from_low_level(
                context,
                value.min_vertex_input_binding_stride_alignment,
            ),
        }
    }
}
