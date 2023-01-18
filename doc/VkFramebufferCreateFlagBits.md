[VkFramebufferCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFramebufferCreateFlagBits.html) - Bitmask specifying framebuffer properties

# C Specifications
Bits which  **can**  be set in [`FramebufferCreateInfo::flags`],
specifying options for framebuffers, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkFramebufferCreateFlagBits {
  // Provided by VK_VERSION_1_2
    VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT = 0x00000001,
  // Provided by VK_KHR_imageless_framebuffer
    VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT_KHR = VK_FRAMEBUFFER_CREATE_IMAGELESS_BIT,
} VkFramebufferCreateFlagBits;
```

# Description
- [`IMAGELESS`] specifies that image views are not specified, and only attachment compatibility information will be provided via a [`FramebufferAttachmentImageInfo`] structure.

# Related
- [`crate::vulkan1_0`]
- [`FramebufferCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        