[vkEndCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkEndCommandBuffer.html) - Finish recording a command buffer

# C Specifications
To complete recording of a command buffer, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkEndCommandBuffer(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer to complete recording.

# Description
If there was an error during recording, the application will be notified by
an unsuccessful return code returned by [`end_command_buffer`].
If the application wishes to further use the command buffer, the command
buffer  **must**  be reset.The command buffer  **must**  have been in the [recording state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle), and is moved to the [executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage
-  [`command_buffer`] **must**  be in the [recording state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If [`command_buffer`] is a primary command buffer, there  **must**  not be an active render pass instance
-    All queries made [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active) during the recording of [`command_buffer`] **must**  have been made inactive
-    Conditional rendering  **must**  not be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#active-conditional-rendering)
-    If [`command_buffer`] is a secondary command buffer, there  **must**  not be an outstanding [`cmd_begin_debug_utils_label_ext`] command recorded to [`command_buffer`] that has not previously been ended by a call to [`cmd_end_debug_utils_label_ext`]
-    If [`command_buffer`] is a secondary command buffer, there  **must**  not be an outstanding [`cmd_debug_marker_begin_ext`] command recorded to [`command_buffer`] that has not previously been ended by a call to [`cmd_debug_marker_end_ext`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        