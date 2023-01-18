[VkCoarseSampleOrderCustomNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCoarseSampleOrderCustomNV.html) - Structure specifying parameters controlling shading rate image usage

# C Specifications
The [`CoarseSampleOrderCustomNV`] structure is defined as:
```c
// Provided by VK_NV_shading_rate_image
typedef struct VkCoarseSampleOrderCustomNV {
    VkShadingRatePaletteEntryNV        shadingRate;
    uint32_t                           sampleCount;
    uint32_t                           sampleLocationCount;
    const VkCoarseSampleLocationNV*    pSampleLocations;
} VkCoarseSampleOrderCustomNV;
```

# Members
- [`shading_rate`] is a shading rate palette entry that identifies the fragment width and height for the combination of fragment area and per-pixel coverage sample count to control.
- [`sample_count`] identifies the per-pixel coverage sample count for the combination of fragment area and coverage sample count to control.
- [`sample_location_count`] specifies the number of sample locations in the custom ordering.
- [`sample_locations`] is a pointer to an array of [`CoarseSampleLocationNV`] structures specifying the location of each sample in the custom ordering.

# Description
The [`CoarseSampleOrderCustomNV`] structure is used with a coverage
sample ordering type of `VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV` to
specify the order of coverage samples for one combination of fragment width,
fragment height, and coverage sample count.When using a custom sample ordering, element *j* in [`sample_locations`]
specifies a specific pixel location and
[sample index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask) that corresponds to
[coverage index](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-multisampling-coverage-mask)*j* in the
multi-pixel fragment.
## Valid Usage
-  [`shading_rate`] **must**  be a shading rate that generates fragments with more than one pixel
-  [`sample_count`] **must**  correspond to a sample count enumerated in [`SampleCountFlags`] whose corresponding bit is set in [`PhysicalDeviceLimits::framebuffer_no_attachments_sample_counts`]
-  [`sample_location_count`] **must**  be equal to the product of [`sample_count`], the fragment width for [`shading_rate`], and the fragment height for [`shading_rate`]
-  [`sample_location_count`] **must**  be less than or equal to the value of [`PhysicalDeviceShadingRateImagePropertiesNV::shading_rate_max_coarse_samples`]
-    The array [`sample_locations`] **must**  contain exactly one entry for every combination of valid values for `pixelX`, `pixelY`, and `sample` in the structure [`CoarseSampleOrderCustomNV`]

## Valid Usage (Implicit)
-  [`shading_rate`] **must**  be a valid [`ShadingRatePaletteEntryNV`] value
-  [`sample_locations`] **must**  be a valid pointer to an array of [`sample_location_count`][`CoarseSampleLocationNV`] structures
-  [`sample_location_count`] **must**  be greater than `0`

# Related
- [`VK_NV_shading_rate_image`]
- [`CoarseSampleLocationNV`]
- [`PipelineViewportCoarseSampleOrderStateCreateInfoNV`]
- [`ShadingRatePaletteEntryNV`]
- [`cmd_set_coarse_sample_order_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        