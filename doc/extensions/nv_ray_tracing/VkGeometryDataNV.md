[VkGeometryDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryDataNV.html) - Structure specifying geometry in a bottom-level acceleration structure

# C Specifications
The [`GeometryDataNV`] structure specifes geometry in a bottom-level
acceleration structure and is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkGeometryDataNV {
    VkGeometryTrianglesNV    triangles;
    VkGeometryAABBNV         aabbs;
} VkGeometryDataNV;
```

# Members
- [`triangles`] contains triangle data if [`GeometryNV::geometry_type`] is `VK_GEOMETRY_TYPE_TRIANGLES_NV`.
- [`aabbs`] contains axis-aligned bounding box data if [`GeometryNV::geometry_type`] is `VK_GEOMETRY_TYPE_AABBS_NV`.

# Description
## Valid Usage (Implicit)
-  [`triangles`] **must**  be a valid [`GeometryTrianglesNV`] structure
-  [`aabbs`] **must**  be a valid [`GeometryAabbNV`] structure

# Related
- [`VK_NV_ray_tracing`]
- [`GeometryAabbNV`]
- [`GeometryNV`]
- [`GeometryTrianglesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        