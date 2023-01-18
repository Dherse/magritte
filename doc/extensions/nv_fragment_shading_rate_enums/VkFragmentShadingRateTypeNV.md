[VkFragmentShadingRateTypeNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFragmentShadingRateTypeNV.html) - Enumeration with fragment shading rate types

# C Specifications
The [`FragmentShadingRateTypeNV`] enumerated type specifies whether a
graphics pipeline gets its pipeline fragment shading rates and combiners
from the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure
or the [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure.
```c
// Provided by VK_NV_fragment_shading_rate_enums
typedef enum VkFragmentShadingRateTypeNV {
    VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV = 0,
    VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV = 1,
} VkFragmentShadingRateTypeNV;
```

# Description
- [`FRAGMENT_SIZE`] specifies that a graphics pipeline should obtain its pipeline fragment shading rate and shading rate combiner state from the [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure and that any state specified by the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure should be ignored.
- [`ENUMS`] specifies that a graphics pipeline should obtain its pipeline fragment shading rate and shading rate combiner state from the [`PipelineFragmentShadingRateEnumStateCreateInfoNV`] structure and that any state specified by the [`PipelineFragmentShadingRateStateCreateInfoKHR`] structure should be ignored.

# Related
- [`VK_NV_fragment_shading_rate_enums`]
- [`PipelineFragmentShadingRateEnumStateCreateInfoNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        