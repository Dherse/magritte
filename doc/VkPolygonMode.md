[VkPolygonMode](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPolygonMode.html) - Control polygon rasterization mode

# C Specifications
Possible values of the
[`PipelineRasterizationStateCreateInfo::polygon_mode`] property of
the currently active pipeline, specifying the method of rasterization for
polygons, are:
```c
// Provided by VK_VERSION_1_0
typedef enum VkPolygonMode {
    VK_POLYGON_MODE_FILL = 0,
    VK_POLYGON_MODE_LINE = 1,
    VK_POLYGON_MODE_POINT = 2,
  // Provided by VK_NV_fill_rectangle
    VK_POLYGON_MODE_FILL_RECTANGLE_NV = 1000153000,
} VkPolygonMode;
```

# Description
- [`POINT`] specifies that polygon vertices are drawn as points.
- [`LINE`] specifies that polygon edges are drawn as line segments.
- [`FILL`] specifies that polygons are rendered using the polygon rasterization rules in this section.
- [`FILL_RECTANGLE_NV`] specifies that polygons are rendered using polygon rasterization rules, modified to consider a sample within the primitive if the sample location is inside the axis-aligned bounding box of the triangle after projection. Note that the barycentric weights used in attribute interpolation  **can**  extend outside the range [0,1] when these primitives are shaded. Special treatment is given to a sample position on the boundary edge of the bounding box. In such a case, if two rectangles lie on either side of a common edge (with identical endpoints) on which a sample position lies, then exactly one of the triangles  **must**  produce a fragment that covers that sample during rasterization.Polygons rendered in [`FILL_RECTANGLE_NV`] mode  **may**  be clipped by the frustum or by user clip planes. If clipping is applied, the triangle is culled rather than clipped.Area calculation and facingness are determined for [`FILL_RECTANGLE_NV`] mode using the triangle’s vertices.
These modes affect only the final rasterization of polygons: in particular,
a polygon’s vertices are shaded and the polygon is clipped and possibly
culled before these modes are applied.

# Related
- [`crate::vulkan1_0`]
- [`PipelineRasterizationStateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        