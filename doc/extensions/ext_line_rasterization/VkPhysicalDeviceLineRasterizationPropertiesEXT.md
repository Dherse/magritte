[VkPhysicalDeviceLineRasterizationPropertiesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationPropertiesEXT.html) - Structure describing line rasterization properties supported by an implementation

# C Specifications
The [`PhysicalDeviceLineRasterizationPropertiesEXT`] structure is
defined as:
```c
// Provided by VK_EXT_line_rasterization
typedef struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           lineSubPixelPrecisionBits;
} VkPhysicalDeviceLineRasterizationPropertiesEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`line_sub_pixel_precision_bits`] is the number of bits of subpixel precision in framebuffer coordinates x<sub>f</sub> and y<sub>f</sub> when rasterizing [line segments](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).

# Description
If the [`PhysicalDeviceLineRasterizationPropertiesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES_EXT`

# Related
- [`VK_EXT_line_rasterization`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        