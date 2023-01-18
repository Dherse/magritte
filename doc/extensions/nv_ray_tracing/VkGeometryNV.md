[VkGeometryNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryNV.html) - Structure specifying a geometry in a bottom-level acceleration structure

# C Specifications
The [`GeometryNV`] structure describes geometry in a bottom-level
acceleration structure and is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkGeometryNV {
    VkStructureType       sType;
    const void*           pNext;
    VkGeometryTypeKHR     geometryType;
    VkGeometryDataNV      geometry;
    VkGeometryFlagsKHR    flags;
} VkGeometryNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`geometry_type`] specifies the [`GeometryTypeKHR`] which this geometry refers to.
- [`geometry`] contains the geometry data as described in [`GeometryDataNV`].
- [`flags`] has [`GeometryFlagBitsKHR`] describing options for this geometry.

# Description
## Valid Usage
-  [`geometry_type`] **must**  be `VK_GEOMETRY_TYPE_TRIANGLES_NV` or `VK_GEOMETRY_TYPE_AABBS_NV`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_NV`
-  [`p_next`] **must**  be `NULL`
-  [`geometry_type`] **must**  be a valid [`GeometryTypeKHR`] value
-  [`geometry`] **must**  be a valid [`GeometryDataNV`] structure
-  [`flags`] **must**  be a valid combination of [`GeometryFlagBitsKHR`] values

# Related
- [`VK_NV_ray_tracing`]
- [`AccelerationStructureInfoNV`]
- [`GeometryDataNV`]
- [`GeometryFlagsKHR`]
- [`GeometryTypeKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        