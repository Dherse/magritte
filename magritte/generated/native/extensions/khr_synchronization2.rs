use crate::native::{
    vulkan1_0::{BaseOutStructure, Buffer, CommandBuffer, DeviceSize, Queue, StructureType},
    vulkan1_3::{
        AccessFlagBits2, AccessFlags2, BufferMemoryBarrier2, CommandBufferSubmitInfo, DependencyInfo,
        FNCmdPipelineBarrier2, FNCmdResetEvent2, FNCmdSetEvent2, FNCmdWaitEvents2, FNCmdWriteTimestamp2,
        FNQueueSubmit2, ImageMemoryBarrier2, MemoryBarrier2, PhysicalDeviceSynchronization2Features,
        PipelineStageFlagBits2, PipelineStageFlags2, SemaphoreSubmitInfo, SubmitFlagBits, SubmitFlags, SubmitInfo2,
    },
};
///See [`AccessFlags2`]
#[doc(alias = "VkAccessFlags2KHR")]
pub type AccessFlags2KHR = AccessFlags2;
///See [`PipelineStageFlags2`]
#[doc(alias = "VkPipelineStageFlags2KHR")]
pub type PipelineStageFlags2KHR = PipelineStageFlags2;
///See [`SubmitFlags`]
#[doc(alias = "VkSubmitFlagsKHR")]
pub type SubmitFlagsKHR = SubmitFlags;
///See [`AccessFlagBits2`]
#[doc(alias = "VkAccessFlagBits2KHR")]
pub type AccessFlagBits2KHR = AccessFlagBits2;
///See [`PipelineStageFlagBits2`]
#[doc(alias = "VkPipelineStageFlagBits2KHR")]
pub type PipelineStageFlagBits2KHR = PipelineStageFlagBits2;
///See [`SubmitFlagBits`]
#[doc(alias = "VkSubmitFlagBitsKHR")]
pub type SubmitFlagBitsKHR = SubmitFlagBits;
///See [`MemoryBarrier2`]
#[doc(alias = "VkMemoryBarrier2KHR")]
pub type MemoryBarrier2KHR = MemoryBarrier2;
///See [`ImageMemoryBarrier2`]
#[doc(alias = "VkImageMemoryBarrier2KHR")]
pub type ImageMemoryBarrier2KHR = ImageMemoryBarrier2;
///See [`BufferMemoryBarrier2`]
#[doc(alias = "VkBufferMemoryBarrier2KHR")]
pub type BufferMemoryBarrier2KHR = BufferMemoryBarrier2;
///See [`DependencyInfo`]
#[doc(alias = "VkDependencyInfoKHR")]
pub type DependencyInfoKHR = DependencyInfo;
///See [`SemaphoreSubmitInfo`]
#[doc(alias = "VkSemaphoreSubmitInfoKHR")]
pub type SemaphoreSubmitInfoKHR = SemaphoreSubmitInfo;
///See [`CommandBufferSubmitInfo`]
#[doc(alias = "VkCommandBufferSubmitInfoKHR")]
pub type CommandBufferSubmitInfoKHR = CommandBufferSubmitInfo;
///See [`SubmitInfo2`]
#[doc(alias = "VkSubmitInfo2KHR")]
pub type SubmitInfo2KHR = SubmitInfo2;
///See [`PhysicalDeviceSynchronization2Features`]
#[doc(alias = "VkPhysicalDeviceSynchronization2FeaturesKHR")]
pub type PhysicalDeviceSynchronization2FeaturesKHR = PhysicalDeviceSynchronization2Features;
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "checkpointExecutionStageMask")]
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}
impl Default for QueueFamilyCheckpointProperties2NV {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueueFamilyCheckpointProperties2Nv,
            p_next: unsafe { std::mem::zeroed() },
            checkpoint_execution_stage_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CheckpointData2NV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub stage: PipelineStageFlags2,
    #[doc(alias = "pCheckpointMarker")]
    pub checkpoint_marker: *mut std::ffi::c_void,
}
impl Default for CheckpointData2NV {
    fn default() -> Self {
        Self {
            s_type: StructureType::CheckpointData2Nv,
            p_next: unsafe { std::mem::zeroed() },
            stage: unsafe { std::mem::zeroed() },
            checkpoint_marker: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_synchronization2::{
    Flags64, KHR_SYNCHRONIZATION_2_EXTENSION_NAME, KHR_SYNCHRONIZATION_2_SPEC_VERSION,
};
///See [`cmd_set_event2`]
#[doc(alias = "vkCmdSetEvent2KHR")]
pub type FNCmdSetEvent2Khr = FNCmdSetEvent2;
///See [`cmd_reset_event2`]
#[doc(alias = "vkCmdResetEvent2KHR")]
pub type FNCmdResetEvent2Khr = FNCmdResetEvent2;
///See [`cmd_wait_events2`]
#[doc(alias = "vkCmdWaitEvents2KHR")]
pub type FNCmdWaitEvents2Khr = FNCmdWaitEvents2;
///See [`cmd_pipeline_barrier2`]
#[doc(alias = "vkCmdPipelineBarrier2KHR")]
pub type FNCmdPipelineBarrier2Khr = FNCmdPipelineBarrier2;
///See [`queue_submit2`]
#[doc(alias = "vkQueueSubmit2KHR")]
pub type FNQueueSubmit2Khr = FNQueueSubmit2;
///See [`cmd_write_timestamp2`]
#[doc(alias = "vkCmdWriteTimestamp2KHR")]
pub type FNCmdWriteTimestamp2Khr = FNCmdWriteTimestamp2;
#[doc(alias = "vkGetQueueCheckpointData2NV")]
pub type FNGetQueueCheckpointData2Nv = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV,
);
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
pub type FNCmdWriteBufferMarker2Amd = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);