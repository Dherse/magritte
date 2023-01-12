[VkPipelineViewportShadingRateImageStateCreateInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineViewportShadingRateImageStateCreateInfoNV.html) - Structure specifying parameters controlling shading rate image usage

# C Specifications
If the [`p_next`] chain of [`PipelineViewportStateCreateInfo`] includes
a [`PipelineViewportShadingRateImageStateCreateInfoNV`] structure, then
that structure includes parameters controlling the shading rate.The [`PipelineViewportShadingRateImageStateCreateInfoNV`] structure is
defined as:
```c
// Provided by VK_NV_shading_rate_image
typedef struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
    VkStructureType                  sType;
    const void*                      pNext;
    VkBool32                         shadingRateImageEnable;
    uint32_t                         viewportCount;
    const VkShadingRatePaletteNV*    pShadingRatePalettes;
} VkPipelineViewportShadingRateImageStateCreateInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`shading_rate_image_enable`] specifies whether shading rate image and palettes are used during rasterization.
- [`viewport_count`] specifies the number of per-viewport palettes used to translate values stored in shading rate images.
- [`shading_rate_palettes`] is a pointer to an array of [`ShadingRatePaletteNV`] structures defining the palette for each viewport. If the shading rate palette state is dynamic, this member is ignored.

# Description
If this structure is not present, [`shading_rate_image_enable`] is considered
to be `VK_FALSE`, and the shading rate image and palettes are not used.
## Valid Usage
-    If the [multiple viewports](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-multiViewport) feature is not enabled, [`viewport_count`] **must**  be `0` or `1`
-  [`viewport_count`] **must**  be less than or equal to [`PhysicalDeviceLimits::max_viewports`]
-    If [`shading_rate_image_enable`] is `VK_TRUE`, [`viewport_count`] **must**  be greater or equal to the [`viewport_count`] member of [`PipelineViewportStateCreateInfo`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_SHADING_RATE_IMAGE_STATE_CREATE_INFO_NV`

# Related
- [`nv_shading_rate_image`]
- [`Bool32`]
- [`ShadingRatePaletteNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        