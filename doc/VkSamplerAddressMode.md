[VkSamplerAddressMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerAddressMode.html) - Specify behavior of sampling with texture coordinates outside an image

# C Specifications
Possible values of the [`SamplerCreateInfo`]::`addressMode*`
parameters, specifying the behavior of sampling with coordinates outside the
range [0,1] for the respective u, v, or w coordinate
as defined in the [Wrapping Operation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-wrapping-operation)
section, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkSamplerAddressMode {
    VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
    VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
    VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
  // Provided by VK_VERSION_1_2, VK_KHR_sampler_mirror_clamp_to_edge
    VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
  // Provided by VK_KHR_sampler_mirror_clamp_to_edge
    VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE_KHR = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE,
} VkSamplerAddressMode;
```

# Description
- [`REPEAT`] specifies that the repeat wrap mode will be used.
- [`MIRRORED_REPEAT`] specifies that the mirrored repeat wrap mode will be used.
- [`CLAMP_TO_EDGE`] specifies that the clamp to edge wrap mode will be used.
- [`CLAMP_TO_BORDER`] specifies that the clamp to border wrap mode will be used.
- [`MIRROR_CLAMP_TO_EDGE`] specifies that the     mirror clamp to edge wrap mode will be used.     This is only valid if [samplerMirrorClampToEdge](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerMirrorClampToEdge) is enabled, or if     the `[`VK_KHR_sampler_mirror_clamp_to_edge`]` extension is enabled.

# Related
- [`crate::vulkan1_0`]
- [`SamplerCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        