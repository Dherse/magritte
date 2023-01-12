[VkLogicOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLogicOp.html) - Framebuffer logical operations

# C Specifications
Logical operations are controlled by the `logicOpEnable` and
`logicOp` members of [`PipelineColorBlendStateCreateInfo`].
It can also be controlled by [`cmd_set_logic_op_ext`] if graphics pipeline
is created with `VK_DYNAMIC_STATE_LOGIC_OP_EXT` set in
[`PipelineDynamicStateCreateInfo::dynamic_states`].
If `logicOpEnable` is `VK_TRUE`, then a logical operation selected
by `logicOp` is applied between each color attachment and the fragment’s
corresponding output value, and blending of all attachments is treated as if
it were disabled.
Any attachments using color formats for which logical operations are not
supported simply pass through the color values unmodified.
The logical operation is applied independently for each of the red, green,
blue, and alpha components.
The `logicOp` is selected from the following operations:
```c
// Provided by VK_VERSION_1_0
typedef enum VkLogicOp {
    VK_LOGIC_OP_CLEAR = 0,
    VK_LOGIC_OP_AND = 1,
    VK_LOGIC_OP_AND_REVERSE = 2,
    VK_LOGIC_OP_COPY = 3,
    VK_LOGIC_OP_AND_INVERTED = 4,
    VK_LOGIC_OP_NO_OP = 5,
    VK_LOGIC_OP_XOR = 6,
    VK_LOGIC_OP_OR = 7,
    VK_LOGIC_OP_NOR = 8,
    VK_LOGIC_OP_EQUIVALENT = 9,
    VK_LOGIC_OP_INVERT = 10,
    VK_LOGIC_OP_OR_REVERSE = 11,
    VK_LOGIC_OP_COPY_INVERTED = 12,
    VK_LOGIC_OP_OR_INVERTED = 13,
    VK_LOGIC_OP_NAND = 14,
    VK_LOGIC_OP_SET = 15,
} VkLogicOp;
```

# Description
The logical operations supported by Vulkan are summarized in the following
table in which
- ¬ is bitwise invert,
- ∧ is bitwise and,
- ∨ is bitwise or,
- ⊕ is bitwise exclusive or,
- s is the fragment’s R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub> or A<sub>s0</sub> component value for the fragment output corresponding to the color attachment being updated, and
- d is the color attachment’s R, G, B or A component value:
The result of the logical operation is then written to the color attachment
as controlled by the component write mask, described in
[Blend Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blendoperations).

# Related
- [`crate::vulkan1_0`]
- [`PipelineColorBlendStateCreateInfo`]
- [`cmd_set_logic_op_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        