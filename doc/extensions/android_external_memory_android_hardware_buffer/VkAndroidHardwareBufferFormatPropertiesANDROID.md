[VkAndroidHardwareBufferFormatPropertiesANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatPropertiesANDROID.html) - Structure describing the image format properties of an Android hardware buffer

# C Specifications
To obtain format properties of an Android hardware buffer, include a
[`AndroidHardwareBufferFormatPropertiesANDROID`] structure in the
[`p_next`] chain of the [`AndroidHardwareBufferPropertiesANDROID`]
structure passed to [`get_android_hardware_buffer_properties_android`].
This structure is defined as:
```c
// Provided by VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkAndroidHardwareBufferFormatPropertiesANDROID {
    VkStructureType                  sType;
    void*                            pNext;
    VkFormat                         format;
    uint64_t                         externalFormat;
    VkFormatFeatureFlags             formatFeatures;
    VkComponentMapping               samplerYcbcrConversionComponents;
    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
    VkSamplerYcbcrRange              suggestedYcbcrRange;
    VkChromaLocation                 suggestedXChromaOffset;
    VkChromaLocation                 suggestedYChromaOffset;
} VkAndroidHardwareBufferFormatPropertiesANDROID;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`format`] is the Vulkan format corresponding to the Android hardware buffer’s format, or `VK_FORMAT_UNDEFINED` if there is not an equivalent Vulkan format.
- [`external_format`] is an implementation-defined external format identifier for use with [`ExternalFormatANDROID`]. It  **must**  not be zero.
- [`format_features`] describes the capabilities of this external format when used with an image bound to memory imported from `buffer`.
- [`sampler_ycbcr_conversion_components`] is the component swizzle that  **should**  be used in [`SamplerYcbcrConversionCreateInfo`].
- [`suggested_ycbcr_model`] is a suggested color model to use in the [`SamplerYcbcrConversionCreateInfo`].
- [`suggested_ycbcr_range`] is a suggested numerical value range to use in [`SamplerYcbcrConversionCreateInfo`].
- [`suggested_x_chroma_offset`] is a suggested X chroma offset to use in [`SamplerYcbcrConversionCreateInfo`].
- [`suggested_y_chroma_offset`] is a suggested Y chroma offset to use in [`SamplerYcbcrConversionCreateInfo`].

# Description
If the Android hardware buffer has one of the formats listed in the
[Format Equivalence
table](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-external-android-hardware-buffer-formats), then [`format`] **must**  have the equivalent Vulkan format listed in
the table.
Otherwise, [`format`] **may**  be `VK_FORMAT_UNDEFINED`, indicating the
Android hardware buffer  **can**  only be used with an external format.The [`format_features`] member  **must**  include
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT` and at least one of
`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT` or
`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`, and  **should**  include
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`.Android hardware buffers with the same external format  **must**  have the same
support for `VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT`,
`VK_FORMAT_FEATURE_MIDPOINT_CHROMA_SAMPLES_BIT`,
`VK_FORMAT_FEATURE_COSITED_CHROMA_SAMPLES_BIT`,
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT`,
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT`,
and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT`.
in [`format_features`].
Other format features  **may**  differ between Android hardware buffers that have
the same external format.
This allows applications to use the same [`SamplerYcbcrConversion`]
object (and samplers and pipelines created from them) for any Android
hardware buffers that have the same external format.If [`format`] is not `VK_FORMAT_UNDEFINED`, then the value of
[`sampler_ycbcr_conversion_components`] **must**  be valid when used as the
`components` member of [`SamplerYcbcrConversionCreateInfo`] with
that format.
If [`format`] is `VK_FORMAT_UNDEFINED`, all members of
[`sampler_ycbcr_conversion_components`] **must**  be the
[identity swizzle](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-image-views-identity-mappings).Implementations  **may**  not always be able to determine the color model,
numerical range, or chroma offsets of the image contents, so the values in
[`AndroidHardwareBufferFormatPropertiesANDROID`] are only suggestions.
Applications  **should**  treat these values as sensible defaults to use in the
absence of more reliable information obtained through some other means.
If the underlying physical device is also usable via OpenGL ES with the
[`GL_OES_EGL_image_external`](https://www.khronos.org/registry/OpenGL/extensions/OES/OES_EGL_image_external.txt)
extension, the implementation  **should**  suggest values that will produce
similar sampled values as would be obtained by sampling the same external
image via `samplerExternalOES` in OpenGL ES using equivalent sampler
parameters.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_ANDROID`

# Related
- [`VK_ANDROID_external_memory_android_hardware_buffer`]
- [`ChromaLocation`]
- [`ComponentMapping`]
- [`Format`]
- [`FormatFeatureFlags`]
- [`SamplerYcbcrModelConversion`]
- [`SamplerYcbcrRange`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        