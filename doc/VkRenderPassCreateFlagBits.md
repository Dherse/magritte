[VkRenderPassCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkRenderPassCreateFlagBits.html) - Bitmask specifying additional properties of a render pass

# C Specifications
Bits which  **can**  be set in [`RenderPassCreateInfo::flags`],
describing additional properties of the render pass, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkRenderPassCreateFlagBits {
  // Provided by VK_QCOM_render_pass_transform
    VK_RENDER_PASS_CREATE_TRANSFORM_BIT_QCOM = 0x00000002,
} VkRenderPassCreateFlagBits;
```

# Description
- [`TRANSFORM_QCOM`] specifies that the created render pass is compatible with [render pass transform](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#vertexpostproc-renderpass-transform).

# Related
- [`crate::vulkan1_0`]
- [VkRenderPassCreateFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        