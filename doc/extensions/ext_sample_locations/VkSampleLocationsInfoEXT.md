[VkSampleLocationsInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampleLocationsInfoEXT.html) - Structure specifying a set of sample locations

# C Specifications
The [`SampleLocationsInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkSampleLocationsInfoEXT {
    VkStructureType               sType;
    const void*                   pNext;
    VkSampleCountFlagBits         sampleLocationsPerPixel;
    VkExtent2D                    sampleLocationGridSize;
    uint32_t                      sampleLocationsCount;
    const VkSampleLocationEXT*    pSampleLocations;
} VkSampleLocationsInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`sample_locations_per_pixel`] is a [`SampleCountFlagBits`] value specifying the number of sample locations per pixel.
- [`sample_location_grid_size`] is the size of the sample location grid to select custom sample locations for.
- [`sample_locations_count`] is the number of sample locations in [`sample_locations`].
- [`sample_locations`] is a pointer to an array of [`sample_locations_count`][`SampleLocationEXT`] structures.

# Description
This structure  **can**  be used either to specify the sample locations to be
used for rendering or to specify the set of sample locations an image
subresource has been last rendered with for the purposes of layout
transitions of depth/stencil images created with
`VK_IMAGE_CREATE_SAMPLE_LOCATIONS_COMPATIBLE_DEPTH_BIT_EXT`.The sample locations in [`sample_locations`] specify
[`sample_locations_per_pixel`] number of sample locations for each pixel in
the grid of the size specified in [`sample_location_grid_size`].
The sample location for sample i at the pixel grid location
(x,y) is taken from [`sample_locations`][(x +  y ×
`sampleLocationGridSize.width`) × [`sample_locations_per_pixel`]
+  i].If the render pass has a fragment density map, the implementation will
choose the sample locations for the fragment and the contents of
[`sample_locations`] **may**  be ignored.
## Valid Usage
-  [`sample_locations_per_pixel`] **must**  be a bit value that is set in [`PhysicalDeviceSampleLocationsPropertiesEXT::sample_location_sample_counts`]
-  [`sample_locations_count`] **must**  equal [`sample_locations_per_pixel`] × `sampleLocationGridSize.width` × `sampleLocationGridSize.height`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SAMPLE_LOCATIONS_INFO_EXT`
-    If [`sample_locations_count`] is not `0`, [`sample_locations`] **must**  be a valid pointer to an array of [`sample_locations_count`][`SampleLocationEXT`] structures

# Related
- [`VK_EXT_sample_locations`]
- [`AttachmentSampleLocationsEXT`]
- [`Extent2D`]
- [`PipelineSampleLocationsStateCreateInfoEXT`]
- [`SampleCountFlagBits`]
- [`SampleLocationEXT`]
- [`StructureType`]
- [`SubpassSampleLocationsEXT`]
- [`cmd_set_sample_locations_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        