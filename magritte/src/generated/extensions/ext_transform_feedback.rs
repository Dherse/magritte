//![VK_EXT_transform_feedback](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_transform_feedback.html) - device extension
//!# Description
//!This extension adds transform feedback to the Vulkan API by exposing the
//!SPIR-V `TransformFeedback` and `GeometryStreams` capabilities to
//!capture vertex, tessellation or geometry shader outputs to one or more
//!buffers.
//!It adds API functionality to bind transform feedback buffers to capture the
//!primitives emitted by the graphics pipeline from SPIR-V outputs decorated
//!for transform feedback.
//!The transform feedback capture can be paused and resumed by way of storing
//!and retrieving a byte counter.
//!The captured data can be drawn again where the vertex count is derived from
//!the byte counter without CPU intervention.
//!If the implementation is capable, a vertex stream other than zero can be
//!rasterized.All these features are designed to match the full capabilities of OpenGL
//!core transform feedback functionality and beyond.
//!Many of the features are optional to allow base OpenGL ES GPUs to also
//!implement this extension.The primary purpose of the functionality exposed by this extension is
//! to
//!support translation layers from other 3D APIs.
//!This functionality is not considered forward looking, and is not expected to
//!be promoted to a KHR extension or to core Vulkan.
//!Unless this is needed for translation, it is recommended that developers use
//!alternative techniques of using the GPU to process and capture vertex data.
# ! [doc = concat ! ("# " , "Revision")]
//!1
# ! [doc = concat ! ("# " , "Dependencies")]
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
# ! [doc = concat ! ("# " , "Contacts")]
//! - Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_transform_feedback]
//!   @pdaniell-nv%0A<<Here describe the issue or question you have about the
//!   VK_EXT_transform_feedback extension>>)
# ! [doc = concat ! ("# " , "New commands")]
//! - [`cmd_begin_query_indexed_ext`]
//! - [`cmd_begin_transform_feedback_ext`]
//! - [`cmd_bind_transform_feedback_buffers_ext`]
//! - [`cmd_draw_indirect_byte_count_ext`]
//! - [`cmd_end_query_indexed_ext`]
//! - [`cmd_end_transform_feedback_ext`]
# ! [doc = concat ! ("# " , "New structures")]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceTransformFeedbackFeaturesEXT`]
//! - Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceTransformFeedbackPropertiesEXT`]
//! - Extending [`PipelineRasterizationStateCreateInfo`]:  -
//!   [`PipelineRasterizationStateStreamCreateInfoEXT`]
# ! [doc = concat ! ("# " , "New bitmasks")]
//! - [`PipelineRasterizationStateStreamCreateFlagsEXT`]
# ! [doc = concat ! ("# " , "New constants")]
//! - [`EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME`]
//! - [`EXT_TRANSFORM_FEEDBACK_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT`  -
//!   `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT`  -
//!   `VK_ACCESS_TRANSFORM_FEEDBACK_WRITE_BIT_EXT`
//! - Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT`  -
//!   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
//! - Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`  -
//!   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
# ! [doc = concat ! ("# " , "Known issues & F.A.Q.")]
//!1) Should we include pause/resume functionality? **RESOLVED** : Yes, this is needed to ease
//! layering other APIs which have this
//!functionality.
//!To pause use [`cmd_end_transform_feedback_ext`] and provide valid buffer
//!handles in the `pCounterBuffers` array and offsets in the
//!`pCounterBufferOffsets` array for the implementation to save the resume
//!points.
//!Then to resume use [`cmd_begin_transform_feedback_ext`] with the previous
//!`pCounterBuffers` and `pCounterBufferOffsets` values.
//!Between the pause and resume there needs to be a memory barrier for the
//!counter buffers with a source access of
//!`VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_WRITE_BIT_EXT` at pipeline stage
//!`VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT` to a destination access
//!of `VK_ACCESS_TRANSFORM_FEEDBACK_COUNTER_READ_BIT_EXT` at pipeline stage
//!`VK_PIPELINE_STAGE_TRANSFORM_FEEDBACK_BIT_EXT`.2) How does this interact with multiview?
//! **RESOLVED** : Transform feedback cannot be made active in a render pass with
//!multiview enabled.3) How should queries be done? **RESOLVED** : There is a new query type
//!`VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`.
//!A query pool created with this type will capture 2 integers -
//!numPrimitivesWritten and numPrimitivesNeeded - for the specified vertex
//!stream output from the last
//![pre-rasterization shader
//!stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
//!The vertex stream output queried is zero by default, but can be specified
//!with the new [`cmd_begin_query_indexed_ext`] and
//![`cmd_end_query_indexed_ext`] commands.
# ! [doc = concat ! ("# " , "Version history")]
//! - Revision 1, 2018-10-09 (Piers Daniell)  - Internal revisions
//!# Other info
//! * 2018-10-09
//! * - Baldur Karlsson, Valve  - Boris Zanin, Mobica  - Daniel Rakos, AMD  - Donald Scorgie,
//!   Imagination  - Henri Verbeet, CodeWeavers  - Jan-Harald Fredriksen, Arm  - Jason Ekstrand,
//!   Intel  - Jeff Bolz, NVIDIA  - Jesse Barker, Unity  - Jesse Hall, Google  - Pierre-Loup
//!   Griffais, Valve  - Philip Rebohle, DXVK  - Ruihao Zhang, Qualcomm  - Samuel Pitoiset, Valve  -
//!   Slawomir Grajewski, Intel  - Stu Smith, Imagination Technologies
//!# Related
//! - [`PhysicalDeviceTransformFeedbackFeaturesEXT`]
//! - [`PhysicalDeviceTransformFeedbackPropertiesEXT`]
//! - [`PipelineRasterizationStateStreamCreateFlagsEXT`]
//! - [`PipelineRasterizationStateStreamCreateInfoEXT`]
//! - [`cmd_begin_query_indexed_ext`]
//! - [`cmd_begin_transform_feedback_ext`]
//! - [`cmd_bind_transform_feedback_buffers_ext`]
//! - [`cmd_draw_indirect_byte_count_ext`]
//! - [`cmd_end_query_indexed_ext`]
//! - [`cmd_end_transform_feedback_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{
        BaseInStructure, BaseOutStructure, Bool32, Buffer, CommandBuffer, Device, DeviceSize, QueryControlFlags,
        QueryPool, StructureType,
    },
    AsRaw, Unique,
};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_SPEC_VERSION")]
pub const EXT_TRANSFORM_FEEDBACK_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME")]
pub const EXT_TRANSFORM_FEEDBACK_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_transform_feedback");
///[vkCmdBindTransformFeedbackBuffersEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) - Bind transform feedback buffers to a command buffer
///# C Specifications
///To bind transform feedback buffers to a command buffer for use in subsequent
///drawing commands, call:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdBindTransformFeedbackBuffersEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    firstBinding,
///    uint32_t                                    bindingCount,
///    const VkBuffer*                             pBuffers,
///    const VkDeviceSize*                         pOffsets,
///    const VkDeviceSize*                         pSizes);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`first_binding`] is the index of the first transform feedback binding whose state is updated
///   by the command.
/// - [`binding_count`] is the number of transform feedback bindings whose state is updated by the
///   command.
/// - [`p_buffers`] is a pointer to an array of buffer handles.
/// - [`p_offsets`] is a pointer to an array of buffer offsets.
/// - [`p_sizes`] is `NULL` or a pointer to an array of [`DeviceSize`] buffer sizes, specifying the
///   maximum number of bytes to capture to the corresponding transform feedback buffer. If
///   [`p_sizes`] is `NULL`, or the value of the [`p_sizes`] array element is [`WHOLE_SIZE`], then
///   the maximum number of bytes captured will be the size of the corresponding buffer minus the
///   buffer offset.
///# Description
///The values taken from elements i of [`p_buffers`], [`p_offsets`] and
///[`p_sizes`] replace the current state for the transform feedback binding
///[`first_binding`] +  i, for i in [0,
///[`binding_count`]).
///The transform feedback binding is updated to start at the offset indicated
///by [`p_offsets`][i] from the start of the buffer [`p_buffers`][i].
///## Valid Usage
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
/// - [`first_binding`] **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - The sum of [`first_binding`] and [`binding_count`] **must**  be less than or equal to
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - All elements of [`p_offsets`] **must**  be less than the size of the corresponding element in
///   [`p_buffers`]
/// - All elements of [`p_offsets`] **must**  be a multiple of 4
/// - All elements of [`p_buffers`] **must**  have been created with the
///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT` flag
/// - If the optional `pSize` array is specified, each element of [`p_sizes`] **must**  either be
///   [`WHOLE_SIZE`], or be less than or equal to
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffer_size`]
/// - All elements of [`p_sizes`] **must**  be either [`WHOLE_SIZE`], or less than or equal to the
///   size of the corresponding buffer in [`p_buffers`]
/// - All elements of [`p_offsets`] plus [`p_sizes`], where the [`p_sizes`], element is not
///   [`WHOLE_SIZE`],  **must**  be less than or equal to the size of the corresponding buffer in
///   [`p_buffers`]
/// - Each element of [`p_buffers`] that is non-sparse  **must**  be bound completely and
///   contiguously to a single [`DeviceMemory`] object
/// - Transform feedback  **must**  not be active when the
///   [`cmd_bind_transform_feedback_buffers_ext`] command is recorded
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`p_buffers`] **must**  be a valid pointer to an array of [`binding_count`] valid [`Buffer`]
///   handles
/// - [`p_offsets`] **must**  be a valid pointer to an array of [`binding_count`][`DeviceSize`]
///   values
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - [`binding_count`] **must**  be greater than `0`
/// - Both of [`command_buffer`], and the elements of [`p_buffers`] **must**  have been created,
///   allocated, or retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
pub type FNCmdBindTransformFeedbackBuffersExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_binding: u32,
        binding_count: u32,
        p_buffers: *const Buffer,
        p_offsets: *const DeviceSize,
        p_sizes: *const DeviceSize,
    ),
>;
///[vkCmdBeginTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) - Make transform feedback active in the command buffer
///# C Specifications
///Transform feedback for specific transform feedback buffers is made active by
///calling:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdBeginTransformFeedbackEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    firstCounterBuffer,
///    uint32_t                                    counterBufferCount,
///    const VkBuffer*                             pCounterBuffers,
///    const VkDeviceSize*                         pCounterBufferOffsets);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`first_counter_buffer`] is the index of the first transform feedback buffer corresponding to
///   [`p_counter_buffers`][0] and [`p_counter_buffer_offsets`][0].
/// - [`counter_buffer_count`] is the size of the [`p_counter_buffers`] and
///   [`p_counter_buffer_offsets`] arrays.
/// - [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter
///   buffers. Each buffer contains a 4 byte integer value representing the byte offset from the
///   start of the corresponding transform feedback buffer from where to start capturing vertex
///   data. If the byte offset stored to the counter buffer location was done using
///   [`cmd_end_transform_feedback_ext`] it can be used to resume transform feedback from the
///   previous location. If [`p_counter_buffers`] is `NULL`, then transform feedback will start
///   capturing vertex data to byte offset zero in all bound transform feedback buffers. For each
///   element of [`p_counter_buffers`] that is [`crate::Handle::null`], transform feedback will
///   start capturing vertex data to byte zero in the corresponding bound transform feedback buffer.
/// - [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values
///   specifying offsets within each of the [`p_counter_buffers`] where the counter values were
///   previously written. The location in each counter buffer at these offsets  **must**  be large
///   enough to contain 4 bytes of data. This data is the number of bytes captured by the previous
///   transform feedback to this buffer. If [`p_counter_buffer_offsets`] is `NULL`, then it is
///   assumed the offsets are zero.
///# Description
///The active transform feedback buffers will capture primitives emitted from
///the corresponding `XfbBuffer` in the bound graphics pipeline.
///Any `XfbBuffer` emitted that does not output to an active transform
///feedback buffer will not be captured.
///## Valid Usage
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
/// - Transform feedback  **must**  not be active
/// - [`first_counter_buffer`] **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or
///   equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`,
///   [`p_counter_buffers`] **must**  be a valid pointer to an array of
///   [`counter_buffer_count`][`Buffer`] handles that are either valid or [`crate::Handle::null`]
/// - For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**
///   reference a buffer large enough to hold 4 bytes at the corresponding offset from the
///   [`p_counter_buffer_offsets`] array
/// - If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
/// - For each buffer handle in the [`p_counter_buffers`] array that is not [`crate::Handle::null`]
///   it  **must**  have been created with a `usage` value containing
///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
/// - A valid graphics pipeline  **must**  be bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization)
///   of the bound graphics pipeline  **must**  have been declared with the `Xfb` execution mode
/// - Transform feedback  **must**  not be made active in a render pass instance with multiview
///   enabled
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffer_offsets`] is not `NULL`,
///   [`p_counter_buffer_offsets`] **must**  be a valid pointer to an array of
///   [`counter_buffer_count`][`DeviceSize`] values
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
/// - Both of [`command_buffer`], and the elements of [`p_counter_buffers`] that are valid handles
///   of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
pub type FNCmdBeginTransformFeedbackExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ),
>;
///[vkCmdEndTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) - Make transform feedback inactive in the command buffer
///# C Specifications
///Transform feedback for specific transform feedback buffers is made inactive
///by calling:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdEndTransformFeedbackEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    firstCounterBuffer,
///    uint32_t                                    counterBufferCount,
///    const VkBuffer*                             pCounterBuffers,
///    const VkDeviceSize*                         pCounterBufferOffsets);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`first_counter_buffer`] is the index of the first transform feedback buffer corresponding to
///   [`p_counter_buffers`][0] and [`p_counter_buffer_offsets`][0].
/// - [`counter_buffer_count`] is the size of the [`p_counter_buffers`] and
///   [`p_counter_buffer_offsets`] arrays.
/// - [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter
///   buffers. The counter buffers are used to record the current byte positions of each transform
///   feedback buffer where the next vertex output data would be captured. This  **can**  be used by
///   a subsequent [`cmd_begin_transform_feedback_ext`] call to resume transform feedback capture
///   from this position. It can also be used by [`cmd_draw_indirect_byte_count_ext`] to determine
///   the vertex count of the draw call.
/// - [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values
///   specifying offsets within each of the [`p_counter_buffers`] where the counter values can be
///   written. The location in each counter buffer at these offsets  **must**  be large enough to
///   contain 4 bytes of data. The data stored at this location is the byte offset from the start of
///   the transform feedback buffer binding where the next vertex data would be written. If
///   [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets are zero.
///# Description
///## Valid Usage
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
/// - Transform feedback  **must**  be active
/// - [`first_counter_buffer`] **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or
///   equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
/// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`,
///   [`p_counter_buffers`] **must**  be a valid pointer to an array of
///   [`counter_buffer_count`][`Buffer`] handles that are either valid or [`crate::Handle::null`]
/// - For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**
///   reference a buffer large enough to hold 4 bytes at the corresponding offset from the
///   [`p_counter_buffer_offsets`] array
/// - If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
/// - For each buffer handle in the [`p_counter_buffers`] array that is not [`crate::Handle::null`]
///   it  **must**  have been created with a `usage` value containing
///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffer_offsets`] is not `NULL`,
///   [`p_counter_buffer_offsets`] **must**  be a valid pointer to an array of
///   [`counter_buffer_count`][`DeviceSize`] values
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
/// - Both of [`command_buffer`], and the elements of [`p_counter_buffers`] that are valid handles
///   of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same
///   [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEndTransformFeedbackEXT")]
pub type FNCmdEndTransformFeedbackExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        first_counter_buffer: u32,
        counter_buffer_count: u32,
        p_counter_buffers: *const Buffer,
        p_counter_buffer_offsets: *const DeviceSize,
    ),
