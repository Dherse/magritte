use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, DeviceSize, QueryControlFlags, QueryPool,
    StructureType,
};
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "transformFeedback")]
    pub transform_feedback: Bool32,
    #[doc(alias = "geometryStreams")]
    pub geometry_streams: Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTransformFeedbackFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            transform_feedback: unsafe { std::mem::zeroed() },
            geometry_streams: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
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
    pub transform_feedback_queries: Bool32,
    #[doc(alias = "transformFeedbackStreamsLinesTriangles")]
    pub transform_feedback_streams_lines_triangles: Bool32,
    #[doc(alias = "transformFeedbackRasterizationStreamSelect")]
    pub transform_feedback_rasterization_stream_select: Bool32,
    #[doc(alias = "transformFeedbackDraw")]
    pub transform_feedback_draw: Bool32,
}
impl Default for PhysicalDeviceTransformFeedbackPropertiesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceTransformFeedbackPropertiesExt,
            p_next: unsafe { std::mem::zeroed() },
            max_transform_feedback_streams: unsafe { std::mem::zeroed() },
            max_transform_feedback_buffers: unsafe { std::mem::zeroed() },
            max_transform_feedback_buffer_size: unsafe { std::mem::zeroed() },
            max_transform_feedback_stream_data_size: unsafe { std::mem::zeroed() },
            max_transform_feedback_buffer_data_size: unsafe { std::mem::zeroed() },
            max_transform_feedback_buffer_data_stride: unsafe { std::mem::zeroed() },
            transform_feedback_queries: unsafe { std::mem::zeroed() },
            transform_feedback_streams_lines_triangles: unsafe { std::mem::zeroed() },
            transform_feedback_rasterization_stream_select: unsafe { std::mem::zeroed() },
            transform_feedback_draw: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    #[doc(alias = "rasterizationStream")]
    pub rasterization_stream: u32,
}
impl Default for PipelineRasterizationStateStreamCreateInfoEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::PipelineRasterizationStateStreamCreateInfoExt,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            rasterization_stream: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_transform_feedback::{
    PipelineRasterizationStateStreamCreateFlagsEXT, EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME,
    EXT_TRANSFORM_FEEDBACK_SPEC_VERSION,
};
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
