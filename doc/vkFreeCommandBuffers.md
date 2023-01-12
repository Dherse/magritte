[vkFreeCommandBuffers](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeCommandBuffers.html) - Free command buffers

# C Specifications
To free command buffers, call:
```c
// Provided by VK_VERSION_1_0
void vkFreeCommandBuffers(
    VkDevice                                    device,
    VkCommandPool                               commandPool,
    uint32_t                                    commandBufferCount,
    const VkCommandBuffer*                      pCommandBuffers);
```

# Parameters
- [`device`] is the logical device that owns the command pool.
- [`command_pool`] is the command pool from which the command buffers were allocated.
- [`command_buffer_count`] is the length of the [`p_command_buffers`] array.
- [`p_command_buffers`] is a pointer to an array of handles of command buffers to free.

# Description
Any primary command buffer that is in the [recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle) and has any element of [`p_command_buffers`]
recorded into it, becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage
-    All elements of [`p_command_buffers`] **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)
-  [`p_command_buffers`] **must**  be a valid pointer to an array of [`command_buffer_count`][`CommandBuffer`] handles, each element of which  **must**  either be a valid handle or `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`command_pool`] **must**  be a valid [`CommandPool`] handle
-  [`command_buffer_count`] **must**  be greater than `0`
-  [`command_pool`] **must**  have been created, allocated, or retrieved from [`device`]
-    Each element of [`p_command_buffers`] that is a valid handle  **must**  have been created, allocated, or retrieved from [`command_pool`]

## Host Synchronization
- Host access to [`command_pool`] **must**  be externally synchronized
- Host access to each member of [`p_command_buffers`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`CommandPool`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        