[vkCmdBeginTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginTransformFeedbackEXT.html) - Make transform feedback active in the command buffer

# C Specifications
Transform feedback for specific transform feedback buffers is made active by
calling:
```c
// Provided by VK_EXT_transform_feedback
void vkCmdBeginTransformFeedbackEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstCounterBuffer,
    uint32_t                                    counterBufferCount,
    const VkBuffer*                             pCounterBuffers,
    const VkDeviceSize*                         pCounterBufferOffsets);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`first_counter_buffer`] is the index of the first transform feedback buffer corresponding to [`p_counter_buffers`][0] and [`p_counter_buffer_offsets`][0].
- [`counter_buffer_count`] is the size of the [`p_counter_buffers`] and [`p_counter_buffer_offsets`] arrays.
- [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter buffers. Each buffer contains a 4 byte integer value representing the byte offset from the start of the corresponding transform feedback buffer from where to start capturing vertex data. If the byte offset stored to the counter buffer location was done using [`cmd_end_transform_feedback_ext`] it can be used to resume transform feedback from the previous location. If [`p_counter_buffers`] is `NULL`, then transform feedback will start capturing vertex data to byte offset zero in all bound transform feedback buffers. For each element of [`p_counter_buffers`] that is [`crate::Handle::null`], transform feedback will start capturing vertex data to byte zero in the corresponding bound transform feedback buffer.
- [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values specifying offsets within each of the [`p_counter_buffers`] where the counter values were previously written. The location in each counter buffer at these offsets  **must**  be large enough to contain 4 bytes of data. This data is the number of bytes captured by the previous transform feedback to this buffer. If [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets are zero.

# Description
The active transform feedback buffers will capture primitives emitted from
the corresponding `XfbBuffer` in the bound graphics pipeline.
Any `XfbBuffer` emitted that does not output to an active transform
feedback buffer will not be captured.
## Valid Usage
-  [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
-    Transform feedback  **must**  not be active
-  [`first_counter_buffer`] **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`, [`p_counter_buffers`] **must**  be a valid pointer to an array of [`counter_buffer_count`][`Buffer`] handles that are either valid or [`crate::Handle::null`]
-    For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**  reference a buffer large enough to hold 4 bytes at the corresponding offset from the [`p_counter_buffer_offsets`] array
-    If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
-    For each buffer handle in the [`p_counter_buffers`] array that is not [`crate::Handle::null`] it  **must**  have been created with a `usage` value containing `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`
-    A valid graphics pipeline  **must**  be bound to `VK_PIPELINE_BIND_POINT_GRAPHICS`
-    The last [pre-rasterization shader stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#pipeline-graphics-subsets-pre-rasterization) of the bound graphics pipeline  **must**  have been declared with the `Xfb` execution mode
-    Transform feedback  **must**  not be made active in a render pass instance with multiview enabled

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-    If [`counter_buffer_count`] is not `0`, and [`p_counter_buffer_offsets`] is not `NULL`, [`p_counter_buffer_offsets`] **must**  be a valid pointer to an array of [`counter_buffer_count`][`DeviceSize`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    This command  **must**  only be called inside of a render pass instance
-    Both of [`command_buffer`], and the elements of [`p_counter_buffers`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_transform_feedback`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        