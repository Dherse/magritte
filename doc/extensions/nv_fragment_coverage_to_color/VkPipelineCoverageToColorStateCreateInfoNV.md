[VkPipelineCoverageToColorStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineCoverageToColorStateCreateInfoNV.html) - Structure specifying whether fragment coverage replaces a color

# C Specifications
The [`PipelineCoverageToColorStateCreateInfoNV`] structure is defined
as:
```c
// Provided by VK_NV_fragment_coverage_to_color
typedef struct VkPipelineCoverageToColorStateCreateInfoNV {
    VkStructureType                                sType;
    const void*                                    pNext;
    VkPipelineCoverageToColorStateCreateFlagsNV    flags;
    VkBool32                                       coverageToColorEnable;
    uint32_t                                       coverageToColorLocation;
} VkPipelineCoverageToColorStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`coverage_to_color_enable`] controls whether the fragment coverage value replaces a fragment color output.
- [`coverage_to_color_location`] controls which fragment shader color output value is replaced.

# Description
If the [`p_next`] chain of [`PipelineMultisampleStateCreateInfo`]
includes a [`PipelineCoverageToColorStateCreateInfoNV`] structure, then
that structure controls whether the fragment coverage is substituted for a
fragment color output and, if so, which output is replaced.If [`coverage_to_color_enable`] is [`TRUE`], the
[coverage mask](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) replaces the first
component of the color value corresponding to the fragment shader output
location with `Location` equal to [`coverage_to_color_location`] and
`Index` equal to zero.
If the color attachment format has fewer bits than the coverage mask, the
low bits of the sample coverage mask are taken without any clamping.
If the color attachment format has more bits than the coverage mask, the
high bits of the sample coverage mask are filled with zeros.If [`coverage_to_color_enable`] is [`FALSE`], these operations are
skipped.
If this structure is not included in the [`p_next`] chain, it is as if
[`coverage_to_color_enable`] is [`FALSE`].
## Valid Usage
-    If [`coverage_to_color_enable`] is [`TRUE`], then the render pass subpass indicated by [`GraphicsPipelineCreateInfo::render_pass`] and [`GraphicsPipelineCreateInfo::subpass`] **must**  have a color attachment at the location selected by [`coverage_to_color_location`], with a [`Format`] of `VK_FORMAT_R8_UINT`, `VK_FORMAT_R8_SINT`, `VK_FORMAT_R16_UINT`, `VK_FORMAT_R16_SINT`, `VK_FORMAT_R32_UINT`, or `VK_FORMAT_R32_SINT`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_COVERAGE_TO_COLOR_STATE_CREATE_INFO_NV`
-  [`flags`] **must**  be `0`

# Related
- [`VK_NV_fragment_coverage_to_color`]
- [`Bool32`]
- [`PipelineCoverageToColorStateCreateFlagsNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        