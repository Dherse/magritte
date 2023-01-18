[VkConservativeRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkConservativeRasterizationModeEXT.html) - Specify the conservative rasterization mode

# C Specifications
Possible values of
[`PipelineRasterizationConservativeStateCreateInfoEXT::conservative_rasterization_mode`],
specifying the conservative rasterization mode are:
```c
// Provided by VK_EXT_conservative_rasterization
typedef enum VkConservativeRasterizationModeEXT {
    VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT = 0,
    VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT = 1,
    VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT = 2,
} VkConservativeRasterizationModeEXT;
```

# Description
- [`DISABLED`] specifies that conservative rasterization is disabled and rasterization proceeds as normal.
- [`OVERESTIMATE`] specifies that conservative rasterization is enabled in overestimation mode.
- [`UNDERESTIMATE`] specifies that conservative rasterization is enabled in underestimation mode.

# Related
- [`VK_EXT_conservative_rasterization`]
- [`PipelineRasterizationConservativeStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        