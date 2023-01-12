[VkSamplerYcbcrConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversion.html) - Opaque handle to a device-specific sampler Y′C<sub>B</sub>C<sub>R</sub> conversion description

# C Specifications
A sampler Y′C<sub>B</sub>C<sub>R</sub> conversion is an opaque representation of a
device-specific sampler Y′C<sub>B</sub>C<sub>R</sub> conversion description, represented as a
[`SamplerYcbcrConversion`] handle:
```c
// Provided by VK_VERSION_1_1
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSamplerYcbcrConversion)
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrConversion VkSamplerYcbcrConversionKHR;
```

# Related
- [`crate::vulkan1_1`]
- [`SamplerYcbcrConversionInfo`]
- [`create_sampler_ycbcr_conversion`]
- [`create_sampler_ycbcr_conversion_khr`]
- [`destroy_sampler_ycbcr_conversion`]
- [`destroy_sampler_ycbcr_conversion_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        