//!# [VK_KHR_synchronization2](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/VK_KHR_synchronization2.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, Buffer, CommandBuffer, DeviceSize, Queue, StructureType},
    vulkan1_3::{
        AccessFlagBits2, AccessFlags2, BufferMemoryBarrier2, CommandBufferSubmitInfo, DependencyInfo,
        FNCmdPipelineBarrier2, FNCmdResetEvent2, FNCmdSetEvent2, FNCmdWaitEvents2, FNCmdWriteTimestamp2,
        FNQueueSubmit2, ImageMemoryBarrier2, MemoryBarrier2, PhysicalDeviceSynchronization2Features,
        PipelineStageFlagBits2, PipelineStageFlags2, SemaphoreSubmitInfo, SubmitFlagBits, SubmitFlags, SubmitInfo2,
    },
};
use std::ffi::CStr;
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
///# [VkQueueFamilyCheckpointProperties2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/VkQueueFamilyCheckpointProperties2NV.md")]
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "checkpointExecutionStageMask")]
    checkpoint_execution_stage_mask: PipelineStageFlags2,
}
///# [VkCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/VkCheckpointData2NV.md")]
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CheckpointData2NV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    stage: PipelineStageFlags2,
    #[doc(alias = "pCheckpointMarker")]
    checkpoint_marker: *mut std::ffi::c_void,
}
///# [VkFlags64](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFlags64.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/VkFlags64.md")]
#[doc(alias = "VkFlags64")]
pub type Flags64 = u64;
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_synchronization2");
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
///# [vkGetQueueCheckpointData2NV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/vkGetQueueCheckpointData2NV.md")]
#[doc(alias = "vkGetQueueCheckpointData2NV")]
pub type FNGetQueueCheckpointData2Nv = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointData2NV,
);
///# [vkCmdWriteBufferMarker2AMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_synchronization2/vkCmdWriteBufferMarker2AMD.md")]
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
pub type FNCmdWriteBufferMarker2Amd = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
