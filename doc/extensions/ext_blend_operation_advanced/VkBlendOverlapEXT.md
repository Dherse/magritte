[VkBlendOverlapEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBlendOverlapEXT.html) - Enumerant specifying the blend overlap parameter

# C Specifications
The weighting functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> are defined
in table [Advanced Blend Overlap
Modes](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#framebuffer-blend-advanced-overlap-modes).
In these functions, the A components of the source and destination colors
are taken to indicate the portion of the pixel covered by the fragment
(source) and the fragments previously accumulated in the pixel
(destination).
The functions p<sub>0</sub>, p<sub>1</sub>, and p<sub>2</sub> approximate the
relative portion of the pixel covered by the intersection of the source and
destination, covered only by the source, and covered only by the
destination, respectively.Possible values of
[`PipelineColorBlendAdvancedStateCreateInfoEXT::blend_overlap`],
specifying the blend overlap functions, are:
```c
// Provided by VK_EXT_blend_operation_advanced
typedef enum VkBlendOverlapEXT {
    VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0,
    VK_BLEND_OVERLAP_DISJOINT_EXT = 1,
    VK_BLEND_OVERLAP_CONJOINT_EXT = 2,
} VkBlendOverlapEXT;
```

# Description
- [`UNCORRELATED`] specifies that there is no correlation between the source and destination coverage.
- [`CONJOINT`] specifies that the source and destination coverage are considered to have maximal overlap.
- [`DISJOINT`] specifies that the source and destination coverage are considered to have minimal overlap.

# Related
- [`VK_EXT_blend_operation_advanced`]
- [`PipelineColorBlendAdvancedStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        