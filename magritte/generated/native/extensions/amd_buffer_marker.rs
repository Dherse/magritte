//!# [VK_AMD_buffer_marker](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html)
# ! [doc = include_str ! ("../../../../doc/extensions/amd_buffer_marker/VK_AMD_buffer_marker.md")]
use crate::{
    cstr,
    vulkan1_0::{Buffer, CommandBuffer, DeviceSize, PipelineStageFlagBits},
};
use std::ffi::CStr;
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: &'static CStr = cstr!("VK_AMD_buffer_marker");
///# [vkCmdWriteBufferMarkerAMD](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html)
# [doc = include_str ! ("../../../../doc/extensions/amd_buffer_marker/vkCmdWriteBufferMarkerAMD.md")]
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
pub type FNCmdWriteBufferMarkerAmd = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    pipeline_stage: PipelineStageFlagBits,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);
