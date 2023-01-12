[VkFilter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkFilter.html) - Specify filters used for texture lookups

# C Specifications
Possible values of the [`SamplerCreateInfo::mag_filter`] and
`minFilter` parameters, specifying filters used for texture lookups,
are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkFilter {
    VK_FILTER_NEAREST = 0,
    VK_FILTER_LINEAR = 1,
  // Provided by VK_IMG_filter_cubic
    VK_FILTER_CUBIC_IMG = 1000015000,
  // Provided by VK_EXT_filter_cubic
    VK_FILTER_CUBIC_EXT = VK_FILTER_CUBIC_IMG,
} VkFilter;
```

# Description
- [`VK_FILTER`] specifies nearest filtering.
- [`VK_FILTER`] specifies linear filtering.
- [`CUBIC_EXT`] specifies cubic filtering.
These filters are described in detail in [Texel
Filtering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-filtering).

# Related
- [`crate::vulkan1_0`]
- [`BlitImageInfo2`]
- [`SamplerCreateInfo`]
- [`SamplerYcbcrConversionCreateInfo`]
- [`cmd_blit_image`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        