pub use crate::common::extensions::ext_conservative_rasterization::{
    ConservativeRasterizationModeEXT, PipelineRasterizationConservativeStateCreateFlagsEXT,
    EXT_CONSERVATIVE_RASTERIZATION_EXTENSION_NAME, EXT_CONSERVATIVE_RASTERIZATION_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceConservativeRasterizationPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceConservativeRasterizationPropertiesEXT {
    #[doc(alias = "primitiveOverestimationSize")]
    pub primitive_overestimation_size: f32,
    #[doc(alias = "maxExtraPrimitiveOverestimationSize")]
    pub max_extra_primitive_overestimation_size: f32,
    #[doc(alias = "extraPrimitiveOverestimationSizeGranularity")]
    pub extra_primitive_overestimation_size_granularity: f32,
    #[doc(alias = "primitiveUnderestimation")]
    pub primitive_underestimation: bool,
    #[doc(alias = "conservativePointAndLineRasterization")]
    pub conservative_point_and_line_rasterization: bool,
    #[doc(alias = "degenerateTrianglesRasterized")]
    pub degenerate_triangles_rasterized: bool,
    #[doc(alias = "degenerateLinesRasterized")]
    pub degenerate_lines_rasterized: bool,
    #[doc(alias = "fullyCoveredFragmentShaderInputVariable")]
    pub fully_covered_fragment_shader_input_variable: bool,
    #[doc(alias = "conservativeRasterizationPostDepthCoverage")]
    pub conservative_rasterization_post_depth_coverage: bool,
}
impl PhysicalDeviceConservativeRasterizationPropertiesEXT {
    ///Get a reference to the `primitive_overestimation_size` field.
    pub fn primitive_overestimation_size(&self) -> f32 {
        self.primitive_overestimation_size
    }
    ///Get a reference to the `max_extra_primitive_overestimation_size` field.
    pub fn max_extra_primitive_overestimation_size(&self) -> f32 {
        self.max_extra_primitive_overestimation_size
    }
    ///Get a reference to the `extra_primitive_overestimation_size_granularity` field.
    pub fn extra_primitive_overestimation_size_granularity(&self) -> f32 {
        self.extra_primitive_overestimation_size_granularity
    }
    ///Get a reference to the `primitive_underestimation` field.
    pub fn primitive_underestimation(&self) -> &bool {
        &self.primitive_underestimation
    }
    ///Get a reference to the `conservative_point_and_line_rasterization` field.
    pub fn conservative_point_and_line_rasterization(&self) -> &bool {
        &self.conservative_point_and_line_rasterization
    }
    ///Get a reference to the `degenerate_triangles_rasterized` field.
    pub fn degenerate_triangles_rasterized(&self) -> &bool {
        &self.degenerate_triangles_rasterized
    }
    ///Get a reference to the `degenerate_lines_rasterized` field.
    pub fn degenerate_lines_rasterized(&self) -> &bool {
        &self.degenerate_lines_rasterized
    }
    ///Get a reference to the `fully_covered_fragment_shader_input_variable` field.
    pub fn fully_covered_fragment_shader_input_variable(&self) -> &bool {
        &self.fully_covered_fragment_shader_input_variable
    }
    ///Get a reference to the `conservative_rasterization_post_depth_coverage` field.
    pub fn conservative_rasterization_post_depth_coverage(&self) -> &bool {
        &self.conservative_rasterization_post_depth_coverage
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    type LowLevel =
        crate::native::extensions::ext_conservative_rasterization::PhysicalDeviceConservativeRasterizationPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate :: native :: extensions :: ext_conservative_rasterization :: PhysicalDeviceConservativeRasterizationPropertiesEXT { s_type : StructureType :: PhysicalDeviceConservativeRasterizationPropertiesExt , p_next : std :: ptr :: null_mut () , primitive_overestimation_size : self . primitive_overestimation_size . into_low_level (context , bump) , max_extra_primitive_overestimation_size : self . max_extra_primitive_overestimation_size . into_low_level (context , bump) , extra_primitive_overestimation_size_granularity : self . extra_primitive_overestimation_size_granularity . into_low_level (context , bump) , primitive_underestimation : self . primitive_underestimation . into_low_level (context , bump) , conservative_point_and_line_rasterization : self . conservative_point_and_line_rasterization . into_low_level (context , bump) , degenerate_triangles_rasterized : self . degenerate_triangles_rasterized . into_low_level (context , bump) , degenerate_lines_rasterized : self . degenerate_lines_rasterized . into_low_level (context , bump) , fully_covered_fragment_shader_input_variable : self . fully_covered_fragment_shader_input_variable . into_low_level (context , bump) , conservative_rasterization_post_depth_coverage : self . conservative_rasterization_post_depth_coverage . into_low_level (context , bump) }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceConservativeRasterizationPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            primitive_overestimation_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_overestimation_size,
            ),
            max_extra_primitive_overestimation_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_extra_primitive_overestimation_size,
            ),
            extra_primitive_overestimation_size_granularity: crate::conv::FromLowLevel::from_low_level(
                context,
                value.extra_primitive_overestimation_size_granularity,
            ),
            primitive_underestimation: crate::conv::FromLowLevel::from_low_level(
                context,
                value.primitive_underestimation,
            ),
            conservative_point_and_line_rasterization: crate::conv::FromLowLevel::from_low_level(
                context,
                value.conservative_point_and_line_rasterization,
            ),
            degenerate_triangles_rasterized: crate::conv::FromLowLevel::from_low_level(
                context,
                value.degenerate_triangles_rasterized,
            ),
            degenerate_lines_rasterized: crate::conv::FromLowLevel::from_low_level(
                context,
                value.degenerate_lines_rasterized,
            ),
            fully_covered_fragment_shader_input_variable: crate::conv::FromLowLevel::from_low_level(
                context,
                value.fully_covered_fragment_shader_input_variable,
            ),
            conservative_rasterization_post_depth_coverage: crate::conv::FromLowLevel::from_low_level(
                context,
                value.conservative_rasterization_post_depth_coverage,
            ),
        }
    }
}
#[doc(alias = "VkPipelineRasterizationConservativeStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationConservativeStateCreateInfoEXT {
    pub flags: PipelineRasterizationConservativeStateCreateFlagsEXT,
    #[doc(alias = "conservativeRasterizationMode")]
    pub conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    #[doc(alias = "extraPrimitiveOverestimationSize")]
    pub extra_primitive_overestimation_size: f32,
}
impl PipelineRasterizationConservativeStateCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineRasterizationConservativeStateCreateFlagsEXT {
        self.flags
    }
    ///Get a reference to the `conservative_rasterization_mode` field.
    pub fn conservative_rasterization_mode(&self) -> ConservativeRasterizationModeEXT {
        self.conservative_rasterization_mode
    }
    ///Get a reference to the `extra_primitive_overestimation_size` field.
    pub fn extra_primitive_overestimation_size(&self) -> f32 {
        self.extra_primitive_overestimation_size
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationConservativeStateCreateFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `conservative_rasterization_mode` field.
    pub fn conservative_rasterization_mode_mut(&mut self) -> &mut ConservativeRasterizationModeEXT {
        &mut self.conservative_rasterization_mode
    }
    ///Get a mutable reference to the `extra_primitive_overestimation_size` field.
    pub fn extra_primitive_overestimation_size_mut(&mut self) -> &mut f32 {
        &mut self.extra_primitive_overestimation_size
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineRasterizationConservativeStateCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `conservative_rasterization_mode` field.
    pub fn set_conservative_rasterization_mode(
        &mut self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) -> &mut Self {
        self.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    ///Sets the `extra_primitive_overestimation_size` field.
    pub fn set_extra_primitive_overestimation_size(&mut self, extra_primitive_overestimation_size: f32) -> &mut Self {
        self.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineRasterizationConservativeStateCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `conservative_rasterization_mode` field in a builder way.
    pub fn with_conservative_rasterization_mode(
        mut self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) -> Self {
        self.conservative_rasterization_mode = conservative_rasterization_mode;
        self
    }
    ///Sets the `extra_primitive_overestimation_size` field in a builder way.
    pub fn with_extra_primitive_overestimation_size(mut self, extra_primitive_overestimation_size: f32) -> Self {
        self.extra_primitive_overestimation_size = extra_primitive_overestimation_size;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationConservativeStateCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_conservative_rasterization::PipelineRasterizationConservativeStateCreateInfoEXT {
            s_type: StructureType::PipelineRasterizationConservativeStateCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            conservative_rasterization_mode: self.conservative_rasterization_mode.into_low_level(context, bump),
            extra_primitive_overestimation_size: self.extra_primitive_overestimation_size.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationConservativeStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            conservative_rasterization_mode: crate::conv::FromLowLevel::from_low_level(
                context,
                value.conservative_rasterization_mode,
            ),
            extra_primitive_overestimation_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.extra_primitive_overestimation_size,
            ),
        }
    }
}
