[VkSamplerCreateFlagBits](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerCreateFlagBits.html) - Bitmask specifying additional parameters of sampler

# C Specifications
Bits which  **can**  be set in [`SamplerCreateInfo::flags`], specifying
additional parameters of a sampler, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSamplerCreateFlagBits {
  // Provided by VK_EXT_fragment_density_map
    VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT = 0x00000001,
  // Provided by VK_EXT_fragment_density_map
    VK_SAMPLER_CREATE_SUBSAMPLED_COARSE_RECONSTRUCTION_BIT_EXT = 0x00000002,
} VkSamplerCreateFlagBits;
```

# Description
- [`SUBSAMPLED_EXT`] specifies that the sampler will read from an image created with `flags` containing `VK_IMAGE_CREATE_SUBSAMPLED_BIT_EXT`.
- [`SUBSAMPLED_COARSE_RECONSTRUCTION_EXT`] specifies that the implementation  **may**  use approximations when reconstructing a full color value for texture access from a subsampled image.

# Related
- [`crate::vulkan1_0`]
- [`SamplerCreateFlags`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        