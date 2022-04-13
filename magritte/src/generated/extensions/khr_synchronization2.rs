//![VK_KHR_synchronization2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_synchronization2.html) - device extension
//!# Description
//!This extension modifies the original core synchronization APIs to simplify
//!the interface and improve usability of these APIs.
//!It also adds new pipeline stage and access flag types that extend into the
//!64-bit range, as we have run out within the 32-bit range.
//!The new flags are identical to the old values within the 32-bit range, with
//!new stages and bits beyond that.Pipeline stages and access flags are now specified together in
//! memory
//!barrier structures, making the connection between the two more obvious.
//!Additionally, scoping the pipeline stages into the barrier structs allows
//!the use of the `MEMORY_READ` and `MEMORY_WRITE` flags without
//!sacrificing precision.
//!The per-stage access flags should be used to disambiguate specific accesses
//!in a given stage or set of stages - for instance, between uniform reads and
//!sampling operations.Layout transitions have been simplified as well; rather than requiring a
//!different set of layouts for depth/stencil/color attachments, there are
//!generic `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR` and
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` layouts which are contextually
//!applied based on the image format.
//!For example, for a depth format image,
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` is equivalent to
//!`VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL_KHR`.
//!`VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR` also functionally replaces
//!`VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL`.Events are now more efficient, because they include
//! memory dependency
//!information when you set them on the device.
//!Previously, this information was only known when waiting on an event, so the
//!dependencies could not be satisfied until the wait occurred.
//!That sometimes meant stalling the pipeline when the wait occurred.
//!The new API provides enough information for implementations to satisfy these
//!dependencies in parallel with other tasks.Queue submission has been changed to wrap command
//! buffers and semaphores in
//!extensible structures, which incorporate changes from Vulkan 1.1,
//!`[`VK_KHR_device_group`]`, and `[`VK_KHR_timeline_semaphore`]`.
//!This also adds a pipeline stage to the semaphore signal operation, mirroring
//!the existing pipeline stage specification for wait operations.Other miscellaneous changes
//! include:
//! - Events can now be specified as interacting only with the device, allowing more efficient
//!   access to the underlying object.
//! - Image memory barriers that do not perform an image layout transition can be specified by
//!   setting `oldLayout` equal to `newLayout`.  - E.g. the old and new layout can both be set to
//!   `VK_IMAGE_LAYOUT_UNDEFINED`, without discarding data in the image.
//! - Queue family ownership transfer parameters are simplified in some cases.
//! - Where two synchronization commands need to be matched up (queue transfer operations, events),
//!   the dependency information specified in each place must now match completely for consistency.
//! - Extensions with commands or functions with a [`PipelineStageFlags`] or
//!   [`PipelineStageFlagBits`] parameter have had those APIs replaced with equivalents using
//!   [`PipelineStageFlags2KHR`].
//! - The new event and barrier interfaces are now more extensible for future changes.
//! - Relevant pipeline stage masks can now be specified as empty with the new
//!   `VK_PIPELINE_STAGE_NONE_KHR` and `VK_PIPELINE_STAGE_2_NONE_KHR` values.
//! - [`MemoryBarrier2KHR`] can be chained to [`SubpassDependency2`], overriding the original 32-bit
//!   stage and access masks.
//!# Revision
//!1
//!# Dependencies
//! - *Promoted* to [Vulkan 1.3](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.3-promotions)
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Tobias Hector [tobski](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_synchronization2]
//!   @tobski%0A<<Here describe the issue or question you have about the VK_KHR_synchronization2
//!   extension>>)
//!# New functions & commands
//! - [`cmd_pipeline_barrier2_khr`]
//! - [`cmd_reset_event2_khr`]
//! - [`cmd_set_event2_khr`]
//! - [`cmd_wait_events2_khr`]
//! - [`cmd_write_timestamp2_khr`]
//! - [`queue_submit2_khr`]
//!If [`VK_AMD_buffer_marker`] is supported:
//! - [`cmd_write_buffer_marker2_amd`]
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - [`get_queue_checkpoint_data2_nv`]
//!# New structures
//! - [`BufferMemoryBarrier2KHR`]
//! - [`CommandBufferSubmitInfoKHR`]
//! - [`DependencyInfoKHR`]
//! - [`ImageMemoryBarrier2KHR`]
//! - [`SemaphoreSubmitInfoKHR`]
//! - [`SubmitInfo2KHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceSynchronization2FeaturesKHR`]
//! - Extending [`SubpassDependency2`]:  - [`MemoryBarrier2KHR`]
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - [`CheckpointData2NV`]
//! - Extending [`QueueFamilyProperties2`]:  - [`QueueFamilyCheckpointProperties2NV`]
//!# New enums
//! - [`AccessFlagBits2KHR`]
//! - [`PipelineStageFlagBits2KHR`]
//! - [`SubmitFlagBitsKHR`]
//!# New bitmasks
//! - [`AccessFlags2KHR`]
//! - [`PipelineStageFlags2KHR`]
//! - [`SubmitFlagsKHR`]
//!# New constants
//! - [`KHR_SYNCHRONIZATION_2_EXTENSION_NAME`]
//! - [`KHR_SYNCHRONIZATION_2_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_NONE_KHR`
//! - Extending [`EventCreateFlagBits`]:  - `VK_EVENT_CREATE_DEVICE_ONLY_BIT_KHR`
//! - Extending [`ImageLayout`]:  - `VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL_KHR`  -
//!   `VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL_KHR`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_NONE_KHR`
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2_KHR`  -
//!   `VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_DEPENDENCY_INFO_KHR`
//!   - `VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2_KHR`  - `VK_STRUCTURE_TYPE_MEMORY_BARRIER_2_KHR`
//!   - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO_KHR`  - `VK_STRUCTURE_TYPE_SUBMIT_INFO_2_KHR`
//!If [`VK_EXT_blend_operation_advanced`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_COLOR_ATTACHMENT_READ_NONCOHERENT_BIT_EXT`
//!If [`VK_EXT_conditional_rendering`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_CONDITIONAL_RENDERING_READ_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
//!If [`VK_EXT_fragment_density_map`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_FRAGMENT_DENSITY_MAP_READ_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
//!If [`VK_EXT_transform_feedback`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`  -
//!   `VK_ACCESS_2_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`  -
//!   `VK_ACCESS_2_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
//!If [`VK_KHR_acceleration_structure`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_KHR`  -
//!   `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
//!If [`VK_KHR_fragment_shading_rate`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_FRAGMENT_SHADING_RATE_ATTACHMENT_READ_BIT_KHR`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
//!If [`VK_KHR_ray_tracing_pipeline`] is supported:
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_KHR`
//!If [`VK_NV_device_diagnostic_checkpoints`] is supported:
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`  -
//!   `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
//!If [`VK_NV_device_generated_commands`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_COMMAND_PREPROCESS_READ_BIT_NV`  -
//!   `VK_ACCESS_2_COMMAND_PREPROCESS_WRITE_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_COMMAND_PREPROCESS_BIT_NV`
//!If [`VK_NV_mesh_shader`] is supported:
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
//!If [`VK_NV_ray_tracing`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_ACCELERATION_STRUCTURE_READ_BIT_NV`  -
//!   `VK_ACCESS_2_ACCELERATION_STRUCTURE_WRITE_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  -
//!   `VK_PIPELINE_STAGE_2_ACCELERATION_STRUCTURE_BUILD_BIT_NV`  -
//!   `VK_PIPELINE_STAGE_2_RAY_TRACING_SHADER_BIT_NV`
//!If [`VK_NV_shading_rate_image`] is supported:
//! - Extending [`AccessFlagBits2`]:  - `VK_ACCESS_2_SHADING_RATE_IMAGE_READ_BIT_NV`
//! - Extending [`PipelineStageFlagBits2`]:  - `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
//!# Version History
//! - Revision 1, 2020-12-03 (Tobias Hector)  - Internal revisions
//!# Other info
//! * 2020-12-03
//! * - Promoted to Vulkan 1.3 Core  - Interacts with `[`VK_KHR_create_renderpass2`]`
//! * - Tobias Hector
//!# Related
//! - [`AccessFlagBits2KHR`]
//! - [`AccessFlags2KHR`]
//! - [`BufferMemoryBarrier2KHR`]
//! - [`CommandBufferSubmitInfoKHR`]
//! - [`DependencyInfoKHR`]
//! - [`Flags64`]
//! - [`ImageMemoryBarrier2KHR`]
//! - [`MemoryBarrier2KHR`]
//! - [`PhysicalDeviceSynchronization2FeaturesKHR`]
//! - [`PipelineStageFlagBits2KHR`]
//! - [`PipelineStageFlags2KHR`]
//! - [`SemaphoreSubmitInfoKHR`]
//! - [`SubmitFlagBitsKHR`]
//! - [`SubmitFlagsKHR`]
//! - [`SubmitInfo2KHR`]
//! - [`cmd_pipeline_barrier2_khr`]
//! - [`cmd_reset_event2_khr`]
//! - [`cmd_set_event2_khr`]
//! - [`cmd_wait_events2_khr`]
//! - [`cmd_write_timestamp2_khr`]
//! - [`queue_submit2_khr`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseOutStructure, Buffer, CommandBuffer, Device, DeviceSize, Queue, StructureType},
    vulkan1_3::{
        FNCmdPipelineBarrier2, FNCmdResetEvent2, FNCmdSetEvent2, FNCmdWaitEvents2, FNCmdWriteTimestamp2,
        FNQueueSubmit2, PipelineStageFlags2,
    },
    AsRaw, SmallVec, Unique,
};
use std::{
    ffi::{c_void, CStr},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_SPEC_VERSION")]
pub const KHR_SYNCHRONIZATION_2_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_SYNCHRONIZATION_2_EXTENSION_NAME")]
pub const KHR_SYNCHRONIZATION_2_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_synchronization2");
///[VkFlags64](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFlags64.html) - Vulkan 64-bit bitmasks
///# C Specifications
///A collection of 64-bit flags is represented by a bitmask using the type
///[`Flags64`]:
///```c
///// Provided by VK_KHR_synchronization2
///typedef uint64_t VkFlags64;
///```
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`Flags`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkFlags64")]
pub type Flags64 = u64;
///[vkGetQueueCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html) - Retrieve diagnostic checkpoint data
///# C Specifications
///If the device encounters an error during execution, the implementation will
///return a `VK_ERROR_DEVICE_LOST` error to the application at some point
///during host execution.
///When this happens, the application  **can**  call
///[`get_queue_checkpoint_data2_nv`] to retrieve information on the most recent
///diagnostic checkpoints that were executed by the device.
///```c
///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
///void vkGetQueueCheckpointData2NV(
///    VkQueue                                     queue,
///    uint32_t*                                   pCheckpointDataCount,
///    VkCheckpointData2NV*                        pCheckpointData);
///```
///# Parameters
/// - [`queue`] is the [`Queue`] object the caller would like to retrieve checkpoint data for
/// - [`p_checkpoint_data_count`] is a pointer to an integer related to the number of checkpoint
///   markers available or queried, as described below.
/// - [`p_checkpoint_data`] is either `NULL` or a pointer to an array of [`CheckpointData2NV`]
///   structures.
///# Description
///If [`p_checkpoint_data`] is `NULL`, then the number of checkpoint markers
///available is returned in [`p_checkpoint_data_count`].
///Otherwise, [`p_checkpoint_data_count`] **must**  point to a variable set by the
///user to the number of elements in the [`p_checkpoint_data`] array, and on
///return the variable is overwritten with the number of structures actually
///written to [`p_checkpoint_data`].If [`p_checkpoint_data_count`] is less than the number of
/// checkpoint markers
///available, at most [`p_checkpoint_data_count`] structures will be written.
///## Valid Usage
/// - The device that [`queue`] belongs to  **must**  be in the lost state
///
///## Valid Usage (Implicit)
/// - [`queue`] **must**  be a valid [`Queue`] handle
/// - [`p_checkpoint_data_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_checkpoint_data_count`] is not `0`, and [`p_checkpoint_data`]
///   is not `NULL`, [`p_checkpoint_data`] **must**  be a valid pointer to an array of
///   [`p_checkpoint_data_count`][`CheckpointData2NV`] structures
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`CheckpointData2NV`]
/// - [`Queue`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetQueueCheckpointData2NV")]
pub type FNGetQueueCheckpointData2Nv = Option<
    for<'lt> unsafe extern "system" fn(
        queue: Queue,
        p_checkpoint_data_count: *mut u32,
        p_checkpoint_data: *mut CheckpointData2NV<'lt>,
    ),