>;
///[vkCmdBeginQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) - Begin an indexed query
///# C Specifications
///To begin an indexed query, call:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdBeginQueryIndexedEXT(
///    VkCommandBuffer                             commandBuffer,
///    VkQueryPool                                 queryPool,
///    uint32_t                                    query,
///    VkQueryControlFlags                         flags,
///    uint32_t                                    index);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which this command will be recorded.
/// - [`query_pool`] is the query pool that will manage the results of the query.
/// - [`query`] is the query index within the query pool that will contain the results.
/// - [`flags`] is a bitmask of [`QueryControlFlagBits`] specifying constraints on the types of
///   queries that  **can**  be performed.
/// - [`index`] is the query type specific index. When the query type is
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the index represents the vertex stream.
///# Description
///The [`cmd_begin_query_indexed_ext`] command operates the same as the
///[`cmd_begin_query`] command, except that it also accepts a query type
///specific [`index`] parameter.
///## Valid Usage
/// - All queries used by the command  **must**  be unavailable
/// - The `queryType` used to create [`query_pool`] **must**  not be `VK_QUERY_TYPE_TIMESTAMP`
/// - The `queryType` used to create [`query_pool`] **must**  not be
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR` or
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
/// - The `queryType` used to create [`query_pool`] **must**  not be
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
/// - If the [precise occlusion queries]() feature is not enabled, or the `queryType` used to create
///   [`query_pool`] was not `VK_QUERY_TYPE_OCCLUSION`, [`flags`] **must**  not contain
///   `VK_QUERY_CONTROL_PRECISE_BIT`
/// - [`query`] **must**  be less than the number of queries in [`query_pool`]
/// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_OCCLUSION`, the
///   [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS` and
///   any of the `pipelineStatistics` indicate graphics operations, the [`CommandPool`] that
///   [`command_buffer`] was allocated from  **must**  support graphics operations
/// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS` and
///   any of the `pipelineStatistics` indicate compute operations, the [`CommandPool`] that
///   [`command_buffer`] was allocated from  **must**  support compute operations
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - If called within a render pass instance, the sum of [`query`] and the number of bits set in
///   the current subpass’s view mask  **must**  be less than or equal to the number of queries in
///   [`query_pool`]
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR` the [`CommandPool`] that
///   [`command_buffer`] was allocated from  **must**  support [video encode operations]()
/// - If the [`query_pool`] was created with the same `queryType` as that of another [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
///   query within [`command_buffer`], then [`index`] **must**  not match the index used for the
///   active query
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`CommandPool`] that [`command_buffer`] was
///   allocated from  **must**  support graphics operations
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
/// - If the `queryType` used to create [`query_pool`] was not
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` then
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_queries`] **must**  be
///   supported
///
/// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the
///   [profiling lock]() **must**  have been held before [`begin_command_buffer`] was called on
///   [`command_buffer`]
/// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and
///   one of the counters used to create [`query_pool`] was
///   `VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR`, the query begin  **must**  be the first
///   recorded command in [`command_buffer`]
/// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and
///   one of the counters used to create [`query_pool`] was
///   `VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR`, the begin command  **must**  not be recorded
///   within a render pass instance
/// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and
///   another query pool with a `queryType``VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` has been used
///   within [`command_buffer`], its parent primary command buffer or secondary command buffer
///   recorded within the same parent primary command buffer as [`command_buffer`], the
///   [`performanceCounterMultipleQueryPools`]() feature  **must**  be enabled
/// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`,
///   this command  **must**  not be recorded in a command buffer that, either directly or through
///   secondary command buffers, also contains a [`cmd_reset_query_pool`] command affecting the same
///   query
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
/// - [`flags`] **must**  be a valid combination of [`QueryControlFlagBits`] values
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`CommandBuffer`]
/// - [`QueryControlFlags`]
/// - [`QueryPool`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdBeginQueryIndexedEXT")]
pub type FNCmdBeginQueryIndexedExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ),
>;
///[vkCmdEndQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html) - Ends a query
///# C Specifications
///To end an indexed query after the set of desired drawing or dispatching
///commands is recorded, call:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdEndQueryIndexedEXT(
///    VkCommandBuffer                             commandBuffer,
///    VkQueryPool                                 queryPool,
///    uint32_t                                    query,
///    uint32_t                                    index);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which this command will be recorded.
/// - [`query_pool`] is the query pool that is managing the results of the query.
/// - [`query`] is the query index within the query pool where the result is stored.
/// - [`index`] is the query type specific index.
///# Description
///The [`cmd_end_query_indexed_ext`] command operates the same as the
///[`cmd_end_query`] command, except that it also accepts a query type
///specific [`index`] parameter.
///## Valid Usage
/// - All queries used by the command  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
/// - [`query`] **must**  be less than the number of queries in [`query_pool`]
/// - [`command_buffer`] **must**  not be a protected command buffer
/// - If [`cmd_end_query_indexed_ext`] is called within a render pass instance, the sum of [`query`]
///   and the number of bits set in the current subpass’s view mask  **must**  be less than or equal
///   to the number of queries in [`query_pool`]
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
/// - If the `queryType` used to create [`query_pool`] was not
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
/// - If the `queryType` used to create [`query_pool`] was
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`[`index`] **must**  equal the [`index`] used to
///   begin the query
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or
///   compute operations
/// - Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`CommandBuffer`]
/// - [`QueryPool`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdEndQueryIndexedEXT")]
pub type FNCmdEndQueryIndexedExt =
    Option<unsafe extern "system" fn(command_buffer: CommandBuffer, query_pool: QueryPool, query: u32, index: u32)>;
///[vkCmdDrawIndirectByteCountEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) - Draw primitives with indirect parameters where the vertex count is derived from the counter byte value in the counter buffer
///# C Specifications
///To record a non-indexed draw call, where the vertex count is based on a byte
///count read from a buffer and the passed in vertex stride parameter, call:
///```c
///// Provided by VK_EXT_transform_feedback
///void vkCmdDrawIndirectByteCountEXT(
///    VkCommandBuffer                             commandBuffer,
///    uint32_t                                    instanceCount,
///    uint32_t                                    firstInstance,
///    VkBuffer                                    counterBuffer,
///    VkDeviceSize                                counterBufferOffset,
///    uint32_t                                    counterOffset,
///    uint32_t                                    vertexStride);
///```
///# Parameters
/// - [`command_buffer`] is the command buffer into which the command is recorded.
/// - [`instance_count`] is the number of instances to draw.
/// - [`first_instance`] is the instance ID of the first instance to draw.
/// - [`counter_buffer`] is the buffer handle from where the byte count is read.
/// - [`counter_buffer_offset`] is the offset into the buffer used to read the byte count, which is
///   used to calculate the vertex count for this draw call.
/// - [`counter_offset`] is subtracted from the byte count read from the [`counter_buffer`] at the
///   [`counter_buffer_offset`]
/// - [`vertex_stride`] is the stride in bytes between each element of the vertex data that is used
///   to calculate the vertex count from the counter value. This value is typically the same value
///   that was used in the graphics pipeline state when the transform feedback was captured as the
///   `XfbStride`.
///# Description
///When the command is executed, primitives are assembled in the same way as
///done with [`cmd_draw`] except the `vertexCount` is calculated based
///on the byte count read from [`counter_buffer`] at offset
///[`counter_buffer_offset`].
///The assembled primitives execute the bound graphics pipeline.The effective `vertexCount` is
/// calculated as follows:
///```c
///const uint32_t * counterBufferPtr = (const uint8_t *)counterBuffer.address +
/// counterBufferOffset;
///vertexCount = floor(max(0, (*counterBufferPtr - counterOffset)) / vertexStride);
///```
///The effective `firstVertex` is zero.
///## Valid Usage
/// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
///   command, then the image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
/// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format features]()
///   **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
/// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
/// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then the
///   image view’s [format features]() **must**  contain
///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this command
///   **must**  have a [`ImageViewType`] and format that supports cubic filtering together with
///   minmax filtering, as specified by
///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
///   [`get_physical_device_image_format_properties2`]
/// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**  only be
///   sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
/// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
/// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format feature
///   **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
/// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind point
///   used by this command, a descriptor set  **must**  have been bound to *n* at the same pipeline
///   bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
///   [[descriptorsets-compatibility]]()
/// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command, a
///   push constant value  **must**  have been set for the same pipeline bind point, with a
///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used to
///   create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
/// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline bind
///   point used by this command
/// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires any
///   dynamic state, that state  **must**  have been set or inherited (if the
///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done so
///   after any previously bound pipeline with the corresponding state not specified as dynamic
/// - There  **must**  not have been any calls to dynamic state setting commands for any state not
///   specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used by this
///   command, since that pipeline was bound
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used to
///   sample from any [`Image`] with a [`ImageView`] of the type `VK_IMAGE_VIEW_TYPE_3D`,
///   `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`, `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or
///   `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be used
///   with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that includes a
///   LOD bias or any offset values, in any shader stage
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a uniform buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object bound
///   to the pipeline bind point used by this command accesses a storage buffer, it  **must**  not
///   access values outside of the range of the buffer as specified in the descriptor set bound to
///   the same pipeline bind point
/// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind point
///   used by this command  **must**  not be a protected resource
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  only be used with `OpImageSample*` or
///   `OpImageSparseSample*` instructions
/// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses a
///   [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
/// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the image view’s format
/// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many components
///   as the buffer view’s format
/// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a result
///   of this command, the `SampledType` of the `OpTypeImage` operand of that instruction  **must**
///   have a `Width` of 64
/// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
///   **must**  have a `Width` of 64
/// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is accessed
///   as a result of this command, the `SampledType` of the `OpTypeImage` operand of that
///   instruction  **must**  have a `Width` of 32
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created with
///   the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created with
///   the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
///   command
/// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - The subpass index of the current render pass  **must**  be equal to the `subpass` member of
///   the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
/// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via a
///   descriptor set
/// - Memory backing image subresources used as attachments in the current render pass  **must**
///   not be written in any way other than as an attachment by this command
/// - If any recorded command in the current subpass will write to an image subresource as an
///   attachment, this command  **must**  not read from the memory backing that image subresource in
///   any other way than as an attachment
/// - If any recorded command in the current subpass will read from an image subresource used as an
///   attachment in any way other than as an attachment, this command  **must**  not write to that
///   image subresource as an attachment
/// - If the draw is recorded in a render pass instance with multiview enabled, the maximum instance
///   index  **must**  be less than or equal to
///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
/// - If the bound graphics pipeline was created with
///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and the
///   current subpass has a depth/stencil attachment, then that attachment  **must**  have been
///   created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `viewportCount` parameter of [`cmd_set_viewport_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer prior
///   to this drawing command, and the `scissorCount` parameter of [`cmd_set_scissor_with_count`]
///   **must**  match the [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
/// - If the bound graphics pipeline state was created with both the
///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   states enabled then both [`cmd_set_viewport_with_count`] and [`cmd_set_scissor_with_count`]
///   **must**  have been called in the current command buffer prior to this drawing command, and
///   the `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  match the
///   `scissorCount` parameter of [`cmd_set_scissor_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
///   pipeline  **must**  have been created with
///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic
///   states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the `viewportCount`
///   parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
///   graphics pipeline  **must**  have been created with
///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV`
///   dynamic states enabled then the `viewportCount` parameter in the last call to
///   [`cmd_set_viewport_shading_rate_palette_nv`] **must**  be greater than or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal to the
///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been created
///   with [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE`
///   dynamic state enabled then [`cmd_set_depth_bias_enable`] **must**  have been called in the
///   current command buffer prior to this drawing command
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the current
///   command buffer prior to this drawing command and the `logicOp` **must**  be a valid
///   [`LogicOp`] value
/// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
///   state enabled, and any of the shader stages of the bound graphics pipeline write to the
///   `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**  have been
///   called in the current command buffer prior to this drawing command, and the `viewportCount`
///   parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
/// - If rasterization is not disabled in the bound graphics pipeline, then for each color
///   attachment in the subpass, if the corresponding image view’s [format features]() do not
///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of the
///   corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
///   [`FALSE`]
/// - If rasterization is not disabled in the bound graphics pipeline, and neither the
///   `[`amd_mixed_attachment_samples`]` nor the `[`nv_framebuffer_mixed_samples`]` extensions are
///   enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**  be the
///   same as the current subpass color and/or depth/stencil attachments
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command
///   **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this
///   command  **must**  not write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this
///   command  **must**  not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command  **must**  not
///   write any values to the depth attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the `imageView`
///   member of `pStencilAttachment` is not [`crate::Handle::null`], and the `layout` member of
///   `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**
///   not write any values to the stencil attachment
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound graphics pipeline  **must**  have been created with a
///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
///   [`RenderingInfo::color_attachment_count`]
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
///   create the currently bound graphics pipeline
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `attachmentCount` parameter of
///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
///   pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the currently
///   bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
/// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not [`crate::Handle::null`],
///   the currently bound graphics pipeline  **must**  have been created with
///   `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
/// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the corresponding element of the `pColorAttachmentSamples` member of
///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
///   currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of the `depthStencilAttachmentSamples` member of [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] used to create the currently bound graphics pipeline  **must**
///   be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun with
///   [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter greater
///   than `0`, then each element of the [`RenderingInfo::color_attachments`] array with a
///   `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a sample
///   count equal to the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used
///   to create the currently bound graphics pipeline
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
///   [`AttachmentSampleCountInfoNV`] structure, and
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`], the
///   value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the
///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
/// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
///   bound pipeline  **must**  have been created with a [`GraphicsPipelineCreateInfo::render_pass`]
///   equal to [`crate::Handle::null`]
///
/// - All vertex input bindings accessed via vertex input variables declared in the vertex shader
///   entry point’s interface  **must**  have either valid or [`crate::Handle::null`] buffers bound
/// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
///   vertex input variables declared in the vertex shader entry point’s interface  **must**  not be
///   [`crate::Handle::null`]
/// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely contained
///   within the corresponding vertex buffer binding, as described in [[fxvertex-input]]()
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command, and the `primitiveTopology` parameter of
///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the pipeline
///   [`PipelineInputAssemblyStateCreateInfo::topology`] state
/// - If the bound graphics pipeline was created with both the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic states enabled, then
///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer prior to
///   this draw command
/// - If the bound graphics pipeline was created with the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
///   prior to this draw command, and the `pStrides` parameter of [`cmd_bind_vertex_buffers2_ext`]
///   **must**  not be `NULL`
/// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT`
///   dynamic state enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in the
///   current command buffer prior to this draw command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command buffer
///   prior to this drawing command
/// - If the bound graphics pipeline state was created with the
///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
///   buffer prior to this drawing command
/// - The bound graphics pipeline  **must**  not have been created with the
///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
///   `VK_SHADER_STAGE_MESH_BIT_NV`
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
/// - The implementation  **must**  support
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_draw`]
/// - [`vertex_stride`] **must**  be greater than 0 and less than or equal to
///   [`PhysicalDeviceLimits`]`::maxTransformFeedbackBufferDataStride`
/// - If [`counter_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to
///   a single [`DeviceMemory`] object
/// - [`counter_buffer`] **must**  have been created with the `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT`
///   bit set
/// - [`counter_buffer_offset`] **must**  be a multiple of `4`
/// - [`command_buffer`] **must**  not be a protected command buffer
///
///## Valid Usage (Implicit)
/// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
/// - [`counter_buffer`] **must**  be a valid [`Buffer`] handle
/// - [`command_buffer`] **must**  be in the [recording state]()
/// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
///   operations
/// - This command  **must**  only be called inside of a render pass instance
/// - Both of [`command_buffer`], and [`counter_buffer`] **must**  have been created, allocated, or
///   retrieved from the same [`Device`]
///
///## Host Synchronization
/// - Host access to [`command_buffer`] **must**  be externally synchronized
/// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be
///   externally synchronized
///
///## Command Properties
///# Related
/// - [`ext_transform_feedback`]
/// - [`Buffer`]
/// - [`CommandBuffer`]
/// - [`DeviceSize`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
pub type FNCmdDrawIndirectByteCountExt = Option<
    unsafe extern "system" fn(
        command_buffer: CommandBuffer,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ),
