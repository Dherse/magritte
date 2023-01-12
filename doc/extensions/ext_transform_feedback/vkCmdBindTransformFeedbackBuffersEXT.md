[vkCmdBindTransformFeedbackBuffersEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBindTransformFeedbackBuffersEXT.html) - Bind transform feedback buffers to a command buffer

# C Specifications
To bind transform feedback buffers to a command buffer for use in subsequent
drawing commands, call:
```c
// Provided by VK_EXT_transform_feedback
void vkCmdBindTransformFeedbackBuffersEXT(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    firstBinding,
    uint32_t                                    bindingCount,
    const VkBuffer*                             pBuffers,
    const VkDeviceSize*                         pOffsets,
    const VkDeviceSize*                         pSizes);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`first_binding`] is the index of the first transform feedback binding whose state is updated by the command.
- [`binding_count`] is the number of transform feedback bindings whose state is updated by the command.
- [`p_buffers`] is a pointer to an array of buffer handles.
- [`p_offsets`] is a pointer to an array of buffer offsets.
- [`p_sizes`] is `NULL` or a pointer to an array of [`DeviceSize`] buffer sizes, specifying the maximum number of bytes to capture to the corresponding transform feedback buffer. If [`p_sizes`] is `NULL`, or the value of the [`p_sizes`] array element is `VK_WHOLE_SIZE`, then the maximum number of bytes captured will be the size of the corresponding buffer minus the buffer offset.

# Description
The values taken from elements i of [`p_buffers`], [`p_offsets`] and
[`p_sizes`] replace the current state for the transform feedback binding
[`first_binding`] +  i, for i in [0,
[`binding_count`]).
The transform feedback binding is updated to start at the offset indicated
by [`p_offsets`][i] from the start of the buffer [`p_buffers`][i].
## Valid Usage
-  [`PhysicalDeviceTransformFeedbackFeaturesEXT::transform_feedback`] **must**  be enabled
-  [`first_binding`] **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    The sum of [`first_binding`] and [`binding_count`] **must**  be less than or equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffers`]
-    All elements of [`p_offsets`] **must**  be less than the size of the corresponding element in [`p_buffers`]
-    All elements of [`p_offsets`] **must**  be a multiple of 4
-    All elements of [`p_buffers`] **must**  have been created with the `VK_BUFFER_USAGE_TRANSFORM_FEEDBACK_BUFFER_BIT_EXT` flag
-    If the optional `pSize` array is specified, each element of [`p_sizes`] **must**  either be `VK_WHOLE_SIZE`, or be less than or equal to [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_buffer_size`]
-    All elements of [`p_sizes`] **must**  be either `VK_WHOLE_SIZE`, or less than or equal to the size of the corresponding buffer in [`p_buffers`]
-    All elements of [`p_offsets`] plus [`p_sizes`], where the [`p_sizes`], element is not `VK_WHOLE_SIZE`,  **must**  be less than or equal to the size of the corresponding buffer in [`p_buffers`]
-    Each element of [`p_buffers`] that is non-sparse  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    Transform feedback  **must**  not be active when the [`cmd_bind_transform_feedback_buffers_ext`] command is recorded

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_buffers`] **must**  be a valid pointer to an array of [`binding_count`] valid [`Buffer`] handles
-  [`p_offsets`] **must**  be a valid pointer to an array of [`binding_count`][`DeviceSize`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-  [`binding_count`] **must**  be greater than `0`
-    Both of [`command_buffer`], and the elements of [`p_buffers`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_transform_feedback`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        