>;
///[vkCmdWriteBufferMarker2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) - Execute a pipelined write of a marker value into a buffer
///# C Specifications
///To write a 32-bit marker value into a buffer as a pipelined operation, call:
///```c
///// Provided by VK_KHR_synchronization2 with VK_AMD_buffer_marker
///void vkCmdWriteBufferMarker2AMD(
///    VkCommandBuffer                             commandBuffer,
///    VkPipelineStageFlags2                       stage,
///    VkBuffer                                    dstBuffer,
///    VkDeviceSize                                dstOffset,
///    uint32_t                                    marker);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command will be recorded.
/// - [`stage`] specifies the pipeline stage whose completion triggers the marker write.
/// - [`dst_buffer`] is the buffer where the marker will be written.
/// - [`dst_offset`] is the byte offset into the buffer where the marker will be written.
/// - [`marker`] is the 32-bit value of the marker.
///# Description
///The command will write the 32-bit marker value into the buffer only after
///all preceding commands have finished executing up to at least the specified
///pipeline stage.
///This includes the completion of other preceding
///[`cmd_write_buffer_marker2_amd`] commands so long as their specified
///pipeline stages occur either at the same time or earlier than this command’s
///specified [`stage`].While consecutive buffer marker writes with the same [`stage`] parameter
///implicitly complete in submission order, memory and execution dependencies
///between buffer marker writes and other operations  **must**  still be explicitly
///ordered using synchronization commands.
///The access scope for buffer marker writes falls under the
///`VK_ACCESS_TRANSFER_WRITE_BIT`, and the pipeline stages for identifying
///the synchronization scope  **must**  include both [`stage`] and
///`VK_PIPELINE_STAGE_TRANSFER_BIT`.
///## Valid Usage
/// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
/// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
/// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
/// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
/// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
/// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
/// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
/// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
/// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
/// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
///   feature is not enabled, [`stage`] **must**  not contain
///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
/// - The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2)
///   feature  **must**  be enabled
/// - [`stage`] **must**  include only a single pipeline stage
/// - [`stage`] **must**  include only stages that are valid for the queue family that was used to
///   create the command pool that [`command_buffer`] was allocated from
/// - [`dst_offset`] **must**  be less than or equal to the size of [`dst_buffer`] minus `4`
/// - [`dst_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage
///   flag
/// - If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a
///   single [`DeviceMemory`] object
/// - [`dst_offset`] **must**  be a multiple of `4`
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`stage`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
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
/// - [`VK_KHR_synchronization2`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
/// - [`PipelineStageFlags2`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdWriteBufferMarker2AMD")]
pub type FNCmdWriteBufferMarker2Amd = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: u32,
    ),
