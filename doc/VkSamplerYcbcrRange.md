[VkSamplerYcbcrRange](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrRange.html) - Range of encoded values in a color space

# C Specifications
The [`SamplerYcbcrRange`] enum describes whether color components are
encoded using the full range of numerical values or whether values are
reserved for headroom and foot room.
[`SamplerYcbcrRange`] is defined as:
```c
// Provided by VK_VERSION_1_1
typedef enum VkSamplerYcbcrRange {
    VK_SAMPLER_YCBCR_RANGE_ITU_FULL = 0,
    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW = 1,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_RANGE_ITU_FULL_KHR = VK_SAMPLER_YCBCR_RANGE_ITU_FULL,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_RANGE_ITU_NARROW_KHR = VK_SAMPLER_YCBCR_RANGE_ITU_NARROW,
} VkSamplerYcbcrRange;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrRange VkSamplerYcbcrRangeKHR;
```

# Description
- [`VK_SAMPLER_YCBCR_RANGE`] specifies that the full range of the encoded values are valid and interpreted according to the ITU “full range” quantization rules.
- [`VK_SAMPLER_YCBCR_RANGE`] specifies that headroom and foot room are reserved in the numerical range of encoded values, and the remaining values are expanded according to the ITU “narrow range” quantization rules.
The formulae for these conversions is described in the
[Sampler Y′C<sub>B</sub>C<sub>R</sub> Range
Expansion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion-rangeexpand) section of the [Image Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures) chapter.No range modification takes place if `ycbcrModel` is
`VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`; the `ycbcrRange`
field of [`SamplerYcbcrConversionCreateInfo`] is ignored in this case.

# Related
- [`crate::vulkan1_1`]
- [`AndroidHardwareBufferFormatProperties2ANDROID`]
- [`AndroidHardwareBufferFormatPropertiesANDROID`]
- [`BufferCollectionPropertiesFUCHSIA`]
- [`SamplerYcbcrConversionCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        