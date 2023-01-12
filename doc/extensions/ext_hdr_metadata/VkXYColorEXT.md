[VkXYColorEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkXYColorEXT.html) - Specify X,Y chromaticity coordinates

# C Specifications
The [`XyColorEXT`] structure is defined as:
```c
// Provided by VK_EXT_hdr_metadata
typedef struct VkXYColorEXT {
    float    x;
    float    y;
} VkXYColorEXT;
```

# Members
- [`x`] is the x chromaticity coordinate.
- [`y`] is the y chromaticity coordinate.

# Description
Chromaticity coordinates are as specified in CIE 15:2004 “Calculation of
chromaticity coordinates” (Section 7.3) and are limited to between 0 and 1
for real colors for the reference monitor.

# Related
- [`ext_hdr_metadata`]
- [`HdrMetadataEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        