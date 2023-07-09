pub use crate::common::extensions::amd_buffer_marker::{
    AMD_BUFFER_MARKER_EXTENSION_NAME, AMD_BUFFER_MARKER_SPEC_VERSION,
};
use crate::native::vulkan1_0::{Buffer, CommandBuffer, DeviceSize, PipelineStageFlagBits};
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
pub type FNCmdWriteBufferMarkerAmd = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlagBits,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
