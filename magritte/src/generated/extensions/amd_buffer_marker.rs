//![VK_AMD_buffer_marker](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_buffer_marker.html) - device extension
//!# Description
//!This extension adds a new operation to execute pipelined writes of small
//!marker values into a [`Buffer`] object.The primary purpose of these markers is to facilitate the
//! development of
//!debugging tools for tracking which pipelined command contributed to device
//!loss.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_buffer_marker]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the VK_AMD_buffer_marker
//!   extension>>)
//!# New functions & commands
//! - [`cmd_write_buffer_marker_amd`]
//!# New constants
//! - [`AMD_BUFFER_MARKER_EXTENSION_NAME`]
//! - [`AMD_BUFFER_MARKER_SPEC_VERSION`]
//!# Version History
//! - Revision 1, 2018-01-26 (Jaakko Konttinen)  - Initial revision
//!# Other info
//! * 2018-01-26
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Jaakko Konttinen, AMD  - Daniel Rakos, AMD
//!# Related
//! - [`cmd_write_buffer_marker_amd`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{Buffer, CommandBuffer, Device, DeviceSize, PipelineStageFlagBits};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_BUFFER_MARKER_SPEC_VERSION")]
pub const AMD_BUFFER_MARKER_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_AMD_BUFFER_MARKER_EXTENSION_NAME")]
pub const AMD_BUFFER_MARKER_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_AMD_buffer_marker");
///[vkCmdWriteBufferMarkerAMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarkerAMD.html) - Execute a pipelined write of a marker value into a buffer
///# C Specifications
///To write a 32-bit marker value into a buffer as a pipelined operation, call:
///```c
///// Provided by VK_AMD_buffer_marker
///void vkCmdWriteBufferMarkerAMD(
///    VkCommandBuffer                             commandBuffer,
///    VkPipelineStageFlagBits                     pipelineStage,
///    VkBuffer                                    dstBuffer,
///    VkDeviceSize                                dstOffset,
///    uint32_t                                    marker);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`pipeline_stage`] is a [`PipelineStageFlagBits`] value specifying the pipeline stage whose
///   completion triggers the marker write.
/// - [`dst_buffer`] is the buffer where the marker will be written to.
/// - [`dst_offset`] is the byte offset into the buffer where the marker will be written to.
/// - [`marker`] is the 32-bit value of the marker.
///# Description
///The command will write the 32-bit marker value into the buffer only after
///all preceding commands have finished executing up to at least the specified
///pipeline stage.
///This includes the completion of other preceding
///[`cmd_write_buffer_marker_amd`] commands so long as their specified pipeline
///stages occur either at the same time or earlier than this command’s
///specified [`pipeline_stage`].While consecutive buffer marker writes with the same
/// [`pipeline_stage`]
///parameter are implicitly complete in submission order, memory and execution
///dependencies between buffer marker writes and other operations must still be
///explicitly ordered using synchronization commands.
///The access scope for buffer marker writes falls under the
///`VK_ACCESS_TRANSFER_WRITE_BIT`, and the pipeline stages for identifying
///the synchronization scope  **must**  include both [`pipeline_stage`] and
///`VK_PIPELINE_STAGE_TRANSFER_BIT`.
///## Valid Usage
/// - [`pipeline_stage`] **must**  be a [valid stage]() for the queue family that was used to create
///   the command pool that [`command_buffer`] was allocated from
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_MESH_SHADER_BIT_NV` or `VK_PIPELINE_STAGE_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`pipeline_stage`] **must**  not be
///   `VK_PIPELINE_STAGE_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-synchronization2)
///   feature is not enabled, [`pipeline_stage`] **must**  not be `VK_PIPELINE_STAGE_NONE`
/// - [`dst_offset`] **must**  be less than or equal to the size of [`dst_buffer`] minus `4`
/// - [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
/// - If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_offset`] **must**  be a multiple of `4`
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`pipeline_stage`] is not `0`, [`pipeline_stage`] **must**  be a valid
///   [`PipelineStageFlagBits`] value
/// - [`dst_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support transfer,
///   graphics, or compute operations
/// - Both of [`command_buffer`], and [`dst_buffer`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`VK_AMD_buffer_marker`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
/// - [`PipelineStageFlagBits`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdWriteBufferMarkerAMD")]
pub type FNCmdWriteBufferMarkerAmd = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        pipeline_stage: PipelineStageFlagBits,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ),
>;
///The V-table of [`Device`] for functions from VK_AMD_buffer_marker
pub struct DeviceAmdBufferMarkerVTable {
    ///See [`FNCmdWriteBufferMarkerAmd`] for more information.
    pub cmd_write_buffer_marker_amd: FNCmdWriteBufferMarkerAmd,
}
impl DeviceAmdBufferMarkerVTable {
    ///Loads the VTable from the owner and the names
    pub fn load<F>(loader_fn: F, loader: Device) -> Self
    where
        F: Fn(Device, &'static CStr) -> Option<extern "system" fn()>,
    {
        Self {
            cmd_write_buffer_marker_amd: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdWriteBufferMarkerAMD")))
            },
        }
    }
    ///Gets [`Self::cmd_write_buffer_marker_amd`]. See [`FNCmdWriteBufferMarkerAmd`] for more
    /// information.
    pub fn cmd_write_buffer_marker_amd(&self) -> FNCmdWriteBufferMarkerAmd {
        self.cmd_write_buffer_marker_amd
    }
}
