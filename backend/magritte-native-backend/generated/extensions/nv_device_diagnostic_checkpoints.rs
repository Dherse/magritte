use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, CommandBuffer, PipelineStageFlagBits, PipelineStageFlags, Queue, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "checkpointExecutionStageMask")]
    checkpoint_execution_stage_mask: PipelineStageFlags,
}
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CheckpointDataNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    stage: PipelineStageFlagBits,
    #[doc(alias = "pCheckpointMarker")]
    checkpoint_marker: *mut std::ffi::c_void,
}
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME")]
pub const NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_device_diagnostic_checkpoints");
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub type FNGetQueueCheckpointDataNv = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
);
#[doc(alias = "vkCmdSetCheckpointNV")]
pub type FNCmdSetCheckpointNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const std::ffi::c_void);
