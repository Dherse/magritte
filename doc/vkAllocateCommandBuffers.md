[vkAllocateCommandBuffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAllocateCommandBuffers.html) - Allocate command buffers from an existing command pool

# C Specifications
To allocate command buffers, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkAllocateCommandBuffers(
    VkDevice                                    device,
    const VkCommandBufferAllocateInfo*          pAllocateInfo,
    VkCommandBuffer*                            pCommandBuffers);
```

# Parameters
- [`device`] is the logical device that owns the command pool.
- [`p_allocate_info`] is a pointer to a [`CommandBufferAllocateInfo`] structure describing parameters of the allocation.
- [`p_command_buffers`] is a pointer to an array of [`CommandBuffer`] handles in which the resulting command buffer objects are returned. The array  **must**  be at least the length specified by the `commandBufferCount` member of [`p_allocate_info`]. Each allocated command buffer begins in the initial state.

# Description
[`allocate_command_buffers`] **can**  be used to allocate multiple command
buffers.
If the allocation of any of those command buffers fails, the implementation
 **must**  free all successfully allocated command buffer objects from this
command, set all entries of the [`p_command_buffers`] array to `NULL` and
return the error.When command buffers are first allocated, they are in the
[initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_allocate_info`] **must**  be a valid pointer to a valid [`CommandBufferAllocateInfo`] structure
-  [`p_command_buffers`] **must**  be a valid pointer to an array of `pAllocateInfo->commandBufferCount`[`CommandBuffer`] handles
-  `pAllocateInfo->commandBufferCount` **must**  be greater than `0`

## Host Synchronization
- Host access to `pAllocateInfo->commandPool` **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`CommandBufferAllocateInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        