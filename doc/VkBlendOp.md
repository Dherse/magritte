[VkBlendOp](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOp.html) - Framebuffer blending operations

# C Specifications
Once the source and destination blend factors have been selected, they along
with the source and destination components are passed to the blending
operations.
RGB and alpha components  **can**  use different operations.
Possible values of [`BlendOp`], specifying the operations, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkBlendOp {
    VK_BLEND_OP_ADD = 0,
    VK_BLEND_OP_SUBTRACT = 1,
    VK_BLEND_OP_REVERSE_SUBTRACT = 2,
    VK_BLEND_OP_MIN = 3,
    VK_BLEND_OP_MAX = 4,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_ZERO_EXT = 1000148000,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SRC_EXT = 1000148001,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DST_EXT = 1000148002,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SRC_OVER_EXT = 1000148003,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DST_OVER_EXT = 1000148004,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SRC_IN_EXT = 1000148005,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DST_IN_EXT = 1000148006,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SRC_OUT_EXT = 1000148007,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DST_OUT_EXT = 1000148008,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SRC_ATOP_EXT = 1000148009,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DST_ATOP_EXT = 1000148010,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_XOR_EXT = 1000148011,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_MULTIPLY_EXT = 1000148012,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SCREEN_EXT = 1000148013,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_OVERLAY_EXT = 1000148014,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DARKEN_EXT = 1000148015,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_LIGHTEN_EXT = 1000148016,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_COLORDODGE_EXT = 1000148017,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_COLORBURN_EXT = 1000148018,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HARDLIGHT_EXT = 1000148019,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_SOFTLIGHT_EXT = 1000148020,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_DIFFERENCE_EXT = 1000148021,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_EXCLUSION_EXT = 1000148022,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_INVERT_EXT = 1000148023,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_INVERT_RGB_EXT = 1000148024,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_LINEARDODGE_EXT = 1000148025,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_LINEARBURN_EXT = 1000148026,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_VIVIDLIGHT_EXT = 1000148027,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_LINEARLIGHT_EXT = 1000148028,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_PINLIGHT_EXT = 1000148029,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HARDMIX_EXT = 1000148030,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HSL_HUE_EXT = 1000148031,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HSL_SATURATION_EXT = 1000148032,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HSL_COLOR_EXT = 1000148033,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_HSL_LUMINOSITY_EXT = 1000148034,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_PLUS_EXT = 1000148035,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_PLUS_CLAMPED_EXT = 1000148036,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_PLUS_CLAMPED_ALPHA_EXT = 1000148037,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_PLUS_DARKER_EXT = 1000148038,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_MINUS_EXT = 1000148039,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_MINUS_CLAMPED_EXT = 1000148040,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_CONTRAST_EXT = 1000148041,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_INVERT_OVG_EXT = 1000148042,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_RED_EXT = 1000148043,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_GREEN_EXT = 1000148044,
  // Provided by VK_EXT_blend_operation_advanced
    VK_BLEND_OP_BLUE_EXT = 1000148045,
} VkBlendOp;
```

# Description
The semantics of the basic blend operations are described in the table
below:In this table, the following conventions are used:
- R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub> and A<sub>s0</sub> represent the first source color R, G, B, and A components, respectively.
- R<sub>d</sub>, G<sub>d</sub>, B<sub>d</sub> and A<sub>d</sub> represent the R, G, B, and A components of the destination color. That is, the color currently in the corresponding color attachment for this fragment/sample.
- S<sub>r</sub>, S<sub>g</sub>, S<sub>b</sub> and S<sub>a</sub> represent the source blend factor R, G, B, and A components, respectively.
- D<sub>r</sub>, D<sub>g</sub>, D<sub>b</sub> and D<sub>a</sub> represent the destination blend factor R, G, B, and A components, respectively.
The blending operation produces a new set of values R, G, B and
A, which are written to the framebuffer attachment.
If blending is not enabled for this attachment, then R, G, B and
A are assigned R<sub>s0</sub>, G<sub>s0</sub>, B<sub>s0</sub> and A<sub>s0</sub>,
respectively.If the color attachment is fixed-point, the components of the source and
destination values and blend factors are each clamped to [0,1] or
[-1,1] respectively for an unsigned normalized or signed normalized
color attachment prior to evaluating the blend operations.
If the color attachment is floating-point, no clamping occurs.

# Related
- [`crate::vulkan1_0`]
- [`PipelineColorBlendAttachmentState`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        