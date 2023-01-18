[vkCmdEndTransformFeedbackEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndTransformFeedbackEXT.html) - Make transform feedback inactive in the command buffer

# C Specifications
Transform feedback for specific transform feedback buffers is made inactive
by calling:
```c
// Provided by VK_EXT_transform_feedback
void vkCmdEndTransformFeedbackEXT(
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
- [`p_counter_buffers`] is `NULL` or a pointer to an array of [`Buffer`] handles to counter buffers. The counter buffers are used to record the current byte positions of each transform feedback buffer where the next vertex output data would be captured. This  **can**  be used by a subsequent [`cmd_begin_transform_feedback_ext`] call to resume transform feedback capture from this position. It can also be used by [`cmd_draw_indirect_byte_count_ext`] to determine the vertex count of the draw call.
- [`p_counter_buffer_offsets`] is `NULL` or a pointer to an array of [`DeviceSize`] values specifying offsets within each of the [`p_counter_buffers`] where the counter values can be written. The location in each counter buffer at these offsets  **must**  be large enough to contain 4 bytes of data. The data stored at this location is the byte offset from the start of the transform feedback buffer binding where the next vertex data would be written. If [`p_counter_buffer_offsets`] is `NULL`, then it is assumed the offsets are zero.

# Description
## Valid Usage
-  [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
-    Transform feedback  **must**  be active
-  [`first_counter_buffer`] **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    The sum of [`first_counter_buffer`] and [`counter_buffer_count`] **must**  be less than or equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    If [`counter_buffer_count`] is not `0`, and [`p_counter_buffers`] is not `NULL`, [`p_counter_buffers`] **must**  be a valid pointer to an array of [`counter_buffer_count`][`Buffer`] handles that are either valid or [`crate::Handle::null`]
-    For each buffer handle in the array, if it is not [`crate::Handle::null`] it  **must**  reference a buffer large enough to hold 4 bytes at the corresponding offset from the [`p_counter_buffer_offsets`] array
-    If `pCounterBuffer` is `NULL`, then [`p_counter_buffer_offsets`] **must**  also be `NULL`
-    For each buffer handle in the [`p_counter_buffers`] array that is not [`crate::Handle::null`] it  **must**  have been created with a `usage` value containing `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_COUNTER_BUFFER_BIT_EXT`

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
        