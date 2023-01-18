[VkPhysicalDeviceSampleLocationsPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceSampleLocationsPropertiesEXT.html) - Structure describing sample location limits that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceSampleLocationsPropertiesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_sample_locations
typedef struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    VkStructureType       sType;
    void*                 pNext;
    VkSampleCountFlags    sampleLocationSampleCounts;
    VkExtent2D            maxSampleLocationGridSize;
    float                 sampleLocationCoordinateRange[2];
    uint32_t              sampleLocationSubPixelBits;
    VkBool32              variableSampleLocations;
} VkPhysicalDeviceSampleLocationsPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`sample_location_sample_counts`] is a bitmask of [`SampleCountFlagBits`] indicating the sample counts supporting custom sample locations.
- [`max_sample_location_grid_size`] is the maximum size of the pixel grid in which sample locations  **can**  vary that is supported for all sample counts in [`sample_location_sample_counts`].
- [`sample_location_coordinate_range`][2] is the range of supported sample location coordinates.
- [`sample_location_sub_pixel_bits`] is the number of bits of subpixel precision for sample locations.
- [`variable_sample_locations`] specifies whether the sample locations used by all pipelines that will be bound to a command buffer during a subpass  **must**  match. If set to [`TRUE`], the implementation supports variable sample locations in a subpass. If set to [`FALSE`], then the sample locations  **must**  stay constant in each subpass.

# Description
If the [`PhysicalDeviceSampleLocationsPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLE_LOCATIONS_PROPERTIES_EXT`

# Related
- [`VK_EXT_sample_locations`]
- [`Bool32`]
- [`Extent2D`]
- [`SampleCountFlags`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        