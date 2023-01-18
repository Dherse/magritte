use crate::{
    cstr,
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, QueryControlFlags, QueryPool,
        StructureType,
    },
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "transformFeedback")]
    transform_feedback: Bool32,
    #[doc(alias = "geometryStreams")]
    geometry_streams: Bool32,
}
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "maxTransformFeedbackStreams")]
    max_transform_feedback_streams: u32,
    #[doc(alias = "maxTransformFeedbackBuffers")]
    max_transform_feedback_buffers: u32,
    #[doc(alias = "maxTransformFeedbackBufferSize")]
    max_transform_feedback_buffer_size: DeviceSize,
    #[doc(alias = "maxTransformFeedbackStreamDataSize")]
    max_transform_feedback_stream_data_size: u32,
    #[doc(alias = "maxTransformFeedbackBufferDataSize")]
    max_transform_feedback_buffer_data_size: u32,
    #[doc(alias = "maxTransformFeedbackBufferDataStride")]
    max_transform_feedback_buffer_data_stride: u32,
    #[doc(alias = "transformFeedbackQueries")]
    transform_feedback_queries: Bool32,
    #[doc(alias = "transformFeedbackStreamsLinesTriangles")]
    transform_feedback_streams_lines_triangles: Bool32,
    #[doc(alias = "transformFeedbackRasterizationStreamSelect")]
    transform_feedback_rasterization_stream_select: Bool32,
    #[doc(alias = "transformFeedbackDraw")]
    transform_feedback_draw: Bool32,
}
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    #[doc(alias = "rasterizationStream")]
    rasterization_stream: u32,
}
#[doc(alias = "VkPipelineRasterizationStateStreamCreateFlagsEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineRasterizationStateStreamCreateFlagsEXT(u32);
impl PipelineRasterizationStateStreamCreateFlagsEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION")]
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME")]
pub const EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_transform_feedback");
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
pub type FNCmdBindTransformFeedbackBuffersExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    p_buffers: *const Buffer,
    p_offsets: *const DeviceSize,
    p_sizes: *const DeviceSize,
);
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
pub type FNCmdBeginTransformFeedbackExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
pub type FNCmdEndTransformFeedbackExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    p_counter_buffers: *const Buffer,
    p_counter_buffer_offsets: *const DeviceSize,
);
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
pub type FNCmdBeginQueryIndexedExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
pub type FNCmdEndQueryIndexedExt =
    unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32, index: u32);
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
pub type FNCmdDrawIndirectByteCountExt = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);
