[VkCommandBufferUsageFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferUsageFlagBits.html) - Bitmask specifying usage behavior for command buffer

# C Specifications
Bits which  **can**  be set in [`CommandBufferBeginInfo::flags`],
specifying usage behavior for a command buffer, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCommandBufferUsageFlagBits {
    VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
    VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
    VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,
} VkCommandBufferUsageFlagBits;
```

# Description
- [`ONE_TIME_SUBMIT`] specifies that each recording of the command buffer will only be submitted once, and the command buffer will be reset and recorded again between each submission.
- [`RENDER_PASS_CONTINUE`] specifies that a secondary command buffer is considered to be entirely inside a render pass. If this is a primary command buffer, then this bit is ignored.
- [`SIMULTANEOUS_USE`] specifies that a command buffer  **can**  be resubmitted to a queue while it is in the *pending state*, and recorded into multiple primary command buffers.

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferUsageFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        