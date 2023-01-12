[VkAccelerationStructureGeometryTrianglesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) - Structure specifying a triangle geometry in a bottom-level acceleration structure

# C Specifications
The [`AccelerationStructureGeometryTrianglesDataKHR`] structure is
defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureGeometryTrianglesDataKHR {
    VkStructureType                  sType;
    const void*                      pNext;
    VkFormat                         vertexFormat;
    VkDeviceOrHostAddressConstKHR    vertexData;
    VkDeviceSize                     vertexStride;
    uint32_t                         maxVertex;
    VkIndexType                      indexType;
    VkDeviceOrHostAddressConstKHR    indexData;
    VkDeviceOrHostAddressConstKHR    transformData;
} VkAccelerationStructureGeometryTrianglesDataKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`vertex_format`] is the [`Format`] of each vertex element.
- [`vertex_data`] is a device or host address to memory containing vertex data for this geometry.
- [`max_vertex`] is the highest index of a vertex that will be addressed by a build command using this structure.
- [`vertex_stride`] is the stride in bytes between each vertex.
- [`index_type`] is the [`IndexType`] of each index element.
- [`index_data`] is a device or host address to memory containing index data for this geometry.
- [`transform_data`] is a device or host address to memory containing an optional reference to a [`TransformMatrixKHR`] structure describing a transformation from the space in which the vertices in this geometry are described to the space in which the acceleration structure is defined.

# Description
## Valid Usage
-  [`vertex_stride`] **must**  be a multiple of the size in bytes of the smallest component of [`vertex_format`]
-  [`vertex_stride`] **must**  be less than or equal to 2<sup>32</sup>-1
-  [`vertex_format`] **must**  support the `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR` in [`FormatProperties::buffer_features`] as returned by [`get_physical_device_format_properties2`]
-  [`index_type`] **must**  be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or `VK_INDEX_TYPE_NONE_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`
-  [`p_next`] **must**  be `NULL` or a pointer to a valid instance of [`AccelerationStructureGeometryMotionTrianglesDataNV`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`vertex_format`] **must**  be a valid [`Format`] value
-  [`index_type`] **must**  be a valid [`IndexType`] value

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureGeometryDataKHR`]
- [`DeviceOrHostAddressConstKHR`]
- [`DeviceSize`]
- [`Format`]
- [`IndexType`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        