[VkImageFormatConstraintsInfoFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkImageFormatConstraintsInfoFUCHSIA.html) - Structure image-based buffer collection constraints

# C Specifications
The [`ImageFormatConstraintsInfoFUCHSIA`] structure is defined as:
```c
// Provided by VK_FUCHSIA_buffer_collection
typedef struct VkImageFormatConstraintsInfoFUCHSIA {
    VkStructureType                         sType;
    const void*                             pNext;
    VkImageCreateInfo                       imageCreateInfo;
    VkFormatFeatureFlags                    requiredFormatFeatures;
    VkImageFormatConstraintsFlagsFUCHSIA    flags;
    uint64_t                                sysmemPixelFormat;
    uint32_t                                colorSpaceCount;
    const VkSysmemColorSpaceFUCHSIA*        pColorSpaces;
} VkImageFormatConstraintsInfoFUCHSIA;
```

# Members
- [`s_type`] is the type of this structure
- [`p_next`] is `NULL` or a pointer to a structure extending this structure
- [`image_create_info`] is the [`ImageCreateInfo`] used to create a [`Image`] that is to use memory from the [`BufferCollectionFUCHSIA`]
- [`required_format_features`] is a bitmask of [`FormatFeatureFlagBits`] specifying required features of the buffers in the buffer collection
- [`flags`] is reserved for future use
- [`sysmem_pixel_format`] is a `PixelFormatType` value from the `fuchsia.sysmem/image_formats.fidl` FIDL interface
- [`color_space_count`] the element count of [`color_spaces`]
- [`color_spaces`] is a pointer to an array of [`SysmemColorSpaceFUCHSIA`] structs of size [`color_space_count`]

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_IMAGE_FORMAT_CONSTRAINTS_INFO_FUCHSIA`
-  [`p_next`] **must**  be `NULL`
-  [`image_create_info`] **must**  be a valid [`ImageCreateInfo`] structure
-  [`required_format_features`] **must**  be a valid combination of [`FormatFeatureFlagBits`] values
-  [`required_format_features`] **must**  not be `0`
-  [`flags`] **must**  be `0`
-  [`color_spaces`] **must**  be a valid pointer to an array of [`color_space_count`] valid [`SysmemColorSpaceFUCHSIA`] structures
-  [`color_space_count`] **must**  be greater than `0`

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`FormatFeatureFlags`]
- [`ImageConstraintsInfoFUCHSIA`]
- [`ImageCreateInfo`]
- [`ImageFormatConstraintsFlagsFUCHSIA`]
- [`StructureType`]
- [`SysmemColorSpaceFUCHSIA`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        