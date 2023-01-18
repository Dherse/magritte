[VkSamplerMipmapMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerMipmapMode.html) - Specify mipmap mode used for texture lookups

# C Specifications
Possible values of the [`SamplerCreateInfo::mipmap_mode`],
specifying the mipmap mode used for texture lookups, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSamplerMipmapMode {
    VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
    VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
} VkSamplerMipmapMode;
```

# Description
- [`NEAREST`] specifies nearest filtering.
- [`LINEAR`] specifies linear filtering.
These modes are described in detail in [Texel
Filtering](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-texel-filtering).

# Related
- [`crate::vulkan1_0`]
- [`SamplerCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        