[VkCommandPoolCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandPoolCreateFlagBits.html) - Bitmask specifying usage behavior for a command pool

# C Specifications
Bits which  **can**  be set in [`CommandPoolCreateInfo::flags`],
specifying usage behavior for a command pool, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCommandPoolCreateFlagBits {
    VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
    VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
  // Provided by VK_VERSION_1_1
    VK_COMMAND_POOL_CREATE_PROTECTED_BIT = 0x00000004,
} VkCommandPoolCreateFlagBits;
```

# Description
- [`TRANSIENT`] specifies that command buffers allocated from the pool will be short-lived, meaning that they will be reset or freed in a relatively short timeframe. This flag  **may**  be used by the implementation to control memory allocation behavior within the pool.
- [`RESET_COMMAND_BUFFER`] allows any command buffer allocated from a pool to be individually reset to the [initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle); either by calling [`reset_command_buffer`], or via the implicit reset when calling [`begin_command_buffer`]. If this flag is not set on a pool, then [`reset_command_buffer`] **must**  not be called for any command buffer allocated from that pool.
- [`PROTECTED`] specifies that command buffers allocated from the pool are protected command buffers.

# Related
- [`crate::vulkan1_0`]
- [`CommandPoolCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        