[VkClearDepthStencilValue](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkClearDepthStencilValue.html) - Structure specifying a clear depth stencil value

# C Specifications
The [`ClearDepthStencilValue`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkClearDepthStencilValue {
    float       depth;
    uint32_t    stencil;
} VkClearDepthStencilValue;
```

# Members
- [`depth`] is the clear value for the depth aspect of the depth/stencil attachment. It is a floating-point value which is automatically converted to the attachment’s format.
- [`stencil`] is the clear value for the stencil aspect of the depth/stencil attachment. It is a 32-bit integer value which is converted to the attachment’s format by taking the appropriate number of LSBs.

# Description
## Valid Usage
-    Unless the `[`ext_depth_range_unrestricted`]` extension is enabled [`depth`] **must**  be between `0.0` and `1.0`, inclusive

# Related
- [`crate::vulkan1_0`]
- [`ClearValue`]
- [`cmd_clear_depth_stencil_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        