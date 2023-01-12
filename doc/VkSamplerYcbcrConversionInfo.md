[VkSamplerYcbcrConversionInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionInfo.html) - Structure specifying Y′C<sub>B</sub>C<sub>R</sub> conversion to a sampler or image view

# C Specifications
To create a sampler with Y′C<sub>B</sub>C<sub>R</sub> conversion enabled, add a
[`SamplerYcbcrConversionInfo`] structure to the [`p_next`] chain of the
[`SamplerCreateInfo`] structure.
To create a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion, the
[`samplerYcbcrConversion` feature](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-samplerYcbcrConversion) **must**  be enabled.
Conversion  **must**  be fixed at pipeline creation time, through use of a
combined image sampler with an immutable sampler in
[`DescriptorSetLayoutBinding`].A [`SamplerYcbcrConversionInfo`] **must**  be provided for samplers to be
used with image views that access `VK_IMAGE_ASPECT_COLOR_BIT` if the
format is one of the [formats
that require a sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion)
, or if the image view has an
[external format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats)
.The [`SamplerYcbcrConversionInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkSamplerYcbcrConversionInfo {
    VkStructureType             sType;
    const void*                 pNext;
    VkSamplerYcbcrConversion    conversion;
} VkSamplerYcbcrConversionInfo;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrConversionInfo VkSamplerYcbcrConversionInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`conversion`] is a [`SamplerYcbcrConversion`] handle created with [`create_sampler_ycbcr_conversion`].

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO`
-  [`conversion`] **must**  be a valid [`SamplerYcbcrConversion`] handle

# Related
- [`crate::vulkan1_1`]
- [`SamplerYcbcrConversion`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        