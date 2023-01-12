[VkPipelineDepthStencilStateCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineDepthStencilStateCreateFlagBits.html) - Bitmask specifying additional depth/stencil state information.

# C Specifications
Bits which  **can**  be set in the
[`PipelineDepthStencilStateCreateInfo::flags`] parameter are:
```c
// Provided by VK_ARM_rasterization_order_attachment_access
typedef enum VkPipelineDepthStencilStateCreateFlagBits {
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_BIT_ARM = 0x00000001,
  // Provided by VK_ARM_rasterization_order_attachment_access
    VK_PIPELINE_DEPTH_STENCIL_STATE_CREATE_RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_BIT_ARM = 0x00000002,
} VkPipelineDepthStencilStateCreateFlagBits;
```

# Description
- [`RASTERIZATION_ORDER_ATTACHMENT_DEPTH_ACCESS_ARM`] indicates that access to the depth aspects of depth/stencil and input attachments will have implicit framebuffer-local memory dependencies. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more information.
- [`RASTERIZATION_ORDER_ATTACHMENT_STENCIL_ACCESS_ARM`] indicates that access to the stencil aspects of depth/stencil and input attachments will have implicit framebuffer-local memory dependencies. See [renderpass feedback loops](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#renderpass-feedbackloop) for more information.

# Related
- [`arm_rasterization_order_attachment_access`]
- [VkPipelineDepthStencilStateCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        