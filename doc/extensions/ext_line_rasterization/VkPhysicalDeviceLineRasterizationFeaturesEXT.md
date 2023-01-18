[VkPhysicalDeviceLineRasterizationFeaturesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceLineRasterizationFeaturesEXT.html) - Structure describing the line rasterization features that can be supported by an implementation

# C Specifications
The [`PhysicalDeviceLineRasterizationFeaturesEXT`] structure is defined
as:
```c
// Provided by VK_EXT_line_rasterization
typedef struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
    VkStructureType    sType;
    void*              pNext;
    VkBool32           rectangularLines;
    VkBool32           bresenhamLines;
    VkBool32           smoothLines;
    VkBool32           stippledRectangularLines;
    VkBool32           stippledBresenhamLines;
    VkBool32           stippledSmoothLines;
} VkPhysicalDeviceLineRasterizationFeaturesEXT;
```

# Members
This structure describes the following features:

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`rectangular_lines`] indicates whether the implementation supports [rectangular line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines).
- [`bresenham_lines`] indicates whether the implementation supports [Bresenham-style line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
- [`smooth_lines`] indicates whether the implementation supports [smooth line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).
- [`stippled_rectangular_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT` lines, or with `VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT` lines if [`PhysicalDeviceLimits::strict_lines`] is [`TRUE`].
- [`stippled_bresenham_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT` lines.
- [`stippled_smooth_lines`] indicates whether the implementation supports [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple) with `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT` lines.
If the [`PhysicalDeviceLineRasterizationFeaturesEXT`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceFeatures2`] structure passed to
[`get_physical_device_features2`], it is filled in to indicate whether each
corresponding feature is supported.
[`PhysicalDeviceLineRasterizationFeaturesEXT`] **can**  also be used in the [`p_next`] chain of
[`DeviceCreateInfo`] to selectively enable these features.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES_EXT`

# Related
- [`VK_EXT_line_rasterization`]
- [`Bool32`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        