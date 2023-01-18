[VkPipelineColorBlendStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineColorBlendStateCreateFlagBits.html) - Bitmask specifying additional parameters of an image

# C Specifications
Bits which  **can**  be set in the
[`PipelineColorBlendStateCreateInfo::flags`] parameter are:
```c
// Provided by VK_ARM_rasterization_order_attachment_access
typedef enum VkPipelineColorBlendStateCreateFlagBits {
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_PIPELINE_COLOR_BLEND_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_BIT_ARM = 0x00000001,
} VkPipelineColorBlendStateCreateFlagBits;
```

# Description
- [`RASTERIZATION_ORDER_ATTACHMENT_ACCESS_ARM`] indicates that access to color and input attachments will have implicit framebuffer-local memory dependencies, allowing applications to express custom blending operations in a fragment shader. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more information.

# Related
- [`VK_ARM_rasterization_order_attachment_access`]
- [`PipelineColorBlendStateCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        