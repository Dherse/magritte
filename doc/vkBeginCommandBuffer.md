[vkBeginCommandBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBeginCommandBuffer.html) - Start recording a command buffer

# C Specifications
To begin recording a command buffer, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkBeginCommandBuffer(
    VkCommandBuffer                             commandBuffer,
    const VkCommandBufferBeginInfo*             pBeginInfo);
```

# Parameters
- [`command_buffer`] is the handle of the command buffer which is to be put in the recording state.
- [`p_begin_info`] is a pointer to a [`CommandBufferBeginInfo`] structure defining additional information about how the command buffer begins recording.

# Description
## Valid Usage
-  [`command_buffer`] **must**  not be in the [recording or pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If [`command_buffer`] was allocated from a [`CommandPool`] which did not have the `VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT` flag set, [`command_buffer`] **must**  be in the [initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-    If [`command_buffer`] is a secondary command buffer, the `pInheritanceInfo` member of [`p_begin_info`] **must**  be a valid [`CommandBufferInheritanceInfo`] structure
-    If [`command_buffer`] is a secondary command buffer and either the `occlusionQueryEnable` member of the `pInheritanceInfo` member of [`p_begin_info`] is `VK_FALSE`, or the precise occlusion queries feature is not enabled, then `pBeginInfo->pInheritanceInfo->queryFlags` **must**  not contain `VK_QUERY_CONTROL_PRECISE_BIT`
-    If [`command_buffer`] is a primary command buffer, then `pBeginInfo->flags` **must**  not set both the `VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT` and the `VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT` flags

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_begin_info`] **must**  be a valid pointer to a valid [`CommandBufferBeginInfo`] structure

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`CommandBufferBeginInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        