>;
///[VkQueueFamilyCheckpointProperties2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueueFamilyCheckpointProperties2NV.html) - Return structure for queue family checkpoint information query
///# C Specifications
///The [`QueueFamilyCheckpointProperties2NV`] structure is defined as:
///```c
///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
///typedef struct VkQueueFamilyCheckpointProperties2NV {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkPipelineStageFlags2    checkpointExecutionStageMask;
///} VkQueueFamilyCheckpointProperties2NV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`checkpoint_execution_stage_mask`] is a mask indicating which pipeline stages the
///   implementation can execute checkpoint markers in.
///# Description
///Additional queue family information can be queried by setting
///[`QueueFamilyProperties2`]::[`p_next`] to point to a
///[`QueueFamilyCheckpointProperties2NV`] structure.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV`
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueueFamilyCheckpointProperties2NV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct QueueFamilyCheckpointProperties2NV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`checkpoint_execution_stage_mask`] is a mask indicating which pipeline
    ///stages the implementation can execute checkpoint markers in.
    pub checkpoint_execution_stage_mask: PipelineStageFlags2,
}
impl<'lt> Default for QueueFamilyCheckpointProperties2NV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::QUEUE_FAMILY_CHECKPOINT_PROPERTIES2_NV,
            p_next: std::ptr::null_mut(),
            checkpoint_execution_stage_mask: Default::default(),
        }
    }
}
impl<'lt> QueueFamilyCheckpointProperties2NV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn checkpoint_execution_stage_mask(&self) -> PipelineStageFlags2 {
        self.checkpoint_execution_stage_mask
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn checkpoint_execution_stage_mask_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.checkpoint_execution_stage_mask
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::checkpoint_execution_stage_mask`]
    pub fn set_checkpoint_execution_stage_mask(mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> Self {
        self.checkpoint_execution_stage_mask = value;
        self
    }
}
///[VkCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCheckpointData2NV.html) - Return structure for command buffer checkpoint data
///# C Specifications
///The [`CheckpointData2NV`] structure is defined as:
///```c
///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
///typedef struct VkCheckpointData2NV {
///    VkStructureType          sType;
///    void*                    pNext;
///    VkPipelineStageFlags2    stage;
///    void*                    pCheckpointMarker;
///} VkCheckpointData2NV;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`stage`] indicates a single pipeline stage which the checkpoint marker data refers to.
/// - [`checkpoint_marker`] contains the value of the last checkpoint marker executed in the stage
///   that [`stage`] refers to.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV`
/// - [`p_next`] **must**  be `NULL`
///The stages at which a checkpoint marker  **can**  be executed are
///implementation-defined and  **can**  be queried by calling
///[`get_physical_device_queue_family_properties2`].
///# Related
/// - [`VK_KHR_synchronization2`]
/// - [`VK_NV_device_diagnostic_checkpoints`]
/// - [`PipelineStageFlags2`]
/// - [`StructureType`]
/// - [`get_queue_checkpoint_data2_nv`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCheckpointData2NV")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct CheckpointData2NV<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`stage`] indicates a single pipeline stage which the checkpoint
    ///marker data refers to.
    pub stage: PipelineStageFlags2,
    ///[`checkpoint_marker`] contains the value of the last checkpoint marker
    ///executed in the stage that [`stage`] refers to.
    pub checkpoint_marker: *mut c_void,
}
impl<'lt> Default for CheckpointData2NV<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CHECKPOINT_DATA2_NV,
            p_next: std::ptr::null_mut(),
            stage: Default::default(),
            checkpoint_marker: std::ptr::null_mut(),
        }
    }
}
impl<'lt> CheckpointData2NV<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::checkpoint_marker`]
    pub fn checkpoint_marker_raw(&self) -> *mut c_void {
        self.checkpoint_marker
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker_raw(mut self, value: *mut c_void) -> Self {
        self.checkpoint_marker = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::stage`]
    pub fn stage(&self) -> PipelineStageFlags2 {
        self.stage
    }
    ///Gets the value of [`Self::checkpoint_marker`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn checkpoint_marker(&self) -> &c_void {
        &*self.checkpoint_marker
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::stage`]
    pub fn stage_mut(&mut self) -> &mut PipelineStageFlags2 {
        &mut self.stage
    }
    ///Gets a mutable reference to the value of [`Self::checkpoint_marker`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn checkpoint_marker_mut(&mut self) -> &mut c_void {
        &mut *self.checkpoint_marker
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::stage`]
    pub fn set_stage(mut self, value: crate::vulkan1_3::PipelineStageFlags2) -> Self {
        self.stage = value;
        self
    }
    ///Sets the value of [`Self::checkpoint_marker`]
    pub fn set_checkpoint_marker(mut self, value: &'lt mut std::ffi::c_void) -> Self {
        self.checkpoint_marker = value as *mut _;
        self
    }
}
impl Queue {
    ///[vkGetQueueCheckpointData2NV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueueCheckpointData2NV.html) - Retrieve diagnostic checkpoint data
    ///# C Specifications
    ///If the device encounters an error during execution, the implementation will
    ///return a `VK_ERROR_DEVICE_LOST` error to the application at some point
    ///during host execution.
    ///When this happens, the application  **can**  call
    ///[`get_queue_checkpoint_data2_nv`] to retrieve information on the most recent
    ///diagnostic checkpoints that were executed by the device.
    ///```c
    ///// Provided by VK_KHR_synchronization2 with VK_NV_device_diagnostic_checkpoints
    ///void vkGetQueueCheckpointData2NV(
    ///    VkQueue                                     queue,
    ///    uint32_t*                                   pCheckpointDataCount,
    ///    VkCheckpointData2NV*                        pCheckpointData);
    ///```
    ///# Parameters
    /// - [`queue`] is the [`Queue`] object the caller would like to retrieve checkpoint data for
    /// - [`p_checkpoint_data_count`] is a pointer to an integer related to the number of checkpoint
    ///   markers available or queried, as described below.
    /// - [`p_checkpoint_data`] is either `NULL` or a pointer to an array of [`CheckpointData2NV`]
    ///   structures.
    ///# Description
    ///If [`p_checkpoint_data`] is `NULL`, then the number of checkpoint markers
    ///available is returned in [`p_checkpoint_data_count`].
    ///Otherwise, [`p_checkpoint_data_count`] **must**  point to a variable set by the
    ///user to the number of elements in the [`p_checkpoint_data`] array, and on
    ///return the variable is overwritten with the number of structures actually
    ///written to [`p_checkpoint_data`].If [`p_checkpoint_data_count`] is less than the number of
    /// checkpoint markers
    ///available, at most [`p_checkpoint_data_count`] structures will be written.
    ///## Valid Usage
    /// - The device that [`queue`] belongs to  **must**  be in the lost state
    ///
    ///## Valid Usage (Implicit)
    /// - [`queue`] **must**  be a valid [`Queue`] handle
    /// - [`p_checkpoint_data_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_checkpoint_data_count`] is not `0`, and
    ///   [`p_checkpoint_data`] is not `NULL`, [`p_checkpoint_data`] **must**  be a valid pointer to
    ///   an array of [`p_checkpoint_data_count`][`CheckpointData2NV`] structures
    ///# Related
    /// - [`VK_KHR_synchronization2`]
    /// - [`VK_NV_device_diagnostic_checkpoints`]
    /// - [`CheckpointData2NV`]
    /// - [`Queue`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetQueueCheckpointData2NV")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_queue_checkpoint_data2_nv<'lt>(
        self: &Unique<Queue>,
        p_checkpoint_data_count: Option<usize>,
    ) -> SmallVec<CheckpointData2NV<'lt>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_synchronization2()
            .and_then(|vtable| vtable.get_queue_checkpoint_data2_nv())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_synchronization2()
            .and_then(|vtable| vtable.get_queue_checkpoint_data2_nv())
            .unwrap_unchecked();
        let mut p_checkpoint_data_count = match p_checkpoint_data_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_checkpoint_data =
            SmallVec::<CheckpointData2NV<'lt>>::from_elem(Default::default(), p_checkpoint_data_count as usize);
        let _return = _function(
            self.as_raw(),
            &mut p_checkpoint_data_count,
            p_checkpoint_data.as_mut_ptr(),
        );
        p_checkpoint_data
    }
}
impl CommandBuffer {
    ///[vkCmdWriteBufferMarker2AMD](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteBufferMarker2AMD.html) - Execute a pipelined write of a marker value into a buffer
    ///# C Specifications
    ///To write a 32-bit marker value into a buffer as a pipelined operation, call:
    ///```c
    ///// Provided by VK_KHR_synchronization2 with VK_AMD_buffer_marker
    ///void vkCmdWriteBufferMarker2AMD(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkPipelineStageFlags2                       stage,
    ///    VkBuffer                                    dstBuffer,
    ///    VkDeviceSize                                dstOffset,
    ///    uint32_t                                    marker);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command will be recorded.
    /// - [`stage`] specifies the pipeline stage whose completion triggers the marker write.
    /// - [`dst_buffer`] is the buffer where the marker will be written.
    /// - [`dst_offset`] is the byte offset into the buffer where the marker will be written.
    /// - [`marker`] is the 32-bit value of the marker.
    ///# Description
    ///The command will write the 32-bit marker value into the buffer only after
    ///all preceding commands have finished executing up to at least the specified
    ///pipeline stage.
    ///This includes the completion of other preceding
    ///[`cmd_write_buffer_marker2_amd`] commands so long as their specified
    ///pipeline stages occur either at the same time or earlier than this command’s
    ///specified [`stage`].While consecutive buffer marker writes with the same [`stage`] parameter
    ///implicitly complete in submission order, memory and execution dependencies
    ///between buffer marker writes and other operations  **must**  still be explicitly
    ///ordered using synchronization commands.
    ///The access scope for buffer marker writes falls under the
    ///`VK_ACCESS_TRANSFER_WRITE_BIT`, and the pipeline stages for identifying
    ///the synchronization scope  **must**  include both [`stage`] and
    ///`VK_PIPELINE_STAGE_TRANSFER_BIT`.
    ///## Valid Usage
    /// - If the [geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-geometryShader)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT`
    /// - If the [tessellation shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-tessellationShader)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT` or
    ///   `VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT`
    /// - If the [conditional rendering](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-conditionalRendering)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_CONDITIONAL_RENDERING_BIT_EXT`
    /// - If the [fragment density map](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-fragmentDensityMap)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_FRAGMENT_DENSITY_PROCESS_BIT_EXT`
    /// - If the [transform feedback](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-transformFeedback)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_TRANSFORM_FEEDBACK_BIT_EXT`
    /// - If the [mesh shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-meshShader)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_MESH_SHADER_BIT_NV`
    /// - If the [task shaders](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-taskShader)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_TASK_SHADER_BIT_NV`
    /// - If the [shading rate image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-shadingRateImage)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_SHADING_RATE_IMAGE_BIT_NV`
    /// - If the [subpass shading](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-subpassShading)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_SUBPASS_SHADING_BIT_HUAWEI`
    /// - If the [invocation mask image](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#features-invocationMask)
    ///   feature is not enabled, [`stage`] **must**  not contain
    ///   `VK_PIPELINE_STAGE_2_INVOCATION_MASK_BIT_HUAWEI`
    /// - The [`synchronization2`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-synchronization2)
    ///   feature  **must**  be enabled
    /// - [`stage`] **must**  include only a single pipeline stage
    /// - [`stage`] **must**  include only stages that are valid for the queue family that was used
    ///   to create the command pool that [`command_buffer`] was allocated from
    /// - [`dst_offset`] **must**  be less than or equal to the size of [`dst_buffer`] minus `4`
    /// - [`dst_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_TRANSFER_DST_BIT`
    ///   usage flag
    /// - If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to
    ///   a single [`DeviceMemory`] object
    /// - [`dst_offset`] **must**  be a multiple of `4`
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`stage`] **must**  be a valid combination of [`PipelineStageFlagBits2`] values
    /// - [`dst_buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   transfer, graphics, or compute operations
    /// - Both of [`command_buffer`], and [`dst_buffer`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`VK_AMD_buffer_marker`]
    /// - [`VK_KHR_synchronization2`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    /// - [`PipelineStageFlags2`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdWriteBufferMarker2AMD")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_write_buffer_marker2_amd(
        self: &Unique<CommandBuffer>,
        stage: PipelineStageFlags2,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        marker: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .khr_synchronization2()
            .and_then(|vtable| vtable.cmd_write_buffer_marker2_amd())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .khr_synchronization2()
            .and_then(|vtable| vtable.cmd_write_buffer_marker2_amd())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            stage,
            dst_buffer,
            dst_offset,
            marker.unwrap_or_default() as _,
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_KHR_synchronization2`
pub struct DeviceKhrSynchronization2VTable {
    ///See [`FNGetQueueCheckpointData2Nv`] for more information.
    pub get_queue_checkpoint_data2_nv: FNGetQueueCheckpointData2Nv,
    ///See [`FNCmdWriteBufferMarker2Amd`] for more information.
    pub cmd_write_buffer_marker2_amd: FNCmdWriteBufferMarker2Amd,
    ///See [`FNCmdSetEvent2`] for more information.
    pub cmd_set_event2_khr: FNCmdSetEvent2,
    ///See [`FNCmdResetEvent2`] for more information.
    pub cmd_reset_event2_khr: FNCmdResetEvent2,
    ///See [`FNCmdWaitEvents2`] for more information.
    pub cmd_wait_events2_khr: FNCmdWaitEvents2,
    ///See [`FNCmdPipelineBarrier2`] for more information.
    pub cmd_pipeline_barrier2_khr: FNCmdPipelineBarrier2,
    ///See [`FNQueueSubmit2`] for more information.
    pub queue_submit2_khr: FNQueueSubmit2,
    ///See [`FNCmdWriteTimestamp2`] for more information.
    pub cmd_write_timestamp2_khr: FNCmdWriteTimestamp2,
}
impl DeviceKhrSynchronization2VTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_queue_checkpoint_data2_nv: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetQueueCheckpointData2NV").as_ptr()))
            },
            cmd_write_buffer_marker2_amd: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdWriteBufferMarker2AMD").as_ptr()))
            },
            cmd_set_event2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdSetEvent2KHR").as_ptr()))
            },
            cmd_reset_event2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdResetEvent2KHR").as_ptr()))
            },
            cmd_wait_events2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdWaitEvents2KHR").as_ptr()))
            },
            cmd_pipeline_barrier2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdPipelineBarrier2KHR").as_ptr()))
            },
            queue_submit2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkQueueSubmit2KHR").as_ptr()))
            },
            cmd_write_timestamp2_khr: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdWriteTimestamp2KHR").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_queue_checkpoint_data2_nv`]. See [`FNGetQueueCheckpointData2Nv`] for more
    /// information.
    pub fn get_queue_checkpoint_data2_nv(&self) -> FNGetQueueCheckpointData2Nv {
        self.get_queue_checkpoint_data2_nv
    }
    ///Gets [`Self::cmd_write_buffer_marker2_amd`]. See [`FNCmdWriteBufferMarker2Amd`] for more
    /// information.
    pub fn cmd_write_buffer_marker2_amd(&self) -> FNCmdWriteBufferMarker2Amd {
        self.cmd_write_buffer_marker2_amd
    }
    ///Gets [`Self::cmd_set_event2_khr`]. See [`FNCmdSetEvent2`] for more information.
    pub fn cmd_set_event2_khr(&self) -> FNCmdSetEvent2 {
        self.cmd_set_event2_khr
    }
    ///Gets [`Self::cmd_reset_event2_khr`]. See [`FNCmdResetEvent2`] for more information.
    pub fn cmd_reset_event2_khr(&self) -> FNCmdResetEvent2 {
        self.cmd_reset_event2_khr
    }
    ///Gets [`Self::cmd_wait_events2_khr`]. See [`FNCmdWaitEvents2`] for more information.
    pub fn cmd_wait_events2_khr(&self) -> FNCmdWaitEvents2 {
        self.cmd_wait_events2_khr
    }
    ///Gets [`Self::cmd_pipeline_barrier2_khr`]. See [`FNCmdPipelineBarrier2`] for more
    /// information.
    pub fn cmd_pipeline_barrier2_khr(&self) -> FNCmdPipelineBarrier2 {
        self.cmd_pipeline_barrier2_khr
    }
    ///Gets [`Self::queue_submit2_khr`]. See [`FNQueueSubmit2`] for more information.
    pub fn queue_submit2_khr(&self) -> FNQueueSubmit2 {
        self.queue_submit2_khr
    }
    ///Gets [`Self::cmd_write_timestamp2_khr`]. See [`FNCmdWriteTimestamp2`] for more information.
    pub fn cmd_write_timestamp2_khr(&self) -> FNCmdWriteTimestamp2 {
        self.cmd_write_timestamp2_khr
    }
}
