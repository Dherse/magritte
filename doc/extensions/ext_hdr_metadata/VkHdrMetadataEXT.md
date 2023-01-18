[VkHdrMetadataEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkHdrMetadataEXT.html) - Specify Hdr metadata

# C Specifications
The [`HdrMetadataEXT`] structure is defined as:
```c
// Provided by VK_EXT_hdr_metadata
typedef struct VkHdrMetadataEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkXYColorEXT       displayPrimaryRed;
    VkXYColorEXT       displayPrimaryGreen;
    VkXYColorEXT       displayPrimaryBlue;
    VkXYColorEXT       whitePoint;
    float              maxLuminance;
    float              minLuminance;
    float              maxContentLightLevel;
    float              maxFrameAverageLightLevel;
} VkHdrMetadataEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`display_primary_red`] is a [`XyColorEXT`] structure specifying the reference monitor’s red primary in chromaticity coordinates
- [`display_primary_green`] is a [`XyColorEXT`] structure specifying the reference monitor’s green primary in chromaticity coordinates
- [`display_primary_blue`] is a [`XyColorEXT`] structure specifying the reference monitor’s blue primary in chromaticity coordinates
- [`white_point`] is a [`XyColorEXT`] structure specifying the reference monitor’s white-point in chromaticity coordinates
- [`max_luminance`] is the maximum luminance of the reference monitor in nits
- [`min_luminance`] is the minimum luminance of the reference monitor in nits
- [`max_content_light_level`] is content’s maximum luminance in nits
- [`max_frame_average_light_level`] is the maximum frame average light level in nits

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_HDR_METADATA_EXT`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_EXT_hdr_metadata`]
- [`StructureType`]
- [`XyColorEXT`]
- [`set_hdr_metadata_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        