[VkSamplerYcbcrConversionCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSamplerYcbcrConversionCreateInfo.html) - Structure specifying the parameters of the newly created conversion

# C Specifications
The [`SamplerYcbcrConversionCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_1
typedef struct VkSamplerYcbcrConversionCreateInfo {
    VkStructureType                  sType;
    const void*                      pNext;
    VkFormat                         format;
    VkSamplerYcbcrModelConversion    ycbcrModel;
    VkSamplerYcbcrRange              ycbcrRange;
    VkComponentMapping               components;
    VkChromaLocation                 xChromaOffset;
    VkChromaLocation                 yChromaOffset;
    VkFilter                         chromaFilter;
    VkBool32                         forceExplicitReconstruction;
} VkSamplerYcbcrConversionCreateInfo;
```
or the equivalent
```c
// Provided by VK_KHR_sampler_ycbcr_conversion
typedef VkSamplerYcbcrConversionCreateInfo VkSamplerYcbcrConversionCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format`] is the format of the image from which color information will be retrieved.
- [`ycbcr_model`] describes the color matrix for conversion between color models.
- [`ycbcr_range`] describes whether the encoded values have headroom and foot room, or whether the encoding uses the full numerical range.
- [`components`] applies a *swizzle* based on [`ComponentSwizzle`] enums prior to range expansion and color model conversion.
- [`x_chroma_offset`] describes the [sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with downsampled chroma components in the x dimension. [`x_chroma_offset`] has no effect for formats in which chroma components are not downsampled horizontally.
- [`y_chroma_offset`] describes the [sample location](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction) associated with downsampled chroma components in the y dimension. [`y_chroma_offset`] has no effect for formats in which the chroma components are not downsampled vertically.
- [`chroma_filter`] is the filter for chroma reconstruction.
- [`force_explicit_reconstruction`] **can**  be used to ensure that reconstruction is done explicitly, if supported.

# Description
If the [`p_next`] chain includes a [`ExternalFormatANDROID`] structure
with non-zero `externalFormat` member, the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion
object represents an *external format conversion*, and [`format`] **must**  be
`VK_FORMAT_UNDEFINED`.
Such conversions  **must**  only be used to sample image views with a matching
[external
format](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-external-formats).
When creating an external format conversion, the value of [`components`]
is ignored.
## Valid Usage
-    If an external format conversion is being created, [`format`] **must**  be `VK_FORMAT_UNDEFINED`
-    If an external format conversion is not being created, [`format`] **must**  represent unsigned normalized values (i.e. the format must be a `UNORM` format)
-    The [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion  **must**  support `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`
-    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`] **must**  not be `VK_CHROMA_LOCATION_COSITED_EVEN` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
-    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`, [`x_chroma_offset`] and [`y_chroma_offset`] **must**  not be `VK_CHROMA_LOCATION_MIDPOINT` if the corresponding components are [downsampled](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-chroma-reconstruction)
-    If the format has a `_422` or `_420` suffix, then `components.g` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings)
-    If the format has a `_422` or `_420` suffix, then `components.a` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), `VK_COMPONENT_SWIZZLE_ONE`, or `VK_COMPONENT_SWIZZLE_ZERO`
-    If the format has a `_422` or `_420` suffix, then `components.r` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) or `VK_COMPONENT_SWIZZLE_B`
-    If the format has a `_422` or `_420` suffix, then `components.b` **must**  be the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings) or `VK_COMPONENT_SWIZZLE_R`
-    If the format has a `_422` or `_420` suffix, and if either `components.r` or `components.b` is the [identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings), both values  **must**  be the identity swizzle
-    If [`ycbcr_model`] is not `VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY`, then `components.r`, `components.g`, and `components.b` **must**  correspond to components of the [`format`]; that is, `components.r`, `components.g`, and `components.b` **must**  not be `VK_COMPONENT_SWIZZLE_ZERO` or `VK_COMPONENT_SWIZZLE_ONE`, and  **must**  not correspond to a component containing zero or one as a consequence of [conversion to RGBA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-conversion-to-rgba)
-    If [`ycbcr_range`] is `VK_SAMPLER_YCBCR_RANGE_ITU_NARROW` then the R, G and B components obtained by applying the `component` swizzle to [`format`] **must**  each have a bit-depth greater than or equal to 8
-    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`[`force_explicit_reconstruction`] **must**  be [`FALSE`]
-    If the [potential format features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#potential-format-features) of the sampler Y′C<sub>B</sub>C<sub>R</sub> conversion do not support `VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`, [`chroma_filter`] **must**  not be `VK_FILTER_LINEAR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`ExternalFormatANDROID`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`format`] **must**  be a valid [`Format`] value
-  [`ycbcr_model`] **must**  be a valid [`SamplerYcbcrModelConversion`] value
-  [`ycbcr_range`] **must**  be a valid [`SamplerYcbcrRange`] value
-  [`components`] **must**  be a valid [`ComponentMapping`] structure
-  [`x_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
-  [`y_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
-  [`chroma_filter`] **must**  be a valid [`Filter`] value
If [`chroma_filter`] is `VK_FILTER_NEAREST`, chroma samples are
reconstructed to luma component resolution using nearest-neighbour sampling.
Otherwise, chroma samples are reconstructed using interpolation.
More details can be found in [the
description of sampler Y′C<sub>B</sub>C<sub>R</sub> conversion](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures-sampler-YCbCr-conversion) in the [Image
Operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#textures) chapter.

# Related
- [`crate::vulkan1_1`]
- [`Bool32`]
- [`ChromaLocation`]
- [`ComponentMapping`]
- [`Filter`]
- [`Format`]
- [`SamplerYcbcrModelConversion`]
- [`SamplerYcbcrRange`]
- [`StructureType`]
- [`create_sampler_ycbcr_conversion`]
- [`create_sampler_ycbcr_conversion_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        