[VkLineRasterizationModeEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkLineRasterizationModeEXT.html) - Line rasterization modes

# C Specifications
Possible values of
[`PipelineRasterizationLineStateCreateInfoEXT::line_rasterization_mode`]
are:
```c
// Provided by VK_EXT_line_rasterization
typedef enum VkLineRasterizationModeEXT {
    VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT = 0,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT = 1,
    VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT = 2,
    VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT = 3,
} VkLineRasterizationModeEXT;
```

# Description
- [`VK_LINE_RASTERIZATION_MODE_EXT`] is equivalent to [`VK_LINE_RASTERIZATION_MODE_EXT`] if [`PhysicalDeviceLimits::strict_lines`] is `VK_TRUE`, otherwise lines are drawn as non-`strictLines` parallelograms. Both of these modes are defined in [Basic Line Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-basic).
- [`VK_LINE_RASTERIZATION_MODE_EXT`] specifies lines drawn as if they were rectangles extruded from the line
- [`VK_LINE_RASTERIZATION_MODE_EXT`] specifies lines drawn by determining which pixel diamonds the line intersects and exits, as defined in [Bresenham Line Segment Rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-bresenham).
- [`VK_LINE_RASTERIZATION_MODE_EXT`] specifies lines drawn if they were rectangles extruded from the line, with alpha falloff, as defined in [Smooth Lines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-smooth).

# Related
- [`ext_line_rasterization`]
- [`PipelineRasterizationLineStateCreateInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        