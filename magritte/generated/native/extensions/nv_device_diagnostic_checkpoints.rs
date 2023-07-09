use crate::native::vulkan1_0::{
    BaseOutStructure, CommandBuffer, PipelineStageFlagBits, PipelineStageFlags, Queue, StructureType,
};
#[doc(alias = "VkQueueFamilyCheckpointPropertiesNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyCheckpointPropertiesNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "checkpointExecutionStageMask")]
    pub checkpoint_execution_stage_mask: PipelineStageFlags,
}
impl Default for QueueFamilyCheckpointPropertiesNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueueFamilyCheckpointPropertiesNv,
            p_next: unsafe { std::mem::zeroed() },
            checkpoint_execution_stage_mask: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkCheckpointDataNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CheckpointDataNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub stage: PipelineStageFlagBits,
    #[doc(alias = "pCheckpointMarker")]
    pub checkpoint_marker: *mut std::ffi::c_void,
}
impl Default for CheckpointDataNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::CheckpointDataNv,
            p_next: unsafe { std::mem::zeroed() },
            stage: unsafe { std::mem::zeroed() },
            checkpoint_marker: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_device_diagnostic_checkpoints::{
    NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_EXTENSION_NAME, NV_DEVICE_DIAGNOSTIC_CHECKPOINTS_SPEC_VERSION,
};
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub type FNGetQueueCheckpointDataNv = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
);
#[doc(alias = "vkCmdSetCheckpointNV")]
pub type FNCmdSetCheckpointNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const std::ffi::c_void);
