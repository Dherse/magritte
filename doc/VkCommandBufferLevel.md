[VkCommandBufferLevel](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCommandBufferLevel.html) - Enumerant specifying a command buffer level

# C Specifications
Possible values of [`CommandBufferAllocateInfo::level`],
specifying the command buffer level, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCommandBufferLevel {
    VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
    VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
} VkCommandBufferLevel;
```

# Description
- [`VK_COMMAND_BUFFER_LEVEL`] specifies a primary command buffer.
- [`VK_COMMAND_BUFFER_LEVEL`] specifies a secondary command buffer.

# Related
- [`crate::vulkan1_0`]
- [`CommandBufferAllocateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        