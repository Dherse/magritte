[VkAccelerationStructureGeometryMotionTrianglesDataNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryMotionTrianglesDataNV.html) - Structure specifying vertex motion in a bottom-level acceleration structure

# C Specifications
The [`AccelerationStructureGeometryMotionTrianglesDataNV`] structure is
defined as:
```c
// Provided by VK_NV_ray_tracing_motion_blur
typedef struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
    VkStructureType                  sType;
    const void*                      pNext;
    VkDeviceOrHostAddressConstKHR    vertexData;
} VkAccelerationStructureGeometryMotionTrianglesDataNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`vertex_data`] is a pointer to vertex data for this geometry at time 1.0

# Description
If [`AccelerationStructureGeometryMotionTrianglesDataNV`] is included in
the [`p_next`] chain of a
[`AccelerationStructureGeometryTrianglesDataKHR`] structure, the basic
vertex positions are used for the position of the triangles in the geometry
at time 0.0 and the [`vertex_data`] in
[`AccelerationStructureGeometryMotionTrianglesDataNV`] is used for the
vertex positions at time 1.0, with positions linearly interpolated at
intermediate times.Indexing for [`AccelerationStructureGeometryMotionTrianglesDataNV`][`vertex_data`] is equivalent to the basic vertex position data.
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_MOTION_TRIANGLES_DATA_NV`

# Related
- [`VK_NV_ray_tracing_motion_blur`]
- [`DeviceOrHostAddressConstKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        