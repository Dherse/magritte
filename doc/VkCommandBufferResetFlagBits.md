[VkCommandBufferResetFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferResetFlagBits.html) - Bitmask controlling behavior of a command buffer reset

# C Specifications
Bits which  **can**  be set in [`reset_command_buffer`]`::flags`,
controlling the reset operation, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCommandBufferResetFlagBits {
    VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
} VkCommandBufferResetFlagBits;
```

# Description
- [`RELEASE_RESOURCES`] specifies that most or all memory resources currently owned by the command buffer  **should**  be returned to the parent command pool. If this flag is not set, then the command buffer  **may**  hold onto memory resources and reuse them when recording commands. `commandBuffer` is moved to the [initial state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle).

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferResetFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        