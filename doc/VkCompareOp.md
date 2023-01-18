[VkCompareOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCompareOp.html) - Stencil comparison function

# C Specifications
Possible values of [`StencilOpState::compare_op`], specifying the
stencil comparison function, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkCompareOp {
    VK_COMPARE_OP_NEVER = 0,
    VK_COMPARE_OP_LESS = 1,
    VK_COMPARE_OP_EQUAL = 2,
    VK_COMPARE_OP_LESS_OR_EQUAL = 3,
    VK_COMPARE_OP_GREATER = 4,
    VK_COMPARE_OP_NOT_EQUAL = 5,
    VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
    VK_COMPARE_OP_ALWAYS = 7,
} VkCompareOp;
```

# Description
- [`NEVER`] specifies that the test evaluates to false.
- [`LESS`] specifies that the test evaluates A < B.
- [`EQUAL`] specifies that the test evaluates A = B.
- [`LESS_OR_EQUAL`] specifies that the test evaluates A ≤ B.
- [`GREATER`] specifies that the test evaluates A > B.
- [`NOT_EQUAL`] specifies that the test evaluates A ≠ B.
- [`GREATER_OR_EQUAL`] specifies that the test evaluates A ≥ B.
- [`ALWAYS`] specifies that the test evaluates to true.

# Related
- [`crate::vulkan1_0`]
- [`PipelineDepthStencilStateCreateInfo`]
- [`SamplerCreateInfo`]
- [`StencilOpState`]
- [`cmd_set_depth_compare_op`]
- [`cmd_set_depth_compare_op_ext`]
- [`cmd_set_stencil_op`]
- [`cmd_set_stencil_op_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        