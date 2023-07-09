pub use crate::common::extensions::ext_provoking_vertex::{
    ProvokingVertexModeEXT, EXT_PROVOKING_VERTEX_EXTENSION_NAME, EXT_PROVOKING_VERTEX_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceProvokingVertexFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceProvokingVertexFeaturesEXT {
    #[doc(alias = "provokingVertexLast")]
    pub provoking_vertex_last: bool,
    #[doc(alias = "transformFeedbackPreservesProvokingVertex")]
    pub transform_feedback_preserves_provoking_vertex: bool,
}
impl PhysicalDeviceProvokingVertexFeaturesEXT {
    ///Get a reference to the `provoking_vertex_last` field.
    pub fn provoking_vertex_last(&self) -> &bool {
        &self.provoking_vertex_last
    }
    ///Get a reference to the `transform_feedback_preserves_provoking_vertex` field.
    pub fn transform_feedback_preserves_provoking_vertex(&self) -> &bool {
        &self.transform_feedback_preserves_provoking_vertex
    }
    ///Get a mutable reference to the `provoking_vertex_last` field.
    pub fn provoking_vertex_last_mut(&mut self) -> &mut bool {
        &mut self.provoking_vertex_last
    }
    ///Get a mutable reference to the `transform_feedback_preserves_provoking_vertex` field.
    pub fn transform_feedback_preserves_provoking_vertex_mut(&mut self) -> &mut bool {
        &mut self.transform_feedback_preserves_provoking_vertex
    }
    ///Sets the `provoking_vertex_last` field.
    pub fn set_provoking_vertex_last(&mut self, provoking_vertex_last: bool) -> &mut Self {
        self.provoking_vertex_last = provoking_vertex_last;
        self
    }
    ///Sets the `transform_feedback_preserves_provoking_vertex` field.
    pub fn set_transform_feedback_preserves_provoking_vertex(
        &mut self,
        transform_feedback_preserves_provoking_vertex: bool,
    ) -> &mut Self {
        self.transform_feedback_preserves_provoking_vertex = transform_feedback_preserves_provoking_vertex;
        self
    }
    ///Sets the `provoking_vertex_last` field in a builder way.
    pub fn with_provoking_vertex_last(mut self, provoking_vertex_last: bool) -> Self {
        self.provoking_vertex_last = provoking_vertex_last;
        self
    }
    ///Sets the `transform_feedback_preserves_provoking_vertex` field in a builder way.
    pub fn with_transform_feedback_preserves_provoking_vertex(
        mut self,
        transform_feedback_preserves_provoking_vertex: bool,
    ) -> Self {
        self.transform_feedback_preserves_provoking_vertex = transform_feedback_preserves_provoking_vertex;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProvokingVertexFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexFeaturesEXT {
            s_type: StructureType::PhysicalDeviceProvokingVertexFeaturesExt,
            p_next: std::ptr::null_mut(),
            provoking_vertex_last: self.provoking_vertex_last.into_low_level(context, bump),
            transform_feedback_preserves_provoking_vertex: self
                .transform_feedback_preserves_provoking_vertex
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProvokingVertexFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            provoking_vertex_last: crate::conv::FromLowLevel::from_low_level(context, value.provoking_vertex_last),
            transform_feedback_preserves_provoking_vertex: crate::conv::FromLowLevel::from_low_level(
                context,
                value.transform_feedback_preserves_provoking_vertex,
            ),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceProvokingVertexPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceProvokingVertexPropertiesEXT {
    #[doc(alias = "provokingVertexModePerPipeline")]
    pub provoking_vertex_mode_per_pipeline: bool,
    #[doc(alias = "transformFeedbackPreservesTriangleFanProvokingVertex")]
    pub transform_feedback_preserves_triangle_fan_provoking_vertex: bool,
}
impl PhysicalDeviceProvokingVertexPropertiesEXT {
    ///Get a reference to the `provoking_vertex_mode_per_pipeline` field.
    pub fn provoking_vertex_mode_per_pipeline(&self) -> &bool {
        &self.provoking_vertex_mode_per_pipeline
    }
    ///Get a reference to the `transform_feedback_preserves_triangle_fan_provoking_vertex` field.
    pub fn transform_feedback_preserves_triangle_fan_provoking_vertex(&self) -> &bool {
        &self.transform_feedback_preserves_triangle_fan_provoking_vertex
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceProvokingVertexPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_provoking_vertex::PhysicalDeviceProvokingVertexPropertiesEXT {
            s_type: StructureType::PhysicalDeviceProvokingVertexPropertiesExt,
            p_next: std::ptr::null_mut(),
            provoking_vertex_mode_per_pipeline: self.provoking_vertex_mode_per_pipeline.into_low_level(context, bump),
            transform_feedback_preserves_triangle_fan_provoking_vertex: self
                .transform_feedback_preserves_triangle_fan_provoking_vertex
                .into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceProvokingVertexPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            provoking_vertex_mode_per_pipeline: crate::conv::FromLowLevel::from_low_level(
                context,
                value.provoking_vertex_mode_per_pipeline,
            ),
            transform_feedback_preserves_triangle_fan_provoking_vertex: crate::conv::FromLowLevel::from_low_level(
                context,
                value.transform_feedback_preserves_triangle_fan_provoking_vertex,
            ),
        }
    }
}
#[doc(alias = "VkPipelineRasterizationProvokingVertexStateCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    #[doc(alias = "provokingVertexMode")]
    pub provoking_vertex_mode: ProvokingVertexModeEXT,
}
impl PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    ///Get a reference to the `provoking_vertex_mode` field.
    pub fn provoking_vertex_mode(&self) -> ProvokingVertexModeEXT {
        self.provoking_vertex_mode
    }
    ///Get a mutable reference to the `provoking_vertex_mode` field.
    pub fn provoking_vertex_mode_mut(&mut self) -> &mut ProvokingVertexModeEXT {
        &mut self.provoking_vertex_mode
    }
    ///Sets the `provoking_vertex_mode` field.
    pub fn set_provoking_vertex_mode(&mut self, provoking_vertex_mode: ProvokingVertexModeEXT) -> &mut Self {
        self.provoking_vertex_mode = provoking_vertex_mode;
        self
    }
    ///Sets the `provoking_vertex_mode` field in a builder way.
    pub fn with_provoking_vertex_mode(mut self, provoking_vertex_mode: ProvokingVertexModeEXT) -> Self {
        self.provoking_vertex_mode = provoking_vertex_mode;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    type LowLevel =
        crate::native::extensions::ext_provoking_vertex::PipelineRasterizationProvokingVertexStateCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_provoking_vertex::PipelineRasterizationProvokingVertexStateCreateInfoEXT {
            s_type: StructureType::PipelineRasterizationProvokingVertexStateCreateInfoExt,
            p_next: std::ptr::null(),
            provoking_vertex_mode: self.provoking_vertex_mode.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationProvokingVertexStateCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            provoking_vertex_mode: crate::conv::FromLowLevel::from_low_level(context, value.provoking_vertex_mode),
        }
    }
}
