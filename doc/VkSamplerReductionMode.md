[VkSamplerReductionMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerReductionMode.html) - Specify reduction mode for texture filtering

# C Specifications
Reduction modes are specified by [`SamplerReductionMode`], which takes
values:
```c
// Provided by VK_VERSION_1_2
typedef enum VkSamplerReductionMode {
    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE = 0,
    VK_SAMPLER_REDUCTION_MODE_MIN = 1,
    VK_SAMPLER_REDUCTION_MODE_MAX = 2,
  // Provided by VK_EXT_sampler_filter_minmax
    VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE_EXT = VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE,
  // Provided by VK_EXT_sampler_filter_minmax
    VK_SAMPLER_REDUCTION_MODE_MIN_EXT = VK_SAMPLER_REDUCTION_MODE_MIN,
  // Provided by VK_EXT_sampler_filter_minmax
    VK_SAMPLER_REDUCTION_MODE_MAX_EXT = VK_SAMPLER_REDUCTION_MODE_MAX,
} VkSamplerReductionMode;
```
or the equivalent
```c
// Provided by VK_EXT_sampler_filter_minmax
typedef VkSamplerReductionMode VkSamplerReductionModeEXT;
```

# Description
- [`VK_SAMPLER_REDUCTION_MODE`] specifies that texel values are combined by computing a weighted average of values in the footprint, using weights as specified in [the image operations chapter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-unnormalized-to-integer).
- [`VK_SAMPLER_REDUCTION_MODE`] specifies that texel values are combined by taking the component-wise minimum of values in the footprint with non-zero weights.
- [`VK_SAMPLER_REDUCTION_MODE`] specifies that texel values are combined by taking the component-wise maximum of values in the footprint with non-zero weights.

# Related
- [`ext_sampler_filter_minmax`]
- [`crate::vulkan1_2`]
- [`SamplerReductionModeCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        