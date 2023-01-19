//!# [VK_NV_device_diagnostic_checkpoints](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_device_diagnostic_checkpoints.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostic_checkpoints/VK_NV_device_diagnostic_checkpoints.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseOutStructure, CommandBuffer, PipelineStageFlagBits, PipelineStageFlags, Queue, StructureType},
};
use std::ffi::CStr;
///# [VkQueueFamilyCheckpointPropertiesNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointPropertiesNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostic_checkpoints/VkQueueFamilyCheckpointPropertiesNV.md")]
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
///# [VkCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkCheckpointDataNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostic_checkpoints/VkCheckpointDataNV.md")]
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
///# [vkGetQueueCheckpointDataNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointDataNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostic_checkpoints/vkGetQueueCheckpointDataNV.md")]
#[doc(alias = "vkGetQueueCheckpointDataNV")]
pub type FNGetQueueCheckpointDataNv = unsafe extern "system" fn(
    queue: Queue,
    p_checkpoint_data_count: *mut u32,
    p_checkpoint_data: *mut CheckpointDataNV,
);
///# [vkCmdSetCheckpointNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetCheckpointNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_device_diagnostic_checkpoints/vkCmdSetCheckpointNV.md")]
#[doc(alias = "vkCmdSetCheckpointNV")]
pub type FNCmdSetCheckpointNv =
    unsafe extern "system" fn(command_buffer: CommandBuffer, p_checkpoint_marker: *const std::ffi::c_void);
