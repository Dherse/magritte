[VkStencilOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkStencilOp.html) - Stencil comparison function

# C Specifications
Possible values of the `failOp`, `passOp`, and `depthFailOp`
members of [`StencilOpState`], specifying what happens to the stored
stencil value if this or certain subsequent tests fail or pass, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkStencilOp {
    VK_STENCIL_OP_KEEP = 0,
    VK_STENCIL_OP_ZERO = 1,
    VK_STENCIL_OP_REPLACE = 2,
    VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
    VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
    VK_STENCIL_OP_INVERT = 5,
    VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
    VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
} VkStencilOp;
```

# Description
- [`KEEP`] keeps the current value.
- [`ZERO`] sets the value to 0.
- [`REPLACE`] sets the value to `reference`.
- [`INCREMENT_AND_CLAMP`] increments the current value and clamps to the maximum representable unsigned value.
- [`DECREMENT_AND_CLAMP`] decrements the current value and clamps to 0.
- [`INVERT`] bitwise-inverts the current value.
- [`INCREMENT_AND_WRAP`] increments the current value and wraps to 0 when the maximum value would have been exceeded.
- [`DECREMENT_AND_WRAP`] decrements the current value and wraps to the maximum possible value when the value would go below 0.
For purposes of increment and decrement, the stencil bits are considered as
an unsigned integer.

# Related
- [`crate::vulkan1_0`]
- [`StencilOpState`]
- [`cmd_set_stencil_op`]
- [`cmd_set_stencil_op_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        