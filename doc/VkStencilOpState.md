[VkStencilOpState](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilOpState.html) - Structure specifying stencil operation state

# C Specifications
The [`StencilOpState`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkStencilOpState {
    VkStencilOp    failOp;
    VkStencilOp    passOp;
    VkStencilOp    depthFailOp;
    VkCompareOp    compareOp;
    uint32_t       compareMask;
    uint32_t       writeMask;
    uint32_t       reference;
} VkStencilOpState;
```

# Members
- [`fail_op`] is a [`StencilOp`] value specifying the action performed on samples that fail the stencil test.
- [`pass_op`] is a [`StencilOp`] value specifying the action performed on samples that pass both the depth and stencil tests.
- [`depth_fail_op`] is a [`StencilOp`] value specifying the action performed on samples that pass the stencil test and fail the depth test.
- [`compare_op`] is a [`CompareOp`] value specifying the comparison operator used in the stencil test.
- [`compare_mask`] selects the bits of the unsigned integer stencil values participating in the stencil test.
- [`write_mask`] selects the bits of the unsigned integer stencil values updated by the stencil test in the stencil framebuffer attachment.
- [`reference`] is an integer reference value that is used in the unsigned stencil comparison.

# Description
## Valid Usage (Implicit)
-  [`fail_op`] **must**  be a valid [`StencilOp`] value
-  [`pass_op`] **must**  be a valid [`StencilOp`] value
-  [`depth_fail_op`] **must**  be a valid [`StencilOp`] value
-  [`compare_op`] **must**  be a valid [`CompareOp`] value

# Related
- [`crate::vulkan1_0`]
- [`CompareOp`]
- [`PipelineDepthStencilStateCreateInfo`]
- [`StencilOp`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        