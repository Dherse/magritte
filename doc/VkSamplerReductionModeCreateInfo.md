[VkSamplerReductionModeCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionModeCreateInfo.html) - Structure specifying sampler reduction mode

# C Specifications
The [`SamplerReductionModeCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_2
typedef struct VkSamplerReductionModeCreateInfo {
    VkStructureType           sType;
    const void*               pNext;
    VkSamplerReductionMode    reductionMode;
} VkSamplerReductionModeCreateInfo;
```
or the equivalent
```c
// Provided by VK_EXT_sampler_filter_minmax
typedef VkSamplerReductionModeCreateInfo VkSamplerReductionModeCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`reduction_mode`] is a [`SamplerReductionMode`] value controlling how texture filtering combines texel values.

# Description
If the [`p_next`] chain of [`SamplerCreateInfo`] includes a
[`SamplerReductionModeCreateInfo`] structure, then that structure
includes a mode controlling how texture filtering combines texel values.If this structure is not present, [`reduction_mode`] is considered to be
`VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE`.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO`
-  [`reduction_mode`] **must**  be a valid [`SamplerReductionMode`] value

# Related
- [`VK_EXT_sampler_filter_minmax`]
- [`crate::vulkan1_2`]
- [`SamplerReductionMode`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        