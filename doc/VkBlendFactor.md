[VkBlendFactor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendFactor.html) - Framebuffer blending factors

# C Specifications
The source and destination color and alpha blending factors are selected
from the enum:
```c
// Provided by VK_VERSION_1_0
typedef enum VkBlendFactor {
    VK_BLEND_FACTOR_ZERO = 0,
    VK_BLEND_FACTOR_ONE = 1,
    VK_BLEND_FACTOR_SRC_COLOR = 2,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
    VK_BLEND_FACTOR_DST_COLOR = 4,
    VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
    VK_BLEND_FACTOR_SRC_ALPHA = 6,
    VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
    VK_BLEND_FACTOR_DST_ALPHA = 8,
    VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
    VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
    VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
    VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
    VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
    VK_BLEND_FACTOR_SRC1_COLOR = 15,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
    VK_BLEND_FACTOR_SRC1_ALPHA = 17,
    VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
} VkBlendFactor;
```

# Description
The semantics of the enum values are described in the table below:In this table, the following conventions are used:
- R<sub>s0</sub>,G<sub>s0</sub>,B<sub>s0</sub> and A<sub>s0</sub> represent the first source color R, G, B, and A components, respectively, for the fragment output location corresponding to the color attachment being blended.
- R<sub>s1</sub>,G<sub>s1</sub>,B<sub>s1</sub> and A<sub>s1</sub> represent the second source color R, G, B, and A components, respectively, used in dual source blending modes, for the fragment output location corresponding to the color attachment being blended.
- R<sub>d</sub>,G<sub>d</sub>,B<sub>d</sub> and A<sub>d</sub> represent the R, G, B, and A components of the destination color. That is, the color currently in the corresponding color attachment for this fragment/sample.
- R<sub>c</sub>,G<sub>c</sub>,B<sub>c</sub> and A<sub>c</sub> represent the blend constant R, G, B, and A components, respectively.

# Related
- [`crate::vulkan1_0`]
- [`PipelineColorBlendAttachmentState`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        