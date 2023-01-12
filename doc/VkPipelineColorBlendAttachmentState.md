[VkPipelineColorBlendAttachmentState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendAttachmentState.html) - Structure specifying a pipeline color blend attachment state

# C Specifications
The [`PipelineColorBlendAttachmentState`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkPipelineColorBlendAttachmentState {
    VkBool32                 blendEnable;
    VkBlendFactor            srcColorBlendFactor;
    VkBlendFactor            dstColorBlendFactor;
    VkBlendOp                colorBlendOp;
    VkBlendFactor            srcAlphaBlendFactor;
    VkBlendFactor            dstAlphaBlendFactor;
    VkBlendOp                alphaBlendOp;
    VkColorComponentFlags    colorWriteMask;
} VkPipelineColorBlendAttachmentState;
```

# Members
- [`blend_enable`] controls whether blending is enabled for the corresponding color attachment. If blending is not enabled, the source fragment’s color for that attachment is passed through unmodified.
- [`src_color_blend_factor`] selects which blend factor is used to determine the source factors (S<sub>r</sub>,S<sub>g</sub>,S<sub>b</sub>).
- [`dst_color_blend_factor`] selects which blend factor is used to determine the destination factors (D<sub>r</sub>,D<sub>g</sub>,D<sub>b</sub>).
- [`color_blend_op`] selects which blend operation is used to calculate the RGB values to write to the color attachment.
- [`src_alpha_blend_factor`] selects which blend factor is used to determine the source factor S<sub>a</sub>.
- [`dst_alpha_blend_factor`] selects which blend factor is used to determine the destination factor D<sub>a</sub>.
- [`alpha_blend_op`] selects which blend operation is use to calculate the alpha values to write to the color attachment.
- [`color_write_mask`] is a bitmask of [`ColorComponentFlagBits`] specifying which of the R, G, B, and/or A components are enabled for writing, as described for the [Color Write Mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-color-write-mask).

# Description
## Valid Usage
-    If the [dual source blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-dualSrcBlend) feature is not enabled, [`src_color_blend_factor`] **must**  not be `VK_BLEND_FACTOR_SRC1_COLOR`, `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, or `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`
-    If the [dual source blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-dualSrcBlend) feature is not enabled, [`dst_color_blend_factor`] **must**  not be `VK_BLEND_FACTOR_SRC1_COLOR`, `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, or `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`
-    If the [dual source blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-dualSrcBlend) feature is not enabled, [`src_alpha_blend_factor`] **must**  not be `VK_BLEND_FACTOR_SRC1_COLOR`, `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, or `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`
-    If the [dual source blending](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-dualSrcBlend) feature is not enabled, [`dst_alpha_blend_factor`] **must**  not be `VK_BLEND_FACTOR_SRC1_COLOR`, `VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR`, `VK_BLEND_FACTOR_SRC1_ALPHA`, or `VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA`
-    If either of [`color_blend_op`] or [`alpha_blend_op`] is an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced), then [`color_blend_op`] **must**  equal [`alpha_blend_op`]
-    If [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_independent_blend`] is `VK_FALSE` and [`color_blend_op`] is an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced), then [`color_blend_op`] **must**  be the same for all attachments
-    If [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_independent_blend`] is `VK_FALSE` and [`alpha_blend_op`] is an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced), then [`alpha_blend_op`] **must**  be the same for all attachments
-    If [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_all_operations`] is `VK_FALSE`, then [`color_blend_op`] **must**  not be `VK_BLEND_OP_ZERO_EXT`, `VK_BLEND_OP_SRC_EXT`, `VK_BLEND_OP_DST_EXT`, `VK_BLEND_OP_SRC_OVER_EXT`, `VK_BLEND_OP_DST_OVER_EXT`, `VK_BLEND_OP_SRC_IN_EXT`, `VK_BLEND_OP_DST_IN_EXT`, `VK_BLEND_OP_SRC_OUT_EXT`, `VK_BLEND_OP_DST_OUT_EXT`, `VK_BLEND_OP_SRC_ATOP_EXT`, `VK_BLEND_OP_DST_ATOP_EXT`, `VK_BLEND_OP_XOR_EXT`, `VK_BLEND_OP_INVERT_EXT`, `VK_BLEND_OP_INVERT_RGB_EXT`, `VK_BLEND_OP_LINEARDODGE_EXT`, `VK_BLEND_OP_LINEARBURN_EXT`, `VK_BLEND_OP_VIVIDLIGHT_EXT`, `VK_BLEND_OP_LINEARLIGHT_EXT`, `VK_BLEND_OP_PINLIGHT_EXT`, `VK_BLEND_OP_HARDMIX_EXT`, `VK_BLEND_OP_PLUS_EXT`, `VK_BLEND_OP_PLUS_CLAMPED_EXT`, `VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT`, `VK_BLEND_OP_PLUS_DARKER_EXT`, `VK_BLEND_OP_MINUS_EXT`, `VK_BLEND_OP_MINUS_CLAMPED_EXT`, `VK_BLEND_OP_CONTRAST_EXT`, `VK_BLEND_OP_INVERT_OVG_EXT`, `VK_BLEND_OP_RED_EXT`, `VK_BLEND_OP_GREEN_EXT`, or `VK_BLEND_OP_BLUE_EXT`
-    If [`color_blend_op`] or [`alpha_blend_op`] is an [advanced blend operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced), then `colorAttachmentCount` of the subpass this pipeline is compiled against  **must**  be less than or equal to [`PhysicalDeviceBlendOperationAdvancedPropertiesEXT::advanced_blend_max_color_attachments`]
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::constant_alpha_color_blend_factors`] is `VK_FALSE`, [`src_color_blend_factor`] **must**  not be `VK_BLEND_FACTOR_CONSTANT_ALPHA` or `VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA`
-    If the `[`khr_portability_subset`]` extension is enabled, and [`PhysicalDevicePortabilitySubsetFeaturesKHR::constant_alpha_color_blend_factors`] is `VK_FALSE`, [`dst_color_blend_factor`] **must**  not be `VK_BLEND_FACTOR_CONSTANT_ALPHA` or `VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA`

## Valid Usage (Implicit)
-  [`src_color_blend_factor`] **must**  be a valid [`BlendFactor`] value
-  [`dst_color_blend_factor`] **must**  be a valid [`BlendFactor`] value
-  [`color_blend_op`] **must**  be a valid [`BlendOp`] value
-  [`src_alpha_blend_factor`] **must**  be a valid [`BlendFactor`] value
-  [`dst_alpha_blend_factor`] **must**  be a valid [`BlendFactor`] value
-  [`alpha_blend_op`] **must**  be a valid [`BlendOp`] value
-  [`color_write_mask`] **must**  be a valid combination of [`ColorComponentFlagBits`] values

# Related
- [`crate::vulkan1_0`]
- [`BlendFactor`]
- [`BlendOp`]
- [`Bool32`]
- [VkColorComponentFlags]()
- [`PipelineColorBlendStateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        