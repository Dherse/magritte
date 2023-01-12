[vkTrimCommandPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkTrimCommandPool.html) - Trim a command pool

# C Specifications
To trim a command pool, call:
```c
// Provided by VK_VERSION_1_1
void vkTrimCommandPool(
    VkDevice                                    device,
    VkCommandPool                               commandPool,
    VkCommandPoolTrimFlags                      flags);
```
or the equivalent command
```c
// Provided by VK_KHR_maintenance1
void vkTrimCommandPoolKHR(
    VkDevice                                    device,
    VkCommandPool                               commandPool,
    VkCommandPoolTrimFlags                      flags);
```

# Parameters
- [`device`] is the logical device that owns the command pool.
- [`command_pool`] is the command pool to trim.
- [`flags`] is reserved for future use.

# Description
Trimming a command pool recycles unused memory from the command pool back to
the system.
Command buffers allocated from the pool are not affected by the command.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`command_pool`] **must**  be a valid [`CommandPool`] handle
-  [`flags`] **must**  be `0`
-  [`command_pool`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`command_pool`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_1`]
- [`CommandPool`]
- [`CommandPoolTrimFlags`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        