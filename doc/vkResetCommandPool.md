[vkResetCommandPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetCommandPool.html) - Reset a command pool

# C Specifications
To reset a command pool, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkResetCommandPool(
    VkDevice                                    device,
    VkCommandPool                               commandPool,
    VkCommandPoolResetFlags                     flags);
```

# Parameters
- [`device`] is the logical device that owns the command pool.
- [`command_pool`] is the command pool to reset.
- [`flags`] is a bitmask of [`CommandPoolResetFlagBits`] controlling the reset operation.

# Description
Resetting a command pool recycles all of the resources from all of the
command buffers allocated from the command pool back to the command pool.
All command buffers that have been allocated from the command pool are put
in the [initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).Any primary command buffer allocated from another [`CommandPool`] that
is in the [recording or executable state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle) and
has a secondary command buffer allocated from [`command_pool`] recorded
into it, becomes [invalid](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).
## Valid Usage
-    All [`CommandBuffer`] objects allocated from [`command_pool`] **must**  not be in the [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`command_pool`] **must**  be a valid [`CommandPool`] handle
-  [`flags`] **must**  be a valid combination of [`CommandPoolResetFlagBits`] values
-  [`command_pool`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`command_pool`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`crate::vulkan1_0`]
- [`CommandPool`]
- [VkCommandPoolResetFlags]()
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        