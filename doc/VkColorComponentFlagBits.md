[VkColorComponentFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkColorComponentFlagBits.html) - Bitmask controlling which components are written to the framebuffer

# C Specifications
Bits which  **can**  be set in
[`PipelineColorBlendAttachmentState::color_write_mask`], determining
whether the final color values R, G, B and A are written to the
framebuffer attachment, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkColorComponentFlagBits {
    VK_COLOR_COMPONENT_R_BIT = 0x00000001,
    VK_COLOR_COMPONENT_G_BIT = 0x00000002,
    VK_COLOR_COMPONENT_B_BIT = 0x00000004,
    VK_COLOR_COMPONENT_A_BIT = 0x00000008,
} VkColorComponentFlagBits;
```

# Description
- [`R`] specifies that the R value is written to the color attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
- [`G`] specifies that the G value is written to the color attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
- [`B`] specifies that the B value is written to the color attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
- [`A`] specifies that the A value is written to the color attachment for the appropriate sample. Otherwise, the value in memory is unmodified.
The color write mask operation is applied regardless of whether blending is
enabled.The color write mask operation is applied only if
[Color Write Enable](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-enable) is enabled for the
respective attachment.
Otherwise the color write mask is ignored and writes to all components of
the attachment are disabled.

# Related
- [`crate::vulkan1_0`]
- [`ColorComponentFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        