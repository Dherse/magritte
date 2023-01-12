[VkSamplerYcbcrModelConversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrModelConversion.html) - Color model component of a color space

# C Specifications
[`SamplerYcbcrModelConversion`] defines the conversion from the source
color model to the shader color model.
Possible values are:
```c
// Provided by VK_VERSION_1_1
typedef enum VkSamplerYcbcrModelConversion {
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY = 0,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY = 1,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 = 2,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 = 3,
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 = 4,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY_KHR = VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY_KHR = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709_KHR = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601_KHR = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601,
  // Provided by VK_KHR_sampler_ycbcr_conversion
    VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020_KHR = VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020,
} VkSamplerYcbcrModelConversion;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrModelConversion VkSamplerYcbcrModelConversionKHR;
```

# Description
- [`VK_SAMPLER_YCBCR_MODEL_CONVERSION`] specifies that the input values to the conversion are unmodified.
- [`VK_SAMPLER_YCBCR_MODEL_CONVERSION`] specifies no model conversion but the inputs are range expanded as for Y′C<sub>B</sub>C<sub>R</sub>.
- [`VK_SAMPLER_YCBCR_MODEL_CONVERSION`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.709 and described in the “BT.709 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
- [`VK_SAMPLER_YCBCR_MODEL_CONVERSION`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.601 and described in the “BT.601 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
- [`VK_SAMPLER_YCBCR_MODEL_CONVERSION`] specifies the color model conversion from Y′C<sub>B</sub>C<sub>R</sub> to R′G′B′ defined in BT.2020 and described in the “BT.2020 Y′C<sub>B</sub>C<sub>R</sub> conversion” section of the [Khronos Data Format Specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#data-format).
In the `VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_*` color models, for the
input to the sampler Y′C<sub>B</sub>C<sub>R</sub> range expansion and model conversion:
- the Y (Y′ luma) component corresponds to the G component of an RGB image.
- the CB (C<sub>B</sub> or “U” blue color difference) component corresponds to the B component of an RGB image.
- the CR (C<sub>R</sub> or “V” red color difference) component corresponds to the R component of an RGB image.
- the alpha component, if present, is not modified by color model conversion.
These rules reflect the mapping of components after the component swizzle
operation (controlled by
[`SamplerYcbcrConversionCreateInfo::components`]).

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
        