>;
///[VkPipelineRasterizationStateStreamCreateFlagsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateFlagsEXT.html) - Reserved for future use
///# C Specifications
///```c
///// Provided by VK_EXT_transform_feedback
///typedef VkFlags VkPipelineRasterizationStateStreamCreateFlagsEXT;
///```
///# Related
/// - [`ext_transform_feedback`]
/// - [`PipelineRasterizationStateStreamCreateInfoEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct PipelineRasterizationStateStreamCreateFlagsEXT(u32);
impl const Default for PipelineRasterizationStateStreamCreateFlagsEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PipelineRasterizationStateStreamCreateFlagsEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(PipelineRasterizationStateStreamCreateFlagsEXT))
            .field(&self.0)
            .finish()
    }
}
///[VkPhysicalDeviceTransformFeedbackFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackFeaturesEXT.html) - Structure describing transform feedback features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTransformFeedbackFeaturesEXT`] structure is defined
///as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           transformFeedback;
///    VkBool32           geometryStreams;
///} VkPhysicalDeviceTransformFeedbackFeaturesEXT;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`transform_feedback`] indicates whether the implementation supports transform feedback and
///   shader modules  **can**  declare the `TransformFeedback` capability.
/// - [`geometry_streams`] indicates whether the implementation supports the `GeometryStreams`
///   SPIR-V capability.
///If the [`PhysicalDeviceTransformFeedbackFeaturesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`get_physical_device_features2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceTransformFeedbackFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT`
///# Related
/// - [`ext_transform_feedback`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceTransformFeedbackFeaturesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`transform_feedback`] indicates whether
    ///the implementation supports transform feedback and shader modules  **can**
    ///declare the `TransformFeedback` capability.
    pub transform_feedback: Bool32,
    ///[`geometry_streams`] indicates whether the
    ///implementation supports the `GeometryStreams` SPIR-V capability.
    pub geometry_streams: Bool32,
}
impl<'lt> Default for PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_FEATURES_EXT,
            p_next: std::ptr::null_mut(),
            transform_feedback: 0,
            geometry_streams: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTransformFeedbackFeaturesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::transform_feedback`]
    pub fn transform_feedback_raw(&self) -> Bool32 {
        self.transform_feedback
    }
    ///Gets the raw value of [`Self::geometry_streams`]
    pub fn geometry_streams_raw(&self) -> Bool32 {
        self.geometry_streams
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback`]
    pub fn set_transform_feedback_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback = value;
        self
    }
    ///Sets the raw value of [`Self::geometry_streams`]
    pub fn set_geometry_streams_raw(&mut self, value: Bool32) -> &mut Self {
        self.geometry_streams = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback`]
    pub fn with_transform_feedback_raw(mut self, value: Bool32) -> Self {
        self.transform_feedback = value;
        self
    }
    ///Sets the raw value of [`Self::geometry_streams`]
    pub fn with_geometry_streams_raw(mut self, value: Bool32) -> Self {
        self.geometry_streams = value;
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
    ///Gets the value of [`Self::transform_feedback`]
    pub fn transform_feedback(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback as u8) }
    }
    ///Gets the value of [`Self::geometry_streams`]
    pub fn geometry_streams(&self) -> bool {
        unsafe { std::mem::transmute(self.geometry_streams as u8) }
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
    ///Gets a mutable reference to the value of [`Self::transform_feedback`]
    pub fn transform_feedback_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::geometry_streams`]
    pub fn geometry_streams_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.geometry_streams as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.geometry_streams as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::transform_feedback`]
    pub fn set_transform_feedback(&mut self, value: bool) -> &mut Self {
        self.transform_feedback = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::geometry_streams`]
    pub fn set_geometry_streams(&mut self, value: bool) -> &mut Self {
        self.geometry_streams = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::transform_feedback`]
    pub fn with_transform_feedback(mut self, value: bool) -> Self {
        self.transform_feedback = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::geometry_streams`]
    pub fn with_geometry_streams(mut self, value: bool) -> Self {
        self.geometry_streams = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceTransformFeedbackPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceTransformFeedbackPropertiesEXT.html) - Structure describing transform feedback properties that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
