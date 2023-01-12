[VkAccelerationStructureBuildRangeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) - Structure specifying build offsets and counts for acceleration structure builds

# C Specifications
[`AccelerationStructureBuildRangeInfoKHR`] is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureBuildRangeInfoKHR {
    uint32_t    primitiveCount;
    uint32_t    primitiveOffset;
    uint32_t    firstVertex;
    uint32_t    transformOffset;
} VkAccelerationStructureBuildRangeInfoKHR;
```

# Members
- [`primitive_count`] defines the number of primitives for a corresponding acceleration structure geometry.
- [`primitive_offset`] defines an offset in bytes into the memory where primitive data is defined.
- [`first_vertex`] is the index of the first vertex to build from for triangle geometry.
- [`transform_offset`] defines an offset in bytes into the memory where a transform matrix is defined.

# Description
The primitive count and primitive offset are interpreted differently
depending on the [`GeometryTypeKHR`] used:
- For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, [`primitive_count`] is the number of triangles to be built, where each triangle is treated as 3 vertices.  - If the geometry uses indices, [`primitive_count`] × 3 indices are consumed from [`AccelerationStructureGeometryTrianglesDataKHR::index_data`], starting at an offset of [`primitive_offset`]. The value of [`first_vertex`] is added to the index values before fetching vertices.  - If the geometry does not use indices, [`primitive_count`] × 3 vertices are consumed from [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`], starting at an offset of [`primitive_offset`] +  [`AccelerationStructureGeometryTrianglesDataKHR::vertex_stride`] × [`first_vertex`].  - If [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`] is not `NULL`, a single [`TransformMatrixKHR`] structure is consumed from [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`], at an offset of [`transform_offset`]. This matrix describes a transformation from the space in which the vertices for all triangles in this geometry are described to the space in which the acceleration structure is defined. 
- For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, [`primitive_count`] is the number of axis-aligned bounding boxes. [`primitive_count`][`AabbPositionsKHR`] structures are consumed from [`AccelerationStructureGeometryAabbsDataKHR::data`], starting at an offset of [`primitive_offset`].
- For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, [`primitive_count`] is the number of acceleration structures. [`primitive_count`][`AccelerationStructureInstanceKHR`] or [`AccelerationStructureMotionInstanceNV`] structures are consumed from [`AccelerationStructureGeometryInstancesDataKHR::data`], starting at an offset of [`primitive_offset`].

## Valid Usage
-    For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry uses indices, the offset [`primitive_offset`] from [`AccelerationStructureGeometryTrianglesDataKHR::index_data`] **must**  be a multiple of the element size of [`AccelerationStructureGeometryTrianglesDataKHR::index_type`]
-    For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry does not use indices, the offset [`primitive_offset`] from [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`] **must**  be a multiple of the component size of [`AccelerationStructureGeometryTrianglesDataKHR::vertex_format`]
-    For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the offset [`transform_offset`] from [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`] **must**  be a multiple of 16
-    For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, the offset [`primitive_offset`] from [`AccelerationStructureGeometryAabbsDataKHR::data`] **must**  be a multiple of 8
-    For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the offset [`primitive_offset`] from [`AccelerationStructureGeometryInstancesDataKHR::data`] **must**  be a multiple of 16

# Related
- [`khr_acceleration_structure`]
- [`build_acceleration_structures_khr`]
- [`cmd_build_acceleration_structures_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        