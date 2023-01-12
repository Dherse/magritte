[VkRenderingFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderingFlagBits.html) - Bitmask specifying additional properties of a dynamic render pass instance

# C Specifications
Bits which  **can**  be set in [`RenderingInfo::flags`] describing
additional properties of the render pass are:
```c
// Provided by VK_VERSION_1_3
typedef enum VkRenderingFlagBits {
    VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = 0x00000001,
    VK_RENDERING_SUSPENDING_BIT = 0x00000002,
    VK_RENDERING_RESUMING_BIT = 0x00000004,
    VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT_KHR = VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT,
    VK_RENDERING_SUSPENDING_BIT_KHR = VK_RENDERING_SUSPENDING_BIT,
    VK_RENDERING_RESUMING_BIT_KHR = VK_RENDERING_RESUMING_BIT,
} VkRenderingFlagBits;
```
or the equivalent
```c
// Provided by VK_KHR_dynamic_rendering
typedef VkRenderingFlagBits VkRenderingFlagBitsKHR;
```

# Description
- [`VK_RENDERING_FLAG_BITS`] specifies that draw calls for the render pass instance will be recorded in secondary command buffers.
- [`VK_RENDERING_FLAG_BITS`] specifies that the render pass instance is resuming an earlier suspended render pass instance.
- [`VK_RENDERING_FLAG_BITS`] specifies that the render pass instance will be suspended.
The contents of `pRenderingInfo` **must**  match between suspended render
pass instances and the render pass instances that resume them, other than
the presence or absence of the [`VK_RENDERING_FLAG_BITS`],
[`VK_RENDERING_FLAG_BITS`], and
[`VK_RENDERING_FLAG_BITS`] flags.
No action or synchronization commands, or other render pass instances, are
allowed between suspending and resuming render pass instances.

# Related
- [`khr_dynamic_rendering`]
- [`crate::vulkan1_3`]
- [VkRenderingFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        