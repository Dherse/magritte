[VkAndroidHardwareBufferFormatProperties2ANDROID](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAndroidHardwareBufferFormatProperties2ANDROID.html) - Structure describing the image format properties of an Android hardware buffer

# C Specifications
The format properties of an Android hardware buffer  **can**  be obtained by
including a [`AndroidHardwareBufferFormatProperties2ANDROID`] structure
in the [`p_next`] chain of the
[`AndroidHardwareBufferPropertiesANDROID`] structure passed to
[`get_android_hardware_buffer_properties_android`].
This structure is defined as:
```c
// Provided by VK_KHR_format_feature_flags2 with VK_ANDROID_external_memory_android_hardware_buffer
typedef struct VkAndroidHardwareBufferFormatProperties2ANDROID {
    VkStructureType                  sType;
    void*                            pNext;
    VkFormat                         format;
    uint64_t                         externalFormat;
    VkFormatFeatureFlags2            formatFeatures;
    VkComponentMapping               samplerYcbcrConversionComponents;
    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
    VkSamplerYcbcrRange              suggestedYcbcrRange;
    VkChromaLocation                 suggestedXChromaOffset;
    VkChromaLocation                 suggestedYChromaOffset;
} VkAndroidHardwareBufferFormatProperties2ANDROID;
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
The bits reported in [`format_features`] **must**  include the bits reported in
the corresponding fields of
[`AndroidHardwareBufferFormatPropertiesANDROID`]::[`format_features`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ANDROID_HARDWARE_BUFFER_FORMAT_PROPERTIES_2_ANDROID`

# Related
- [`android_external_memory_android_hardware_buffer`]
- [`khr_format_feature_flags2`]
- [`ChromaLocation`]
- [`ComponentMapping`]
- [`Format`]
- [`FormatFeatureFlags2`]
- [`SamplerYcbcrModelConversion`]
- [`SamplerYcbcrRange`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        