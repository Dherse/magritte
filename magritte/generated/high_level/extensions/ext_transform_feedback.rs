pub use crate::common::extensions::ext_transform_feedback::{
    PipelineRasterizationStateStreamCreateFlagsEXT, EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME,
    EXT_TRANSFORM_FEEDBACK_SPEC_VERSION,
};
use crate::{
    context::Context,
    vulkan1_0::{DeviceSize, StructureType},
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::sync::Arc;
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    #[doc(alias = "transformFeedback")]
    pub transform_feedback: bool,
    #[doc(alias = "geometryStreams")]
    pub geometry_streams: bool,
}
impl PhysicalDeviceTransformFeedbackFeaturesEXT {
    ///Get a reference to the `transform_feedback` field.
    pub fn transform_feedback(&self) -> &bool {
        &self.transform_feedback
    }
    ///Get a reference to the `geometry_streams` field.
    pub fn geometry_streams(&self) -> &bool {
        &self.geometry_streams
    }
    ///Get a mutable reference to the `transform_feedback` field.
    pub fn transform_feedback_mut(&mut self) -> &mut bool {
        &mut self.transform_feedback
    }
    ///Get a mutable reference to the `geometry_streams` field.
    pub fn geometry_streams_mut(&mut self) -> &mut bool {
        &mut self.geometry_streams
    }
    ///Sets the `transform_feedback` field.
    pub fn set_transform_feedback(&mut self, transform_feedback: bool) -> &mut Self {
        self.transform_feedback = transform_feedback;
        self
    }
    ///Sets the `geometry_streams` field.
    pub fn set_geometry_streams(&mut self, geometry_streams: bool) -> &mut Self {
        self.geometry_streams = geometry_streams;
        self
    }
    ///Sets the `transform_feedback` field in a builder way.
    pub fn with_transform_feedback(mut self, transform_feedback: bool) -> Self {
        self.transform_feedback = transform_feedback;
        self
    }
    ///Sets the `geometry_streams` field in a builder way.
    pub fn with_geometry_streams(mut self, geometry_streams: bool) -> Self {
        self.geometry_streams = geometry_streams;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTransformFeedbackFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackFeaturesEXT {
            s_type: StructureType::PhysicalDeviceTransformFeedbackFeaturesExt,
            p_next: std::ptr::null_mut(),
            transform_feedback: self.transform_feedback.into_low_level(context, bump),
            geometry_streams: self.geometry_streams.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTransformFeedbackFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            transform_feedback: crate::conv::FromLowLevel::from_low_level(context, value.transform_feedback),
            geometry_streams: crate::conv::FromLowLevel::from_low_level(context, value.geometry_streams),
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    #[doc(alias = "maxTransformFeedbackStreams")]
    pub max_transform_feedback_streams: u32,
    #[doc(alias = "maxTransformFeedbackBuffers")]
    pub max_transform_feedback_buffers: u32,
    #[doc(alias = "maxTransformFeedbackBufferSize")]
    pub max_transform_feedback_buffer_size: DeviceSize,
    #[doc(alias = "maxTransformFeedbackStreamDataSize")]
    pub max_transform_feedback_stream_data_size: u32,
    #[doc(alias = "maxTransformFeedbackBufferDataSize")]
    pub max_transform_feedback_buffer_data_size: u32,
    #[doc(alias = "maxTransformFeedbackBufferDataStride")]
    pub max_transform_feedback_buffer_data_stride: u32,
    #[doc(alias = "transformFeedbackQueries")]
    pub transform_feedback_queries: bool,
    #[doc(alias = "transformFeedbackStreamsLinesTriangles")]
    pub transform_feedback_streams_lines_triangles: bool,
    #[doc(alias = "transformFeedbackRasterizationStreamSelect")]
    pub transform_feedback_rasterization_stream_select: bool,
    #[doc(alias = "transformFeedbackDraw")]
    pub transform_feedback_draw: bool,
}
impl PhysicalDeviceTransformFeedbackPropertiesEXT {
    ///Get a reference to the `max_transform_feedback_streams` field.
    pub fn max_transform_feedback_streams(&self) -> u32 {
        self.max_transform_feedback_streams
    }
    ///Get a reference to the `max_transform_feedback_buffers` field.
    pub fn max_transform_feedback_buffers(&self) -> u32 {
        self.max_transform_feedback_buffers
    }
    ///Get a reference to the `max_transform_feedback_buffer_size` field.
    pub fn max_transform_feedback_buffer_size(&self) -> &DeviceSize {
        &self.max_transform_feedback_buffer_size
    }
    ///Get a reference to the `max_transform_feedback_stream_data_size` field.
    pub fn max_transform_feedback_stream_data_size(&self) -> u32 {
        self.max_transform_feedback_stream_data_size
    }
    ///Get a reference to the `max_transform_feedback_buffer_data_size` field.
    pub fn max_transform_feedback_buffer_data_size(&self) -> u32 {
        self.max_transform_feedback_buffer_data_size
    }
    ///Get a reference to the `max_transform_feedback_buffer_data_stride` field.
    pub fn max_transform_feedback_buffer_data_stride(&self) -> u32 {
        self.max_transform_feedback_buffer_data_stride
    }
    ///Get a reference to the `transform_feedback_queries` field.
    pub fn transform_feedback_queries(&self) -> &bool {
        &self.transform_feedback_queries
    }
    ///Get a reference to the `transform_feedback_streams_lines_triangles` field.
    pub fn transform_feedback_streams_lines_triangles(&self) -> &bool {
        &self.transform_feedback_streams_lines_triangles
    }
    ///Get a reference to the `transform_feedback_rasterization_stream_select` field.
    pub fn transform_feedback_rasterization_stream_select(&self) -> &bool {
        &self.transform_feedback_rasterization_stream_select
    }
    ///Get a reference to the `transform_feedback_draw` field.
    pub fn transform_feedback_draw(&self) -> &bool {
        &self.transform_feedback_draw
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PhysicalDeviceTransformFeedbackPropertiesEXT {
    type LowLevel = crate::native::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_transform_feedback::PhysicalDeviceTransformFeedbackPropertiesEXT {
            s_type: StructureType::PhysicalDeviceTransformFeedbackPropertiesExt,
            p_next: std::ptr::null_mut(),
            max_transform_feedback_streams: self.max_transform_feedback_streams.into_low_level(context, bump),
            max_transform_feedback_buffers: self.max_transform_feedback_buffers.into_low_level(context, bump),
            max_transform_feedback_buffer_size: self.max_transform_feedback_buffer_size.into_low_level(context, bump),
            max_transform_feedback_stream_data_size: self
                .max_transform_feedback_stream_data_size
                .into_low_level(context, bump),
            max_transform_feedback_buffer_data_size: self
                .max_transform_feedback_buffer_data_size
                .into_low_level(context, bump),
            max_transform_feedback_buffer_data_stride: self
                .max_transform_feedback_buffer_data_stride
                .into_low_level(context, bump),
            transform_feedback_queries: self.transform_feedback_queries.into_low_level(context, bump),
            transform_feedback_streams_lines_triangles: self
                .transform_feedback_streams_lines_triangles
                .into_low_level(context, bump),
            transform_feedback_rasterization_stream_select: self
                .transform_feedback_rasterization_stream_select
                .into_low_level(context, bump),
            transform_feedback_draw: self.transform_feedback_draw.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PhysicalDeviceTransformFeedbackPropertiesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            max_transform_feedback_streams: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_streams,
            ),
            max_transform_feedback_buffers: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_buffers,
            ),
            max_transform_feedback_buffer_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_buffer_size,
            ),
            max_transform_feedback_stream_data_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_stream_data_size,
            ),
            max_transform_feedback_buffer_data_size: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_buffer_data_size,
            ),
            max_transform_feedback_buffer_data_stride: crate::conv::FromLowLevel::from_low_level(
                context,
                value.max_transform_feedback_buffer_data_stride,
            ),
            transform_feedback_queries: crate::conv::FromLowLevel::from_low_level(
                context,
                value.transform_feedback_queries,
            ),
            transform_feedback_streams_lines_triangles: crate::conv::FromLowLevel::from_low_level(
                context,
                value.transform_feedback_streams_lines_triangles,
            ),
            transform_feedback_rasterization_stream_select: crate::conv::FromLowLevel::from_low_level(
                context,
                value.transform_feedback_rasterization_stream_select,
            ),
            transform_feedback_draw: crate::conv::FromLowLevel::from_low_level(context, value.transform_feedback_draw),
        }
    }
}
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
#[derive(Clone, PartialEq, Debug, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    #[doc(alias = "rasterizationStream")]
    pub rasterization_stream: u32,
}
impl PipelineRasterizationStateStreamCreateInfoEXT {
    ///Get a reference to the `flags` field.
    pub fn flags(&self) -> PipelineRasterizationStateStreamCreateFlagsEXT {
        self.flags
    }
    ///Get a reference to the `rasterization_stream` field.
    pub fn rasterization_stream(&self) -> u32 {
        self.rasterization_stream
    }
    ///Get a mutable reference to the `flags` field.
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationStateStreamCreateFlagsEXT {
        &mut self.flags
    }
    ///Get a mutable reference to the `rasterization_stream` field.
    pub fn rasterization_stream_mut(&mut self) -> &mut u32 {
        &mut self.rasterization_stream
    }
    ///Sets the `flags` field.
    pub fn set_flags(&mut self, flags: PipelineRasterizationStateStreamCreateFlagsEXT) -> &mut Self {
        self.flags = flags;
        self
    }
    ///Sets the `rasterization_stream` field.
    pub fn set_rasterization_stream(&mut self, rasterization_stream: u32) -> &mut Self {
        self.rasterization_stream = rasterization_stream;
        self
    }
    ///Sets the `flags` field in a builder way.
    pub fn with_flags(mut self, flags: PipelineRasterizationStateStreamCreateFlagsEXT) -> Self {
        self.flags = flags;
        self
    }
    ///Sets the `rasterization_stream` field in a builder way.
    pub fn with_rasterization_stream(mut self, rasterization_stream: u32) -> Self {
        self.rasterization_stream = rasterization_stream;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineRasterizationStateStreamCreateInfoEXT {
    type LowLevel = crate::native::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateInfoEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        crate::native::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateInfoEXT {
            s_type: StructureType::PipelineRasterizationStateStreamCreateInfoExt,
            p_next: std::ptr::null(),
            flags: self.flags.into_low_level(context, bump),
            rasterization_stream: self.rasterization_stream.into_low_level(context, bump),
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for PipelineRasterizationStateStreamCreateInfoEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        Self {
            flags: crate::conv::FromLowLevel::from_low_level(context, value.flags),
            rasterization_stream: crate::conv::FromLowLevel::from_low_level(context, value.rasterization_stream),
        }
    }
}
