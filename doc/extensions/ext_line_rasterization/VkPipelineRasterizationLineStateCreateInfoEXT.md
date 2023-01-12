[VkPipelineRasterizationLineStateCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPipelineRasterizationLineStateCreateInfoEXT.html) - Structure specifying parameters of a newly created pipeline line rasterization state

# C Specifications
Line segment rasterization options are controlled by the
[`PipelineRasterizationLineStateCreateInfoEXT`] structure.The [`PipelineRasterizationLineStateCreateInfoEXT`] structure is defined
as:
```c
// Provided by VK_EXT_line_rasterization
typedef struct VkPipelineRasterizationLineStateCreateInfoEXT {
    VkStructureType               sType;
    const void*                   pNext;
    VkLineRasterizationModeEXT    lineRasterizationMode;
    VkBool32                      stippledLineEnable;
    uint32_t                      lineStippleFactor;
    uint16_t                      lineStipplePattern;
} VkPipelineRasterizationLineStateCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`line_rasterization_mode`] is a [`LineRasterizationModeEXT`] value selecting the style of line rasterization.
- [`stippled_line_enable`] enables [stippled line rasterization](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-lines-stipple).
- [`line_stipple_factor`] is the repeat factor used in stippled line rasterization.
- [`line_stipple_pattern`] is the bit pattern used in stippled line rasterization.

# Description
If [`stippled_line_enable`] is `VK_FALSE`, the values of
[`line_stipple_factor`] and [`line_stipple_pattern`] are ignored.
## Valid Usage
-    If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT`, then the [rectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rectangularLines) feature  **must**  be enabled
-    If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT`, then the [bresenhamLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bresenhamLines) feature  **must**  be enabled
-    If [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT`, then the [smoothLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bresenhamLines) feature  **must**  be enabled
-    If [`stippled_line_enable`] is `VK_TRUE` and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT`, then the [stippledRectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledRectangularLines) feature  **must**  be enabled
-    If [`stippled_line_enable`] is `VK_TRUE` and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT`, then the [stippledBresenhamLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledBresenhamLines) feature  **must**  be enabled
-    If [`stippled_line_enable`] is `VK_TRUE` and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT`, then the [stippledSmoothLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledSmoothLines) feature  **must**  be enabled
-    If [`stippled_line_enable`] is `VK_TRUE` and [`line_rasterization_mode`] is `VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT`, then the [stippledRectangularLines](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-stippledRectangularLines) feature  **must**  be enabled and [`PhysicalDeviceLimits::strict_lines`] **must**  be `VK_TRUE`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO_EXT`
-  [`line_rasterization_mode`] **must**  be a valid [`LineRasterizationModeEXT`] value

# Related
- [`ext_line_rasterization`]
- [`Bool32`]
- [`LineRasterizationModeEXT`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        