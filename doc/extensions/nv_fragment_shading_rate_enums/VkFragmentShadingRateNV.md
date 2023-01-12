[VkFragmentShadingRateNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateNV.html) - Enumeration with fragment shading rates

# C Specifications
If the `fragmentShadingRateEnums` feature is enabled, fragment shading
rates may be specified using the [`FragmentShadingRateNV`] enumerated
type defined as:
```c
// Provided by VK_NV_fragment_shading_rate_enums
typedef enum VkFragmentShadingRateNV {
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV = 0,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV = 1,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV = 4,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV = 5,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV = 6,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV = 9,
    VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV = 10,
    VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV = 11,
    VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV = 12,
    VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV = 13,
    VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV = 14,
    VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV = 15,
} VkFragmentShadingRateNV;
```

# Description
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x1 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x2 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 2x1 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 2x2 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 2x4 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 4x2 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 4x4 pixels.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x1 pixels, with two fragment shader invocations per fragment.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x1 pixels, with four fragment shader invocations per fragment.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x1 pixels, with eight fragment shader invocations per fragment.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies a fragment size of 1x1 pixels, with sixteen fragment shader invocations per fragment.
- [`VK_FRAGMENT_SHADING_RATE_NV`] specifies that any portions of a primitive that use that shading rate should be discarded without invoking any fragment shader.
To use the shading rates
[`VK_FRAGMENT_SHADING_RATE_NV`],
[`VK_FRAGMENT_SHADING_RATE_NV`],
[`VK_FRAGMENT_SHADING_RATE_NV`], and
[`VK_FRAGMENT_SHADING_RATE_NV`] as a pipeline,
primitive, or attachment shading rate, the
`supersampleFragmentShadingRates` feature  **must**  be enabled.
To use the shading rate [`VK_FRAGMENT_SHADING_RATE_NV`] as
a pipeline, primitive, or attachment shading rate, the
`noInvocationFragmentShadingRates` feature  **must**  be enabled.

# Related
- [`nv_fragment_shading_rate_enums`]
- [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]
- [`cmd_set_fragment_shading_rate_enum_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        