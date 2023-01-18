[VkShadingRatePaletteNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShadingRatePaletteNV.html) - Structure specifying a single shading rate palette

# C Specifications
The [`ShadingRatePaletteNV`] structure specifies to contents of a single
shading rate image palette and is defined as:
```c
// Provided by VK_NV_shading_rate_image
typedef struct VkShadingRatePaletteNV {
    uint32_t                              shadingRatePaletteEntryCount;
    const VkShadingRatePaletteEntryNV*    pShadingRatePaletteEntries;
} VkShadingRatePaletteNV;
```

# Members
- [`shading_rate_palette_entry_count`] specifies the number of entries in the shading rate image palette.
- [`shading_rate_palette_entries`] is a pointer to an array of [`ShadingRatePaletteEntryNV`] enums defining the shading rate for each palette entry.

# Description
## Valid Usage
-  [`shading_rate_palette_entry_count`] **must**  be between `1` and [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_palette_size`], inclusive

## Valid Usage (Implicit)
-  [`shading_rate_palette_entries`] **must**  be a valid pointer to an array of [`shading_rate_palette_entry_count`] valid [`ShadingRatePaletteEntryNV`] values
-  [`shading_rate_palette_entry_count`] **must**  be greater than `0`

# Related
- [`VK_NV_shading_rate_image`]
- [`PipelineViewportShadingRateImageStateCreateInfoNV`]
- [`ShadingRatePaletteEntryNV`]
- [`cmd_set_viewport_shading_rate_palette_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        