///    VkStructureType    sType;
///    void*              pNext;
///    uint32_t           maxTransformFeedbackStreams;
///    uint32_t           maxTransformFeedbackBuffers;
///    VkDeviceSize       maxTransformFeedbackBufferSize;
///    uint32_t           maxTransformFeedbackStreamDataSize;
///    uint32_t           maxTransformFeedbackBufferDataSize;
///    uint32_t           maxTransformFeedbackBufferDataStride;
///    VkBool32           transformFeedbackQueries;
///    VkBool32           transformFeedbackStreamsLinesTriangles;
///    VkBool32           transformFeedbackRasterizationStreamSelect;
///    VkBool32           transformFeedbackDraw;
///} VkPhysicalDeviceTransformFeedbackPropertiesEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_transform_feedback_streams`] is the maximum number of vertex streams that can be output
///   from geometry shaders declared with the `GeometryStreams` capability. If the implementation
///   does not support [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`] then
///   [`max_transform_feedback_streams`] **must**  be set to `1`.
/// - [`max_transform_feedback_buffers`] is the maximum number of transform feedback buffers that can be bound for capturing shader outputs from the last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
/// - [`max_transform_feedback_buffer_size`] is the maximum size that can be specified when binding
///   a buffer for transform feedback in [`cmd_bind_transform_feedback_buffers_ext`].
/// - [`max_transform_feedback_stream_data_size`] is the maximum amount of data in bytes for each
///   vertex that captured to one or more transform feedback buffers associated with a specific
///   vertex stream.
/// - [`max_transform_feedback_buffer_data_size`] is the maximum amount of data in bytes for each
///   vertex that can be captured to a specific transform feedback buffer.
/// - [`max_transform_feedback_buffer_data_stride`] is the maximum stride between each capture of
///   vertex data to the buffer.
/// - [`transform_feedback_queries`] is [`TRUE`] if the implementation supports the
///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` query type. [`transform_feedback_queries`] is
///   [`FALSE`] if queries of this type  **cannot**  be created.
/// - [`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the implementation supports the
///   geometry shader `OpExecutionMode` of `OutputLineStrip` and `OutputTriangleStrip` in addition
///   to `OutputPoints` when more than one vertex stream is output. If
///   [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the implementation only supports
///   an `OpExecutionMode` of `OutputPoints` when more than one vertex stream is output from the
///   geometry shader.
/// - [`transform_feedback_rasterization_stream_select`] is [`TRUE`] if the implementation supports
///   the `GeometryStreams` SPIR-V capability and the application can use
///   [`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which vertex stream output is used
///   for rasterization. Otherwise vertex stream `0` **must**  always be used for rasterization.
/// - [`transform_feedback_draw`] is [`TRUE`] if the implementation supports the
///   [`cmd_draw_indirect_byte_count_ext`] function otherwise the function  **must**  not be called.
///# Description
///If the [`PhysicalDeviceTransformFeedbackPropertiesEXT`] structure is included in the [`p_next`]
/// chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`get_physical_device_properties2`], it is filled in with each
///corresponding implementation-dependent property.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT`
///# Related
/// - [`ext_transform_feedback`]
/// - [`Bool32`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceTransformFeedbackPropertiesEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_transform_feedback_streams`]
    ///is the maximum number of vertex streams that can be output from geometry
    ///shaders declared with the `GeometryStreams` capability.
    ///If the implementation does not support
    ///[`PhysicalDeviceTransformFeedbackFeaturesEXT`]::`geometryStreams`
    ///then [`max_transform_feedback_streams`] **must**  be set to `1`.
    pub max_transform_feedback_streams: u32,
    ///[`max_transform_feedback_buffers`]
    ///is the maximum number of transform feedback buffers that can be bound
    ///for capturing shader outputs from the last
    ///[pre-rasterization shader
    ///stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization).
    pub max_transform_feedback_buffers: u32,
    ///[`max_transform_feedback_buffer_size`] is the maximum size that can be
    ///specified when binding a buffer for transform feedback in
    ///[`cmd_bind_transform_feedback_buffers_ext`].
    pub max_transform_feedback_buffer_size: DeviceSize,
    ///[`max_transform_feedback_stream_data_size`] is the maximum amount of data
    ///in bytes for each vertex that captured to one or more transform feedback
    ///buffers associated with a specific vertex stream.
    pub max_transform_feedback_stream_data_size: u32,
    ///[`max_transform_feedback_buffer_data_size`] is the maximum amount of data
    ///in bytes for each vertex that can be captured to a specific transform
    ///feedback buffer.
    pub max_transform_feedback_buffer_data_size: u32,
    ///[`max_transform_feedback_buffer_data_stride`] is the maximum stride between
    ///each capture of vertex data to the buffer.
    pub max_transform_feedback_buffer_data_stride: u32,
    ///[`transform_feedback_queries`] is
    ///[`TRUE`] if the implementation supports the
    ///`VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` query type.
    ///[`transform_feedback_queries`] is [`FALSE`] if queries of this type
    /// **cannot**  be created.
    pub transform_feedback_queries: Bool32,
    ///[`transform_feedback_streams_lines_triangles`] is [`TRUE`] if the
    ///implementation supports the geometry shader `OpExecutionMode` of
    ///`OutputLineStrip` and `OutputTriangleStrip` in addition to
    ///`OutputPoints` when more than one vertex stream is output.
    ///If [`transform_feedback_streams_lines_triangles`] is [`FALSE`] the
    ///implementation only supports an `OpExecutionMode` of
    ///`OutputPoints` when more than one vertex stream is output from the
    ///geometry shader.
    pub transform_feedback_streams_lines_triangles: Bool32,
    ///[`transform_feedback_rasterization_stream_select`] is [`TRUE`] if the
    ///implementation supports the `GeometryStreams` SPIR-V capability and
    ///the application can use
    ///[`PipelineRasterizationStateStreamCreateInfoEXT`] to modify which
    ///vertex stream output is used for rasterization.
    ///Otherwise vertex stream `0` **must**  always be used for rasterization.
    pub transform_feedback_rasterization_stream_select: Bool32,
    ///[`transform_feedback_draw`] is
    ///[`TRUE`] if the implementation supports the
    ///[`cmd_draw_indirect_byte_count_ext`] function otherwise the function
    /// **must**  not be called.
    pub transform_feedback_draw: Bool32,
}
impl<'lt> Default for PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PHYSICAL_DEVICE_TRANSFORM_FEEDBACK_PROPERTIES_EXT,
            p_next: std::ptr::null_mut(),
            max_transform_feedback_streams: 0,
            max_transform_feedback_buffers: 0,
            max_transform_feedback_buffer_size: Default::default(),
            max_transform_feedback_stream_data_size: 0,
            max_transform_feedback_buffer_data_size: 0,
            max_transform_feedback_buffer_data_stride: 0,
            transform_feedback_queries: 0,
            transform_feedback_streams_lines_triangles: 0,
            transform_feedback_rasterization_stream_select: 0,
            transform_feedback_draw: 0,
        }
    }
}
impl<'lt> PhysicalDeviceTransformFeedbackPropertiesEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *mut BaseOutStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries_raw(&self) -> Bool32 {
        self.transform_feedback_queries
    }
    ///Gets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles_raw(&self) -> Bool32 {
        self.transform_feedback_streams_lines_triangles
    }
    ///Gets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select_raw(&self) -> Bool32 {
        self.transform_feedback_rasterization_stream_select
    }
    ///Gets the raw value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw_raw(&self) -> Bool32 {
        self.transform_feedback_draw
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_queries`]
    pub fn set_transform_feedback_queries_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_queries = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn set_transform_feedback_streams_lines_triangles_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_streams_lines_triangles = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn set_transform_feedback_rasterization_stream_select_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_rasterization_stream_select = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_draw`]
    pub fn set_transform_feedback_draw_raw(&mut self, value: Bool32) -> &mut Self {
        self.transform_feedback_draw = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *mut BaseOutStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_queries`]
    pub fn with_transform_feedback_queries_raw(mut self, value: Bool32) -> Self {
        self.transform_feedback_queries = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn with_transform_feedback_streams_lines_triangles_raw(mut self, value: Bool32) -> Self {
        self.transform_feedback_streams_lines_triangles = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn with_transform_feedback_rasterization_stream_select_raw(mut self, value: Bool32) -> Self {
        self.transform_feedback_rasterization_stream_select = value;
        self
    }
    ///Sets the raw value of [`Self::transform_feedback_draw`]
    pub fn with_transform_feedback_draw_raw(mut self, value: Bool32) -> Self {
        self.transform_feedback_draw = value;
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
    ///Gets the value of [`Self::max_transform_feedback_streams`]
    pub fn max_transform_feedback_streams(&self) -> u32 {
        self.max_transform_feedback_streams
    }
    ///Gets the value of [`Self::max_transform_feedback_buffers`]
    pub fn max_transform_feedback_buffers(&self) -> u32 {
        self.max_transform_feedback_buffers
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn max_transform_feedback_buffer_size(&self) -> DeviceSize {
        self.max_transform_feedback_buffer_size
    }
    ///Gets the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn max_transform_feedback_stream_data_size(&self) -> u32 {
        self.max_transform_feedback_stream_data_size
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn max_transform_feedback_buffer_data_size(&self) -> u32 {
        self.max_transform_feedback_buffer_data_size
    }
    ///Gets the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn max_transform_feedback_buffer_data_stride(&self) -> u32 {
        self.max_transform_feedback_buffer_data_stride
    }
    ///Gets the value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_queries as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_streams_lines_triangles as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_rasterization_stream_select as u8) }
    }
    ///Gets the value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw(&self) -> bool {
        unsafe { std::mem::transmute(self.transform_feedback_draw as u8) }
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
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_streams`]
    pub fn max_transform_feedback_streams_mut(&mut self) -> &mut u32 {
        &mut self.max_transform_feedback_streams
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffers`]
    pub fn max_transform_feedback_buffers_mut(&mut self) -> &mut u32 {
        &mut self.max_transform_feedback_buffers
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn max_transform_feedback_buffer_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.max_transform_feedback_buffer_size
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn max_transform_feedback_stream_data_size_mut(&mut self) -> &mut u32 {
        &mut self.max_transform_feedback_stream_data_size
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn max_transform_feedback_buffer_data_size_mut(&mut self) -> &mut u32 {
        &mut self.max_transform_feedback_buffer_data_size
    }
    ///Gets a mutable reference to the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn max_transform_feedback_buffer_data_stride_mut(&mut self) -> &mut u32 {
        &mut self.max_transform_feedback_buffer_data_stride
    }
    ///Gets a mutable reference to the value of [`Self::transform_feedback_queries`]
    pub fn transform_feedback_queries_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_queries as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_queries as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_streams_lines_triangles`]
    pub fn transform_feedback_streams_lines_triangles_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_streams_lines_triangles as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_streams_lines_triangles as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::transform_feedback_rasterization_stream_select`]
    pub fn transform_feedback_rasterization_stream_select_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_rasterization_stream_select as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_rasterization_stream_select as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::transform_feedback_draw`]
    pub fn transform_feedback_draw_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.transform_feedback_draw as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.transform_feedback_draw as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_streams`]
    pub fn set_max_transform_feedback_streams(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_streams = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffers`]
    pub fn set_max_transform_feedback_buffers(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffers = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn set_max_transform_feedback_buffer_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.max_transform_feedback_buffer_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn set_max_transform_feedback_stream_data_size(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_stream_data_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn set_max_transform_feedback_buffer_data_size(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn set_max_transform_feedback_buffer_data_stride(&mut self, value: u32) -> &mut Self {
        self.max_transform_feedback_buffer_data_stride = value;
        self
    }
    ///Sets the value of [`Self::transform_feedback_queries`]
    pub fn set_transform_feedback_queries(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_queries = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn set_transform_feedback_streams_lines_triangles(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_streams_lines_triangles = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn set_transform_feedback_rasterization_stream_select(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_rasterization_stream_select = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_draw`]
    pub fn set_transform_feedback_draw(&mut self, value: bool) -> &mut Self {
        self.transform_feedback_draw = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_streams`]
    pub fn with_max_transform_feedback_streams(mut self, value: u32) -> Self {
        self.max_transform_feedback_streams = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffers`]
    pub fn with_max_transform_feedback_buffers(mut self, value: u32) -> Self {
        self.max_transform_feedback_buffers = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_size`]
    pub fn with_max_transform_feedback_buffer_size(mut self, value: crate::vulkan1_0::DeviceSize) -> Self {
        self.max_transform_feedback_buffer_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_stream_data_size`]
    pub fn with_max_transform_feedback_stream_data_size(mut self, value: u32) -> Self {
        self.max_transform_feedback_stream_data_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_data_size`]
    pub fn with_max_transform_feedback_buffer_data_size(mut self, value: u32) -> Self {
        self.max_transform_feedback_buffer_data_size = value;
        self
    }
    ///Sets the value of [`Self::max_transform_feedback_buffer_data_stride`]
    pub fn with_max_transform_feedback_buffer_data_stride(mut self, value: u32) -> Self {
        self.max_transform_feedback_buffer_data_stride = value;
        self
    }
    ///Sets the value of [`Self::transform_feedback_queries`]
    pub fn with_transform_feedback_queries(mut self, value: bool) -> Self {
        self.transform_feedback_queries = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_streams_lines_triangles`]
    pub fn with_transform_feedback_streams_lines_triangles(mut self, value: bool) -> Self {
        self.transform_feedback_streams_lines_triangles = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_rasterization_stream_select`]
    pub fn with_transform_feedback_rasterization_stream_select(mut self, value: bool) -> Self {
        self.transform_feedback_rasterization_stream_select = value as u8 as u32;
        self
    }
    ///Sets the value of [`Self::transform_feedback_draw`]
    pub fn with_transform_feedback_draw(mut self, value: bool) -> Self {
        self.transform_feedback_draw = value as u8 as u32;
        self
    }
}
///[VkPipelineRasterizationStateStreamCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationStateStreamCreateInfoEXT.html) - Structure defining the geometry stream used for rasterization
///# C Specifications
///The vertex stream used for rasterization is specified by adding a
///[`PipelineRasterizationStateStreamCreateInfoEXT`] structure to the
///[`p_next`] chain of a [`PipelineRasterizationStateCreateInfo`]
///structure.The [`PipelineRasterizationStateStreamCreateInfoEXT`] structure is
///defined as:
///```c
///// Provided by VK_EXT_transform_feedback
///typedef struct VkPipelineRasterizationStateStreamCreateInfoEXT {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    VkPipelineRasterizationStateStreamCreateFlagsEXT    flags;
///    uint32_t                                            rasterizationStream;
///} VkPipelineRasterizationStateStreamCreateInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`flags`] is reserved for future use.
/// - [`rasterization_stream`] is the vertex stream selected for rasterization.
///# Description
///If this structure is not present, [`rasterization_stream`] is assumed to be
///zero.
///## Valid Usage
/// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::geometry_streams`] **must**  be enabled
/// - [`rasterization_stream`] **must**  be less than
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
/// - [`rasterization_stream`] **must**  be zero if
///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_rasterization_stream_select`]
///   is [`FALSE`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT`
/// - [`flags`] **must**  be `0`
///# Related
/// - [`ext_transform_feedback`]
/// - [`PipelineRasterizationStateStreamCreateFlagsEXT`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPipelineRasterizationStateStreamCreateInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`flags`] is reserved for future use.
    pub flags: PipelineRasterizationStateStreamCreateFlagsEXT,
    ///[`rasterization_stream`] is the vertex stream selected for
    ///rasterization.
    pub rasterization_stream: u32,
}
impl<'lt> Default for PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::PIPELINE_RASTERIZATION_STATE_STREAM_CREATE_INFO_EXT,
            p_next: std::ptr::null(),
            flags: Default::default(),
            rasterization_stream: 0,
        }
    }
}
impl<'lt> PipelineRasterizationStateStreamCreateInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn with_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
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
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> PipelineRasterizationStateStreamCreateFlagsEXT {
        self.flags
    }
    ///Gets the value of [`Self::rasterization_stream`]
    pub fn rasterization_stream(&self) -> u32 {
        self.rasterization_stream
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut PipelineRasterizationStateStreamCreateFlagsEXT {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::rasterization_stream`]
    pub fn rasterization_stream_mut(&mut self) -> &mut u32 {
        &mut self.rasterization_stream
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateFlagsEXT,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rasterization_stream`]
    pub fn set_rasterization_stream(&mut self, value: u32) -> &mut Self {
        self.rasterization_stream = value;
        self
    }
    ///Sets the value of [`Self::s_type`]
    pub fn with_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn with_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::flags`]
    pub fn with_flags(
        mut self,
        value: crate::extensions::ext_transform_feedback::PipelineRasterizationStateStreamCreateFlagsEXT,
    ) -> Self {
        self.flags = value;
        self
    }
    ///Sets the value of [`Self::rasterization_stream`]
    pub fn with_rasterization_stream(mut self, value: u32) -> Self {
        self.rasterization_stream = value;
        self
    }
}
impl CommandBuffer {
    ///[vkCmdBindTransformFeedbackBuffersEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) - Bind transform feedback buffers to a command buffer
    ///# C Specifications
    ///To bind transform feedback buffers to a command buffer for use in subsequent
    ///drawing commands, call:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdBindTransformFeedbackBuffersEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    firstBinding,
    ///    uint32_t                                    bindingCount,
    ///    const VkBuffer*                             pBuffers,
    ///    const VkDeviceSize*                         pOffsets,
    ///    const VkDeviceSize*                         pSizes);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`first_binding`] is the index of the first transform feedback binding whose state is
    ///   updated by the command.
    /// - [`binding_count`] is the number of transform feedback bindings whose state is updated by
    ///   the command.
    /// - [`p_buffers`] is a pointer to an array of buffer handles.
    /// - [`p_offsets`] is a pointer to an array of buffer offsets.
    /// - [`p_sizes`] is `NULL` or a pointer to an array of [`DeviceSize`] buffer sizes, specifying
    ///   the maximum number of bytes to capture to the corresponding transform feedback buffer. If
    ///   [`p_sizes`] is `NULL`, or the value of the [`p_sizes`] array element is [`WHOLE_SIZE`],
    ///   then the maximum number of bytes captured will be the size of the corresponding buffer
    ///   minus the buffer offset.
    ///# Description
    ///The values taken from elements i of [`p_buffers`], [`p_offsets`] and
    ///[`p_sizes`] replace the current state for the transform feedback binding
    ///[`first_binding`] +  i, for i in [0,
    ///[`binding_count`]).
    ///The transform feedback binding is updated to start at the offset indicated
    ///by [`p_offsets`][i] from the start of the buffer [`p_buffers`][i].
    ///## Valid Usage
    /// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
    /// - [`first_binding`] **must**  be less than
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - The sum of [`first_binding`] and [`binding_count`] **must**  be less than or equal to
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - All elements of [`p_offsets`] **must**  be less than the size of the corresponding element
    ///   in [`p_buffers`]
    /// - All elements of [`p_offsets`] **must**  be a multiple of 4
    /// - All elements of [`p_buffers`] **must**  have been created with the
    ///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT` flag
    /// - If the optional `pSize` array is specified, each element of [`p_sizes`] **must**  either
    ///   be [`WHOLE_SIZE`], or be less than or equal to
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffer_size`]
    /// - All elements of [`p_sizes`] **must**  be either [`WHOLE_SIZE`], or less than or equal to
    ///   the size of the corresponding buffer in [`p_buffers`]
    /// - All elements of [`p_offsets`] plus [`p_sizes`], where the [`p_sizes`], element is not
    ///   [`WHOLE_SIZE`],  **must**  be less than or equal to the size of the corresponding buffer
    ///   in [`p_buffers`]
    /// - Each element of [`p_buffers`] that is non-sparse  **must**  be bound completely and
    ///   contiguously to a single [`DeviceMemory`] object
    /// - Transform feedback  **must**  not be active when the
    ///   [`cmd_bind_transform_feedback_buffers_ext`] command is recorded
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`p_buffers`] **must**  be a valid pointer to an array of [`binding_count`] valid
    ///   [`Buffer`] handles
    /// - [`p_offsets`] **must**  be a valid pointer to an array of [`binding_count`][`DeviceSize`]
    ///   values
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - [`binding_count`] **must**  be greater than `0`
    /// - Both of [`command_buffer`], and the elements of [`p_buffers`] **must**  have been created,
    ///   allocated, or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBindTransformFeedbackBuffersEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_bind_transform_feedback_buffers_ext(
        self: &Unique<CommandBuffer>,
        first_binding: Option<u32>,
        p_buffers: &[crate::vulkan1_0::Buffer],
        p_offsets: &[crate::vulkan1_0::DeviceSize],
        p_sizes: Option<&[crate::vulkan1_0::DeviceSize]>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_bind_transform_feedback_buffers_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_bind_transform_feedback_buffers_ext())
            .unwrap_unchecked();
        let binding_count = (|len: usize| len)(p_buffers.len()) as _;
        let _return = _function(
            self.as_raw(),
            first_binding.unwrap_or_default() as _,
            binding_count,
            p_buffers.as_ptr(),
            p_offsets.as_ptr(),
            p_sizes.map(|slice| slice.as_ptr()).unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdBeginTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) - Make transform feedback active in the command buffer
    ///# C Specifications
    ///Transform feedback for specific transform feedback buffers is made active by
    ///calling:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdBeginTransformFeedbackEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    firstCounterBuffer,
    ///    uint32_t                                    counterBufferCount,
    ///    const VkBuffer*                             pCounterBuffers,
    ///    const VkDeviceSize*                         pCounterBufferOffsets);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`first_counter_buffer`] is the index of the first transform feedback buffer corresponding
    ///   to [`p_counter_buffers`][0] and [`p_counter_buffer_offsets`][0].
    /// - [`counter_buffer_count`] is the size of the [`p_counter_buffers`] and
    ///   [`p_counter_buffer_offsets`] arrays.
    /// - [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter
    ///   buffers. Each buffer contains a 4 byte integer value representing the byte offset from the
    ///   start of the corresponding transform feedback buffer from where to start capturing vertex
    ///   data. If the byte offset stored to the counter buffer location was done using
    ///   [`cmd_end_transform_feedback_ext`] it can be used to resume transform feedback from the
    ///   previous location. If [`p_counter_buffers`] is `NULL`, then transform feedback will start
    ///   capturing vertex data to byte offset zero in all bound transform feedback buffers. For
    ///   each element of [`p_counter_buffers`] that is [`crate::Handle::null`], transform feedback
    ///   will start capturing vertex data to byte zero in the corresponding bound transform
    ///   feedback buffer.
    /// - [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values
    ///   specifying offsets within each of the [`p_counter_buffers`] where the counter values were
    ///   previously written. The location in each counter buffer at these offsets  **must**  be
    ///   large enough to contain 4 bytes of data. This data is the number of bytes captured by the
    ///   previous transform feedback to this buffer. If [`p_counter_buffer_offsets`] is `NULL`,
    ///   then it is assumed the offsets are zero.
    ///# Description
    ///The active transform feedback buffers will capture primitives emitted from
    ///the corresponding `XfbBuffer` in the bound graphics pipeline.
    ///Any `XfbBuffer` emitted that does not output to an active transform
    ///feedback buffer will not be captured.
    ///## Valid Usage
    /// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
    /// - Transform feedback  **must**  not be active
    /// - [`first_counter_buffer`] **must**  be less than
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or
    ///   equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`,
    ///   [`p_counter_buffers`] **must**  be a valid pointer to an array of
    ///   [`counter_buffer_count`][`Buffer`] handles that are either valid or
    ///   [`crate::Handle::null`]
    /// - For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**
    ///   reference a buffer large enough to hold 4 bytes at the corresponding offset from the
    ///   [`p_counter_buffer_offsets`] array
    /// - If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
    /// - For each buffer handle in the [`p_counter_buffers`] array that is not
    ///   [`crate::Handle::null`] it  **must**  have been created with a `usage` value containing
    ///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
    /// - A valid graphics pipeline  **must**  be bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization)
    ///   of the bound graphics pipeline  **must**  have been declared with the `Xfb` execution mode
    /// - Transform feedback  **must**  not be made active in a render pass instance with multiview
    ///   enabled
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffer_offsets`] is not `NULL`,
    ///   [`p_counter_buffer_offsets`] **must**  be a valid pointer to an array of
    ///   [`counter_buffer_count`][`DeviceSize`] values
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    /// - Both of [`command_buffer`], and the elements of [`p_counter_buffers`] that are valid
    ///   handles of non-ignored parameters  **must**  have been created, allocated, or retrieved
    ///   from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBeginTransformFeedbackEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_begin_transform_feedback_ext(
        self: &Unique<CommandBuffer>,
        first_counter_buffer: Option<u32>,
        p_counter_buffers: &[crate::vulkan1_0::Buffer],
        p_counter_buffer_offsets: Option<&[crate::vulkan1_0::DeviceSize]>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_begin_transform_feedback_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_begin_transform_feedback_ext())
            .unwrap_unchecked();
        let counter_buffer_count = (|len: usize| len)(p_counter_buffers.len()) as _;
        let _return = _function(
            self.as_raw(),
            first_counter_buffer.unwrap_or_default() as _,
            counter_buffer_count,
            p_counter_buffers.as_ptr(),
            p_counter_buffer_offsets
                .map(|slice| slice.as_ptr())
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdEndTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) - Make transform feedback inactive in the command buffer
    ///# C Specifications
    ///Transform feedback for specific transform feedback buffers is made inactive
    ///by calling:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdEndTransformFeedbackEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    firstCounterBuffer,
    ///    uint32_t                                    counterBufferCount,
    ///    const VkBuffer*                             pCounterBuffers,
    ///    const VkDeviceSize*                         pCounterBufferOffsets);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`first_counter_buffer`] is the index of the first transform feedback buffer corresponding
    ///   to [`p_counter_buffers`][0] and [`p_counter_buffer_offsets`][0].
    /// - [`counter_buffer_count`] is the size of the [`p_counter_buffers`] and
    ///   [`p_counter_buffer_offsets`] arrays.
    /// - [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter
    ///   buffers. The counter buffers are used to record the current byte positions of each
    ///   transform feedback buffer where the next vertex output data would be captured. This
    ///   **can**  be used by a subsequent [`cmd_begin_transform_feedback_ext`] call to resume
    ///   transform feedback capture from this position. It can also be used by
    ///   [`cmd_draw_indirect_byte_count_ext`] to determine the vertex count of the draw call.
    /// - [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values
    ///   specifying offsets within each of the [`p_counter_buffers`] where the counter values can
    ///   be written. The location in each counter buffer at these offsets  **must**  be large
    ///   enough to contain 4 bytes of data. The data stored at this location is the byte offset
    ///   from the start of the transform feedback buffer binding where the next vertex data would
    ///   be written. If [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets are
    ///   zero.
    ///# Description
    ///## Valid Usage
    /// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
    /// - Transform feedback  **must**  be active
    /// - [`first_counter_buffer`] **must**  be less than
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or
    ///   equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
    /// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`,
    ///   [`p_counter_buffers`] **must**  be a valid pointer to an array of
    ///   [`counter_buffer_count`][`Buffer`] handles that are either valid or
    ///   [`crate::Handle::null`]
    /// - For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**
    ///   reference a buffer large enough to hold 4 bytes at the corresponding offset from the
    ///   [`p_counter_buffer_offsets`] array
    /// - If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
    /// - For each buffer handle in the [`p_counter_buffers`] array that is not
    ///   [`crate::Handle::null`] it  **must**  have been created with a `usage` value containing
    ///   `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - If [`counter_buffer_count`] is not `0`, and [`p_counter_buffer_offsets`] is not `NULL`,
    ///   [`p_counter_buffer_offsets`] **must**  be a valid pointer to an array of
    ///   [`counter_buffer_count`][`DeviceSize`] values
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    /// - Both of [`command_buffer`], and the elements of [`p_counter_buffers`] that are valid
    ///   handles of non-ignored parameters  **must**  have been created, allocated, or retrieved
    ///   from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdEndTransformFeedbackEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_end_transform_feedback_ext(
        self: &Unique<CommandBuffer>,
        first_counter_buffer: Option<u32>,
        p_counter_buffers: &[crate::vulkan1_0::Buffer],
        p_counter_buffer_offsets: Option<&[crate::vulkan1_0::DeviceSize]>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_end_transform_feedback_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_end_transform_feedback_ext())
            .unwrap_unchecked();
        let counter_buffer_count = (|len: usize| len)(p_counter_buffers.len()) as _;
        let _return = _function(
            self.as_raw(),
            first_counter_buffer.unwrap_or_default() as _,
            counter_buffer_count,
            p_counter_buffers.as_ptr(),
            p_counter_buffer_offsets
                .map(|slice| slice.as_ptr())
                .unwrap_or_else(std::ptr::null),
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdBeginQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) - Begin an indexed query
    ///# C Specifications
    ///To begin an indexed query, call:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdBeginQueryIndexedEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkQueryPool                                 queryPool,
    ///    uint32_t                                    query,
    ///    VkQueryControlFlags                         flags,
    ///    uint32_t                                    index);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which this command will be recorded.
    /// - [`query_pool`] is the query pool that will manage the results of the query.
    /// - [`query`] is the query index within the query pool that will contain the results.
    /// - [`flags`] is a bitmask of [`QueryControlFlagBits`] specifying constraints on the types of
    ///   queries that  **can**  be performed.
    /// - [`index`] is the query type specific index. When the query type is
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the index represents the vertex stream.
    ///# Description
    ///The [`cmd_begin_query_indexed_ext`] command operates the same as the
    ///[`cmd_begin_query`] command, except that it also accepts a query type
    ///specific [`index`] parameter.
    ///## Valid Usage
    /// - All queries used by the command  **must**  be unavailable
    /// - The `queryType` used to create [`query_pool`] **must**  not be `VK_QUERY_TYPE_TIMESTAMP`
    /// - The `queryType` used to create [`query_pool`] **must**  not be
    ///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR` or
    ///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
    /// - The `queryType` used to create [`query_pool`] **must**  not be
    ///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
    /// - If the [precise occlusion queries]() feature is not enabled, or the `queryType` used to
    ///   create [`query_pool`] was not `VK_QUERY_TYPE_OCCLUSION`, [`flags`] **must**  not contain
    ///   `VK_QUERY_CONTROL_PRECISE_BIT`
    /// - [`query`] **must**  be less than the number of queries in [`query_pool`]
    /// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_OCCLUSION`, the
    ///   [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS`
    ///   and any of the `pipelineStatistics` indicate graphics operations, the [`CommandPool`] that
    ///   [`command_buffer`] was allocated from  **must**  support graphics operations
    /// - If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS`
    ///   and any of the `pipelineStatistics` indicate compute operations, the [`CommandPool`] that
    ///   [`command_buffer`] was allocated from  **must**  support compute operations
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - If called within a render pass instance, the sum of [`query`] and the number of bits set
    ///   in the current subpass’s view mask  **must**  be less than or equal to the number of
    ///   queries in [`query_pool`]
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR` the [`CommandPool`] that
    ///   [`command_buffer`] was allocated from  **must**  support [video encode operations]()
    /// - If the [`query_pool`] was created with the same `queryType` as that of another [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
    ///   query within [`command_buffer`], then [`index`] **must**  not match the index used for the
    ///   active query
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`CommandPool`] that [`command_buffer`]
    ///   was allocated from  **must**  support graphics operations
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less
    ///   than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
    /// - If the `queryType` used to create [`query_pool`] was not
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` then
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_queries`] **must**  be
    ///   supported
    ///
    /// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`,
    ///   the [profiling lock]() **must**  have been held before [`begin_command_buffer`] was called
    ///   on [`command_buffer`]
    /// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`
    ///   and one of the counters used to create [`query_pool`] was
    ///   `VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR`, the query begin  **must**  be the first
    ///   recorded command in [`command_buffer`]
    /// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`
    ///   and one of the counters used to create [`query_pool`] was
    ///   `VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR`, the begin command  **must**  not be
    ///   recorded within a render pass instance
    /// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`
    ///   and another query pool with a `queryType``VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` has been
    ///   used within [`command_buffer`], its parent primary command buffer or secondary command
    ///   buffer recorded within the same parent primary command buffer as [`command_buffer`], the
    ///   [`performanceCounterMultipleQueryPools`]() feature  **must**  be enabled
    /// - If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`,
    ///   this command  **must**  not be recorded in a command buffer that, either directly or
    ///   through secondary command buffers, also contains a [`cmd_reset_query_pool`] command
    ///   affecting the same query
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
    /// - [`flags`] **must**  be a valid combination of [`QueryControlFlagBits`] values
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`CommandBuffer`]
    /// - [`QueryControlFlags`]
    /// - [`QueryPool`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdBeginQueryIndexedEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_begin_query_indexed_ext(
        self: &Unique<CommandBuffer>,
        query_pool: QueryPool,
        query: Option<u32>,
        flags: QueryControlFlags,
        index: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_begin_query_indexed_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_begin_query_indexed_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            query_pool,
            query.unwrap_or_default() as _,
            flags,
            index.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdEndQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html) - Ends a query
    ///# C Specifications
    ///To end an indexed query after the set of desired drawing or dispatching
    ///commands is recorded, call:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdEndQueryIndexedEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    VkQueryPool                                 queryPool,
    ///    uint32_t                                    query,
    ///    uint32_t                                    index);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which this command will be recorded.
    /// - [`query_pool`] is the query pool that is managing the results of the query.
    /// - [`query`] is the query index within the query pool where the result is stored.
    /// - [`index`] is the query type specific index.
    ///# Description
    ///The [`cmd_end_query_indexed_ext`] command operates the same as the
    ///[`cmd_end_query`] command, except that it also accepts a query type
    ///specific [`index`] parameter.
    ///## Valid Usage
    /// - All queries used by the command  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
    /// - [`query`] **must**  be less than the number of queries in [`query_pool`]
    /// - [`command_buffer`] **must**  not be a protected command buffer
    /// - If [`cmd_end_query_indexed_ext`] is called within a render pass instance, the sum of
    ///   [`query`] and the number of bits set in the current subpass’s view mask  **must**  be less
    ///   than or equal to the number of queries in [`query_pool`]
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less
    ///   than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
    /// - If the `queryType` used to create [`query_pool`] was not
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
    /// - If the `queryType` used to create [`query_pool`] was
    ///   `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`[`index`] **must**  equal the [`index`] used
    ///   to begin the query
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`query_pool`] **must**  be a valid [`QueryPool`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support
    ///   graphics, or compute operations
    /// - Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or
    ///   retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`CommandBuffer`]
    /// - [`QueryPool`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdEndQueryIndexedEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_end_query_indexed_ext(
        self: &Unique<CommandBuffer>,
        query_pool: QueryPool,
        query: Option<u32>,
        index: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_end_query_indexed_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_end_query_indexed_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            query_pool,
            query.unwrap_or_default() as _,
            index.unwrap_or_default() as _,
        );
        ()
    }
}
impl CommandBuffer {
    ///[vkCmdDrawIndirectByteCountEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDrawIndirectByteCountEXT.html) - Draw primitives with indirect parameters where the vertex count is derived from the counter byte value in the counter buffer
    ///# C Specifications
    ///To record a non-indexed draw call, where the vertex count is based on a byte
    ///count read from a buffer and the passed in vertex stride parameter, call:
    ///```c
    ///// Provided by VK_EXT_transform_feedback
    ///void vkCmdDrawIndirectByteCountEXT(
    ///    VkCommandBuffer                             commandBuffer,
    ///    uint32_t                                    instanceCount,
    ///    uint32_t                                    firstInstance,
    ///    VkBuffer                                    counterBuffer,
    ///    VkDeviceSize                                counterBufferOffset,
    ///    uint32_t                                    counterOffset,
    ///    uint32_t                                    vertexStride);
    ///```
    ///# Parameters
    /// - [`command_buffer`] is the command buffer into which the command is recorded.
    /// - [`instance_count`] is the number of instances to draw.
    /// - [`first_instance`] is the instance ID of the first instance to draw.
    /// - [`counter_buffer`] is the buffer handle from where the byte count is read.
    /// - [`counter_buffer_offset`] is the offset into the buffer used to read the byte count, which
    ///   is used to calculate the vertex count for this draw call.
    /// - [`counter_offset`] is subtracted from the byte count read from the [`counter_buffer`] at
    ///   the [`counter_buffer_offset`]
    /// - [`vertex_stride`] is the stride in bytes between each element of the vertex data that is
    ///   used to calculate the vertex count from the counter value. This value is typically the
    ///   same value that was used in the graphics pipeline state when the transform feedback was
    ///   captured as the `XfbStride`.
    ///# Description
    ///When the command is executed, primitives are assembled in the same way as
    ///done with [`cmd_draw`] except the `vertexCount` is calculated based
    ///on the byte count read from [`counter_buffer`] at offset
    ///[`counter_buffer_offset`].
    ///The assembled primitives execute the bound graphics pipeline.The effective `vertexCount` is
    /// calculated as follows:
    ///```c
    ///const uint32_t * counterBufferPtr = (const uint8_t *)counterBuffer.address +
    /// counterBufferOffset;
    ///vertexCount = floor(max(0, (*counterBufferPtr - counterOffset)) / vertexStride);
    ///```
    ///The effective `firstVertex` is zero.
    ///## Valid Usage
    /// - If a [`Sampler`] created with `magFilter` or `minFilter` equal to `VK_FILTER_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`Sampler`] created with `mipmapMode` equal to `VK_SAMPLER_MIPMAP_MODE_LINEAR` and
    ///   `compareEnable` equal to [`FALSE`] is used to sample a [`ImageView`] as a result of this
    ///   command, then the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`
    /// - If a [`ImageView`] is sampled with [depth comparison](), the image view’s [format
    ///   features]() **must**  contain `VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT`
    /// - If a [`ImageView`] is accessed using atomic operations as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT`
    /// - If a [`ImageView`] is sampled with `VK_FILTER_CUBIC_EXT` as a result of this command, then
    ///   the image view’s [format features]() **must**  contain
    ///   `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_CUBIC_BIT_EXT`
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` as a result of this command
    ///   **must**  have a [`ImageViewType`] and format that supports cubic filtering, as specified
    ///   by [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`ImageView`] being sampled with `VK_FILTER_CUBIC_EXT` with a reduction mode of either
    ///   `VK_SAMPLER_REDUCTION_MODE_MIN` or `VK_SAMPLER_REDUCTION_MODE_MAX` as a result of this
    ///   command  **must**  have a [`ImageViewType`] and format that supports cubic filtering
    ///   together with minmax filtering, as specified by
    ///   [`FilterCubicImageViewImageFormatPropertiesEXT::filter_cubic_minmax`] returned by
    ///   [`get_physical_device_image_format_properties2`]
    /// - Any [`Image`] created with a [`ImageCreateInfo::flags`] containing
    ///   `VK_IMAGE_CREATE_CORNER_SAMPLED_BIT_NV` sampled as a result of this command  **must**
    ///   only be sampled using a [`SamplerAddressMode`] of `VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE`
    /// - Any [`ImageView`] or [`BufferView`] being written as a storage image or storage texel
    ///   buffer where the image format field of the `OpTypeImage` is `Unknown` then the view’s
    ///   format feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT`
    /// - Any [`ImageView`] or [`BufferView`] being read as a storage image or storage texel buffer
    ///   where the image format field of the `OpTypeImage` is `Unknown` then the view’s format
    ///   feature  **must**  contain `VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT`
    /// - For each set *n* that is statically used by the [`Pipeline`] bound to the pipeline bind
    ///   point used by this command, a descriptor set  **must**  have been bound to *n* at the same
    ///   pipeline bind point, with a [`PipelineLayout`] that is compatible for set *n*, with the
    ///   [`PipelineLayout`] used to create the current [`Pipeline`], as described in
    ///   [[descriptorsets-compatibility]]()
    /// - If the [`maintenance4`]() feature is not enabled, then for each push constant that is
    ///   statically used by the [`Pipeline`] bound to the pipeline bind point used by this command,
    ///   a push constant value  **must**  have been set for the same pipeline bind point, with a
    ///   [`PipelineLayout`] that is compatible for push constants, with the [`PipelineLayout`] used
    ///   to create the current [`Pipeline`], as described in [[descriptorsets-compatibility]]()
    /// - Descriptors in each bound descriptor set, specified via [`cmd_bind_descriptor_sets`],
    ///   **must**  be valid if they are statically used by the [`Pipeline`] bound to the pipeline
    ///   bind point used by this command
    /// - A valid pipeline  **must**  be bound to the pipeline bind point used by this command
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command requires
    ///   any dynamic state, that state  **must**  have been set or inherited (if the
    ///   `[`nv_inherited_viewport_scissor`]` extension is enabled) for [`command_buffer`], and done
    ///   so after any previously bound pipeline with the corresponding state not specified as
    ///   dynamic
    /// - There  **must**  not have been any calls to dynamic state setting commands for any state
    ///   not specified as dynamic in the [`Pipeline`] object bound to the pipeline bind point used
    ///   by this command, since that pipeline was bound
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used to sample from any [`Image`] with a [`ImageView`] of the type
    ///   `VK_IMAGE_VIEW_TYPE_3D`, `VK_IMAGE_VIEW_TYPE_CUBE`, `VK_IMAGE_VIEW_TYPE_1D_ARRAY`,
    ///   `VK_IMAGE_VIEW_TYPE_2D_ARRAY` or `VK_IMAGE_VIEW_TYPE_CUBE_ARRAY`, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions with
    ///   `ImplicitLod`, `Dref` or `Proj` in their name, in any shader stage
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] object that uses unnormalized coordinates, that sampler  **must**  not be
    ///   used with any of the SPIR-V `OpImageSample*` or `OpImageSparseSample*` instructions that
    ///   includes a LOD bias or any offset values, in any shader stage
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a uniform buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If the [robust buffer access]() feature is not enabled, and if the [`Pipeline`] object
    ///   bound to the pipeline bind point used by this command accesses a storage buffer, it
    ///   **must**  not access values outside of the range of the buffer as specified in the
    ///   descriptor set bound to the same pipeline bind point
    /// - If [`command_buffer`] is an unprotected command buffer and [`protectedNoFault`]() is not
    ///   supported, any resource accessed by the [`Pipeline`] object bound to the pipeline bind
    ///   point used by this command  **must**  not be a protected resource
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  only be used with `OpImageSample*` or
    ///   `OpImageSparseSample*` instructions
    /// - If the [`Pipeline`] object bound to the pipeline bind point used by this command accesses
    ///   a [`Sampler`] or [`ImageView`] object that enables [sampler Y′C<sub>B</sub>C<sub>R</sub>
    ///   conversion](), that object  **must**  not use the `ConstOffset` and `Offset` operands
    /// - If a [`ImageView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the image view’s format
    /// - If a [`BufferView`] is accessed using `OpImageWrite` as a result of this command, then the
    ///   `Type` of the `Texel` operand of that instruction  **must**  have at least as many
    ///   components as the buffer view’s format
    /// - If a [`ImageView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`ImageView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If a [`BufferView`] with a [`Format`] that has a 64-bit component width is accessed as a
    ///   result of this command, the `SampledType` of the `OpTypeImage` operand of that instruction
    ///   **must**  have a `Width` of 64
    /// - If a [`BufferView`] with a [`Format`] that has a component width less than 64-bit is
    ///   accessed as a result of this command, the `SampledType` of the `OpTypeImage` operand of
    ///   that instruction  **must**  have a `Width` of 32
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Image`] objects created
    ///   with the `VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - If the [`sparseImageInt64Atomics`]() feature is not enabled, [`Buffer`] objects created
    ///   with the `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` flag  **must**  not be accessed by atomic
    ///   instructions through an `OpTypeImage` with a `SampledType` with a `Width` of 64 by this
    ///   command
    /// - The current render pass  **must**  be [compatible]() with the `renderPass` member of the
    ///   [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`] bound to
    ///   `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - The subpass index of the current render pass  **must**  be equal to the `subpass` member
    ///   of the [`GraphicsPipelineCreateInfo`] structure specified when creating the [`Pipeline`]
    ///   bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
    /// - Every input attachment used by the current subpass  **must**  be bound to the pipeline via
    ///   a descriptor set
    /// - Memory backing image subresources used as attachments in the current render pass  **must**
    ///   not be written in any way other than as an attachment by this command
    /// - If any recorded command in the current subpass will write to an image subresource as an
    ///   attachment, this command  **must**  not read from the memory backing that image
    ///   subresource in any other way than as an attachment
    /// - If any recorded command in the current subpass will read from an image subresource used as
    ///   an attachment in any way other than as an attachment, this command  **must**  not write to
    ///   that image subresource as an attachment
    /// - If the draw is recorded in a render pass instance with multiview enabled, the maximum
    ///   instance index  **must**  be less than or equal to
    ///   [`PhysicalDeviceMultiviewProperties::max_multiview_instance_index`]
    /// - If the bound graphics pipeline was created with
    ///   [`PipelineSampleLocationsStateCreateInfoEXT::sample_locations_enable`] set to [`TRUE`] and
    ///   the current subpass has a depth/stencil attachment, then that attachment  **must**  have
    ///   been created with the `VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT` bit set
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SAMPLE_LOCATIONS_EXT` dynamic state enabled then
    ///   [`cmd_set_sample_locations_ext`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_viewport_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::scissor_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, then
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`] **must**  match the
    ///   [`PipelineViewportStateCreateInfo::viewport_count`] of the pipeline
    /// - If the bound graphics pipeline state was created with both the
    ///   `VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic
    ///   states enabled then both [`cmd_set_viewport_with_count`] and
    ///   [`cmd_set_scissor_with_count`] **must**  have been called in the current command buffer
    ///   prior to this drawing command, and the `viewportCount` parameter of
    ///   [`cmd_set_viewport_with_count`] **must**  match the `scissorCount` parameter of
    ///   [`cmd_set_scissor_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV` dynamic state enabled, then the bound graphics
    ///   pipeline  **must**  have been created with
    ///   [`PipelineViewportWScalingStateCreateInfoNV::viewport_count`] greater or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and `VK_DYNAMIC_STATE_VIEWPORT_W_SCALING_NV`
    ///   dynamic states enabled then the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_w_scaling_nv`] **must**  be greater than or equal to the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic state enabled, then the bound
    ///   graphics pipeline  **must**  have been created with
    ///   [`PipelineViewportShadingRateImageStateCreateInfoNV::viewport_count`] greater or equal to
    ///   the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` and
    ///   `VK_DYNAMIC_STATE_VIEWPORT_SHADING_RATE_PALETTE_NV` dynamic states enabled then the
    ///   `viewportCount` parameter in the last call to [`cmd_set_viewport_shading_rate_palette_nv`]
    ///   **must**  be greater than or equal to the `viewportCount` parameter in the last call to
    ///   [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportSwizzleStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with [`PipelineViewportSwizzleStateCreateInfoNV::viewport_count`] greater or equal
    ///   to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT` dynamic state enabled and a
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure chained from
    ///   `VkPipelineVieportCreateInfo`, then the bound graphics pipeline  **must**  have been
    ///   created with
    ///   [`PipelineViewportExclusiveScissorStateCreateInfoNV::exclusive_scissor_count`] greater or
    ///   equal to the `viewportCount` parameter in the last call to [`cmd_set_viewport_with_count`]
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE` dynamic state enabled then
    ///   [`cmd_set_rasterizer_discard_enable`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE` dynamic state enabled then
    ///   [`cmd_set_depth_bias_enable`] **must**  have been called in the current command buffer
    ///   prior to this drawing command
    /// - If the bound graphics pipeline state was created with the `VK_DYNAMIC_STATE_LOGIC_OP_EXT`
    ///   dynamic state enabled then [`cmd_set_logic_op_ext`] **must**  have been called in the
    ///   current command buffer prior to this drawing command and the `logicOp` **must**  be a
    ///   valid [`LogicOp`] value
    /// - If the [`primitiveFragmentShadingRateWithMultipleViewports`]() limit is not supported, the
    ///   bound graphics pipeline was created with the `VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT`
    ///   dynamic state enabled, and any of the shader stages of the bound graphics pipeline write
    ///   to the `PrimitiveShadingRateKHR` built-in, then [`cmd_set_viewport_with_count`] **must**
    ///   have been called in the current command buffer prior to this drawing command, and the
    ///   `viewportCount` parameter of [`cmd_set_viewport_with_count`] **must**  be `1`
    /// - If rasterization is not disabled in the bound graphics pipeline, then for each color
    ///   attachment in the subpass, if the corresponding image view’s [format features]() do not
    ///   contain `VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT`, then the `blendEnable` member of
    ///   the corresponding element of the `pAttachments` member of `pColorBlendState` **must**  be
    ///   [`FALSE`]
    /// - If rasterization is not disabled in the bound graphics pipeline, and neither the
    ///   `[`amd_mixed_attachment_samples`]` nor the `[`nv_framebuffer_mixed_samples`]` extensions
    ///   are enabled, then [`PipelineMultisampleStateCreateInfo::rasterization_samples`] **must**
    ///   be the same as the current subpass color and/or depth/stencil attachments
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this
    ///   command  **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not write any
    ///   values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL`, this command  **must**  not
    ///   write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is
    ///   `VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL`, this command  **must**  not
    ///   write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pDepthAttachment` is not [`crate::Handle::null`], and the `layout`
    ///   member of `pDepthAttachment` is `VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL`, this command
    ///   **must**  not write any values to the depth attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the
    ///   `imageView` member of `pStencilAttachment` is not [`crate::Handle::null`], and the
    ///   `layout` member of `pStencilAttachment` is `VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL`,
    ///   this command  **must**  not write any values to the stencil attachment
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::view_mask`] equal to [`RenderingInfo::view_mask`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound graphics pipeline  **must**  have been created with a
    ///   [`PipelineRenderingCreateInfo::color_attachment_count`] equal to
    ///   [`RenderingInfo::color_attachment_count`]
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo::color_attachment_count`] greater than `0`, then each element of the
    ///   [`RenderingInfo::color_attachments`] array with a `imageView` not equal to
    ///   [`crate::Handle::null`] **must**  have been created with a [`Format`] equal to the
    ///   corresponding element of [`PipelineRenderingCreateInfo::color_attachment_formats`] used to
    ///   create the currently bound graphics pipeline
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_COLOR_WRITE_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_color_write_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `attachmentCount` parameter of
    ///   [`cmd_set_color_write_enable_ext`] **must**  be equal to the
    ///   [`PipelineColorBlendStateCreateInfo::attachment_count`] of the currently bound graphics
    ///   pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::depth_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineRenderingCreateInfo::stencil_attachment_format`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the [`Format`] used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentShadingRateAttachmentInfoKHR::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_BIT_KHR`
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`] and
    ///   [`RenderingFragmentDensityMapAttachmentInfoEXT::image_view`] was not
    ///   [`crate::Handle::null`], the currently bound graphics pipeline  **must**  have been
    ///   created with `VK_PIPELINE_CREATE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_BIT_EXT`
    /// - If the currently bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the corresponding element of the `pColorAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created with a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of the `depthStencilAttachmentSamples` member of
    ///   [`AttachmentSampleCountInfoAMD`] or [`AttachmentSampleCountInfoNV`] used to create the
    ///   currently bound graphics pipeline  **must**  be equal to the sample count used to create
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the currently bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and the current render pass instance was begun
    ///   with [`cmd_begin_rendering`] with a [`RenderingInfo::color_attachment_count`] parameter
    ///   greater than `0`, then each element of the [`RenderingInfo::color_attachments`] array with
    ///   a `imageView` not equal to [`crate::Handle::null`] **must**  have been created with a
    ///   sample count equal to the value of
    ///   [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create the currently
    ///   bound graphics pipeline
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pDepthAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pDepthAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline was created without a [`AttachmentSampleCountInfoAMD`] or
    ///   [`AttachmentSampleCountInfoNV`] structure, and
    ///   [`RenderingInfo`]::`pStencilAttachment->pname`:imageView was not [`crate::Handle::null`],
    ///   the value of [`PipelineMultisampleStateCreateInfo::rasterization_samples`] used to create
    ///   the currently bound graphics pipeline  **must**  be equal to the sample count used to
    ///   create [`RenderingInfo`]::`pStencilAttachment->pname`:imageView
    /// - If the current render pass instance was begun with [`cmd_begin_rendering`], the currently
    ///   bound pipeline  **must**  have been created with a
    ///   [`GraphicsPipelineCreateInfo::render_pass`] equal to [`crate::Handle::null`]
    ///
    /// - All vertex input bindings accessed via vertex input variables declared in the vertex
    ///   shader entry point’s interface  **must**  have either valid or [`crate::Handle::null`]
    ///   buffers bound
    /// - If the [nullDescriptor]() feature is not enabled, all vertex input bindings accessed via
    ///   vertex input variables declared in the vertex shader entry point’s interface  **must**
    ///   not be [`crate::Handle::null`]
    /// - For a given vertex buffer binding, any attribute data fetched  **must**  be entirely
    ///   contained within the corresponding vertex buffer binding, as described in
    ///   [[fxvertex-input]]()
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_topology_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command, and the `primitiveTopology` parameter of
    ///   [`cmd_set_primitive_topology_ext`] **must**  be of the same [topology class]() as the
    ///   pipeline [`PipelineInputAssemblyStateCreateInfo::topology`] state
    /// - If the bound graphics pipeline was created with both the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` and `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT`
    ///   dynamic states enabled, then [`cmd_set_vertex_input_ext`] **must**  have been called in
    ///   the current command buffer prior to this draw command
    /// - If the bound graphics pipeline was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE_EXT` dynamic state enabled, but not the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command, and the `pStrides` parameter of
    ///   [`cmd_bind_vertex_buffers2_ext`] **must**  not be `NULL`
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_VERTEX_INPUT_EXT` dynamic state enabled, then
    ///   [`cmd_set_vertex_input_ext`] **must**  have been called in the current command buffer
    ///   prior to this draw command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PATCH_CONTROL_POINTS_EXT` dynamic state enabled then
    ///   [`cmd_set_patch_control_points_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - If the bound graphics pipeline state was created with the
    ///   `VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE_EXT` dynamic state enabled then
    ///   [`cmd_set_primitive_restart_enable_ext`] **must**  have been called in the current command
    ///   buffer prior to this drawing command
    /// - The bound graphics pipeline  **must**  not have been created with the
    ///   [`PipelineShaderStageCreateInfo::stage`] member of an element of
    ///   [`GraphicsPipelineCreateInfo::stages`] set to `VK_SHADER_STAGE_TASK_BIT_NV` or
    ///   `VK_SHADER_STAGE_MESH_BIT_NV`
    /// - [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
    /// - The implementation  **must**  support
    ///   [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_draw`]
    /// - [`vertex_stride`] **must**  be greater than 0 and less than or equal to
    ///   [`PhysicalDeviceLimits`]`::maxTransformFeedbackBufferDataStride`
    /// - If [`counter_buffer`] is non-sparse then it  **must**  be bound completely and
    ///   contiguously to a single [`DeviceMemory`] object
    /// - [`counter_buffer`] **must**  have been created with the
    ///   `VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT` bit set
    /// - [`counter_buffer_offset`] **must**  be a multiple of `4`
    /// - [`command_buffer`] **must**  not be a protected command buffer
    ///
    ///## Valid Usage (Implicit)
    /// - [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
    /// - [`counter_buffer`] **must**  be a valid [`Buffer`] handle
    /// - [`command_buffer`] **must**  be in the [recording state]()
    /// - The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics
    ///   operations
    /// - This command  **must**  only be called inside of a render pass instance
    /// - Both of [`command_buffer`], and [`counter_buffer`] **must**  have been created, allocated,
    ///   or retrieved from the same [`Device`]
    ///
    ///## Host Synchronization
    /// - Host access to [`command_buffer`] **must**  be externally synchronized
    /// - Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**
    ///   be externally synchronized
    ///
    ///## Command Properties
    ///# Related
    /// - [`ext_transform_feedback`]
    /// - [`Buffer`]
    /// - [`CommandBuffer`]
    /// - [`DeviceSize`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkCmdDrawIndirectByteCountEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn cmd_draw_indirect_byte_count_ext(
        self: &Unique<CommandBuffer>,
        instance_count: Option<u32>,
        first_instance: Option<u32>,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: Option<u32>,
        vertex_stride: Option<u32>,
    ) -> () {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_draw_indirect_byte_count_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .device()
            .vtable()
            .ext_transform_feedback()
            .and_then(|vtable| vtable.cmd_draw_indirect_byte_count_ext())
            .unwrap_unchecked();
        let _return = _function(
            self.as_raw(),
            instance_count.unwrap_or_default() as _,
            first_instance.unwrap_or_default() as _,
            counter_buffer,
            counter_buffer_offset,
            counter_offset.unwrap_or_default() as _,
            vertex_stride.unwrap_or_default() as _,
        );
        ()
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_transform_feedback`
pub struct DeviceExtTransformFeedbackVTable {
    ///See [`FNCmdBindTransformFeedbackBuffersExt`] for more information.
    pub cmd_bind_transform_feedback_buffers_ext: FNCmdBindTransformFeedbackBuffersExt,
    ///See [`FNCmdBeginTransformFeedbackExt`] for more information.
    pub cmd_begin_transform_feedback_ext: FNCmdBeginTransformFeedbackExt,
    ///See [`FNCmdEndTransformFeedbackExt`] for more information.
    pub cmd_end_transform_feedback_ext: FNCmdEndTransformFeedbackExt,
    ///See [`FNCmdBeginQueryIndexedExt`] for more information.
    pub cmd_begin_query_indexed_ext: FNCmdBeginQueryIndexedExt,
    ///See [`FNCmdEndQueryIndexedExt`] for more information.
    pub cmd_end_query_indexed_ext: FNCmdEndQueryIndexedExt,
    ///See [`FNCmdDrawIndirectByteCountExt`] for more information.
    pub cmd_draw_indirect_byte_count_ext: FNCmdDrawIndirectByteCountExt,
}
impl DeviceExtTransformFeedbackVTable {
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
            cmd_bind_transform_feedback_buffers_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBindTransformFeedbackBuffersEXT").as_ptr(),
                ))
            },
            cmd_begin_transform_feedback_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdBeginTransformFeedbackEXT").as_ptr(),
                ))
            },
            cmd_end_transform_feedback_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndTransformFeedbackEXT").as_ptr()))
            },
            cmd_begin_query_indexed_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdBeginQueryIndexedEXT").as_ptr()))
            },
            cmd_end_query_indexed_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkCmdEndQueryIndexedEXT").as_ptr()))
            },
            cmd_draw_indirect_byte_count_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkCmdDrawIndirectByteCountEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::cmd_bind_transform_feedback_buffers_ext`]. See
    /// [`FNCmdBindTransformFeedbackBuffersExt`] for more information.
    pub fn cmd_bind_transform_feedback_buffers_ext(&self) -> FNCmdBindTransformFeedbackBuffersExt {
        self.cmd_bind_transform_feedback_buffers_ext
    }
    ///Gets [`Self::cmd_begin_transform_feedback_ext`]. See [`FNCmdBeginTransformFeedbackExt`] for
    /// more information.
    pub fn cmd_begin_transform_feedback_ext(&self) -> FNCmdBeginTransformFeedbackExt {
        self.cmd_begin_transform_feedback_ext
    }
    ///Gets [`Self::cmd_end_transform_feedback_ext`]. See [`FNCmdEndTransformFeedbackExt`] for more
    /// information.
    pub fn cmd_end_transform_feedback_ext(&self) -> FNCmdEndTransformFeedbackExt {
        self.cmd_end_transform_feedback_ext
    }
    ///Gets [`Self::cmd_begin_query_indexed_ext`]. See [`FNCmdBeginQueryIndexedExt`] for more
    /// information.
    pub fn cmd_begin_query_indexed_ext(&self) -> FNCmdBeginQueryIndexedExt {
        self.cmd_begin_query_indexed_ext
    }
    ///Gets [`Self::cmd_end_query_indexed_ext`]. See [`FNCmdEndQueryIndexedExt`] for more
    /// information.
    pub fn cmd_end_query_indexed_ext(&self) -> FNCmdEndQueryIndexedExt {
        self.cmd_end_query_indexed_ext
    }
    ///Gets [`Self::cmd_draw_indirect_byte_count_ext`]. See [`FNCmdDrawIndirectByteCountExt`] for
    /// more information.
    pub fn cmd_draw_indirect_byte_count_ext(&self) -> FNCmdDrawIndirectByteCountExt {
        self.cmd_draw_indirect_byte_count_ext
    }
}
