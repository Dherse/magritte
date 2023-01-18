[VkMultisamplePropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkMultisamplePropertiesEXT.html) - Structure returning information about sample count specific additional multisampling capabilities

# C Specifications
The [`MultisamplePropertiesEXT`] structure is defined as
```c
// Provided by VK_EXT_sample_locations
typedef struct VkMultisamplePropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkExtent2D         maxSampleLocationGridSize;
} VkMultisamplePropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_sample_location_grid_size`] is the maximum size of the pixel grid in which sample locations  **can**  vary.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_MULTISAMPLE_PROPERTIES_EXT`
-  [`p_next`] **must**  be `NULL`

# Related
- [`VK_EXT_sample_locations`]
- [`Extent2D`]
- [`StructureType`]
- [`get_physical_device_multisample_properties_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        