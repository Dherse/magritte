[VkClearValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearValue.html) - Structure specifying a clear value

# C Specifications
The [`ClearValue`] union is defined as:
```c
// Provided by VK_VERSION_1_0
typedef union VkClearValue {
    VkClearColorValue           color;
    VkClearDepthStencilValue    depthStencil;
} VkClearValue;
```

# Members
- [`color`] specifies the color image clear values to use when clearing a color image or attachment.
- [`depth_stencil`] specifies the depth and stencil clear values to use when clearing a depth/stencil image or attachment.

# Description
This union is used where part of the API requires either color or
depth/stencil clear values, depending on the attachment, and defines the
initial clear values in the [`RenderPassBeginInfo`] structure.

# Related
- [`crate::vulkan1_0`]
- [`ClearAttachment`]
- [`ClearColorValue`]
- [`ClearDepthStencilValue`]
- [`RenderPassBeginInfo`]
- [`RenderingAttachmentInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        