[VkBufferCollectionPropertiesFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBufferCollectionPropertiesFUCHSIA.html) - Structure specifying the negotiated format chosen by Sysmem

# C Specifications
The [`BufferCollectionPropertiesFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkBufferCollectionPropertiesFUCHSIA {
    VkStructureType                  sType;
    void*                            pNext;
    uint32_t                         memoryTypeBits;
    uint32_t                         bufferCount;
    uint32_t                         createInfoIndex;
    uint64_t                         sysmemPixelFormat;
    VkFormatFeatureFlags             formatFeatures;
    VkSysmemColorSpaceFUCHSIA        sysmemColorSpaceIndex;
    VkComponentMapping               samplerYcbcrConversionComponents;
    VkSamplerYcbcrModelConversion    suggestedYcbcrModel;
    VkSamplerYcbcrRange              suggestedYcbcrRange;
    VkChromaLocation                 suggestedXChromaOffset;
    VkChromaLocation                 suggestedYChromaOffset;
} VkBufferCollectionPropertiesFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`memory_type_bits`] is a bitmask containing one bit set for every memory type which the buffer collection can be imported as buffer collection
- [`buffer_count`] is the number of buffers in the collection
- [`create_info_index`] as described in [Sysmem chosen create infos](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sysmem-chosen-create-infos)
- [`sysmem_pixel_format`] is the Sysmem `PixelFormatType` as defined in `fuchsia.sysmem/image_formats.fidl`
- [`format_features`] is a bitmask of [`FormatFeatureFlagBits`] shared by the buffer collection
- [`sysmem_color_space_index`] is a [`SysmemColorSpaceFUCHSIA`] struct specifying the color space
- [`sampler_ycbcr_conversion_components`] is a [`ComponentMapping`] struct specifying the component mapping
- [`suggested_ycbcr_model`] is a [`SamplerYcbcrModelConversion`] value specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> model
- [`suggested_ycbcr_range`] is a [`SamplerYcbcrRange`] value specifying the suggested Y′C<sub>B</sub>C<sub>R</sub> range
- [`suggested_x_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested X chroma offset
- [`suggested_y_chroma_offset`] is a [`ChromaLocation`] value specifying the suggested Y chroma offset

# Description
`sysmemColorSpace` is only set for image-based buffer collections where
the constraints were specified using [`ImageConstraintsInfoFUCHSIA`] in
a call to [`set_buffer_collection_image_constraints_fuchsia`].For image-based buffer collections, [`create_info_index`] will identify both
the [`ImageConstraintsInfoFUCHSIA`]`::pImageCreateInfos` element and
the [`ImageConstraintsInfoFUCHSIA::format_constraints`] element
chosen by Sysmem when [`set_buffer_collection_image_constraints_fuchsia`] was
called.
The value of [`sysmem_color_space_index`] will be an index to one of the
color spaces provided in the
[`ImageFormatConstraintsInfoFUCHSIA::color_spaces`] array.The implementation must have [`format_features`] with all bits set that
were set in
[`ImageFormatConstraintsInfoFUCHSIA::required_format_features`], by
the call to [`set_buffer_collection_image_constraints_fuchsia`], at
[`create_info_index`] (other bits could be set as well).
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_BUFFER_COLLECTION_PROPERTIES_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`] values
-  [`format_features`] **must**  not be `0`
-  [`sysmem_color_space_index`] **must**  be a valid [`SysmemColorSpaceFUCHSIA`] structure
-  [`sampler_ycbcr_conversion_components`] **must**  be a valid [`ComponentMapping`] structure
-  [`suggested_ycbcr_model`] **must**  be a valid [`SamplerYcbcrModelConversion`] value
-  [`suggested_ycbcr_range`] **must**  be a valid [`SamplerYcbcrRange`] value
-  [`suggested_x_chroma_offset`] **must**  be a valid [`ChromaLocation`] value
-  [`suggested_y_chroma_offset`] **must**  be a valid [`ChromaLocation`] value

# Related
- [`fuchsia_buffer_collection`]
- [`ChromaLocation`]
- [`ComponentMapping`]
- [VkFormatFeatureFlags]()
- [`SamplerYcbcrModelConversion`]
- [`SamplerYcbcrRange`]
- [`StructureType`]
- [`SysmemColorSpaceFUCHSIA`]
- [`get_buffer_collection_properties_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        