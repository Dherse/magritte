[VkGeometryTrianglesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTrianglesNV.html) - Structure specifying a triangle geometry in a bottom-level acceleration structure

# C Specifications
The [`GeometryTrianglesNV`] structure specifies triangle geometry in a
bottom-level acceleration structure and is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkGeometryTrianglesNV {
    VkStructureType    sType;
    const void*        pNext;
    VkBuffer           vertexData;
    VkDeviceSize       vertexOffset;
    uint32_t           vertexCount;
    VkDeviceSize       vertexStride;
    VkFormat           vertexFormat;
    VkBuffer           indexData;
    VkDeviceSize       indexOffset;
    uint32_t           indexCount;
    VkIndexType        indexType;
    VkBuffer           transformData;
    VkDeviceSize       transformOffset;
} VkGeometryTrianglesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`vertex_data`] is the buffer containing vertex data for this geometry.
- [`vertex_offset`] is the offset in bytes within [`vertex_data`] containing vertex data for this geometry.
- [`vertex_count`] is the number of valid vertices.
- [`vertex_stride`] is the stride in bytes between each vertex.
- [`vertex_format`] is a [`Format`] describing the format of each vertex element.
- [`index_data`] is the buffer containing index data for this geometry.
- [`index_offset`] is the offset in bytes within [`index_data`] containing index data for this geometry.
- [`index_count`] is the number of indices to include in this geometry.
- [`index_type`] is a [`IndexType`] describing the format of each index.
- [`transform_data`] is an optional buffer containing an [`TransformMatrixNV`] structure defining a transformation to be applied to this geometry.
- [`transform_offset`] is the offset in bytes in [`transform_data`] of the transform information described above.

# Description
If [`index_type`] is `VK_INDEX_TYPE_NONE_NV`, then this structure
describes a set of triangles determined by [`vertex_count`].
Otherwise, this structure describes a set of indexed triangles determined by
[`index_count`].
## Valid Usage
-  [`vertex_offset`] **must**  be less than the size of [`vertex_data`]
-  [`vertex_offset`] **must**  be a multiple of the component size of [`vertex_format`]
-  [`vertex_format`] **must**  be one of `VK_FORMAT_R32G32B32_SFLOAT`, `VK_FORMAT_R32G32_SFLOAT`, `VK_FORMAT_R16G16B16_SFLOAT`, `VK_FORMAT_R16G16_SFLOAT`, `VK_FORMAT_R16G16_SNORM`, or `VK_FORMAT_R16G16B16_SNORM`
-  [`vertex_stride`] **must**  be less than or equal to 2<sup>32</sup>-1
-  [`index_offset`] **must**  be less than the size of [`index_data`]
-  [`index_offset`] **must**  be a multiple of the element size of [`index_type`]
-  [`index_type`] **must**  be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or `VK_INDEX_TYPE_NONE_NV`
-  [`index_data`] **must**  be [`crate::Handle::null`] if [`index_type`] is `VK_INDEX_TYPE_NONE_NV`
-  [`index_data`] **must**  be a valid [`Buffer`] handle if [`index_type`] is not `VK_INDEX_TYPE_NONE_NV`
-  [`index_count`] **must**  be `0` if [`index_type`] is `VK_INDEX_TYPE_NONE_NV`
-  [`transform_offset`] **must**  be less than the size of [`transform_data`]
-  [`transform_offset`] **must**  be a multiple of `16`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_TRIANGLES_NV`
-  [`p_next`] **must**  be `NULL`
-    If [`vertex_data`] is not [`crate::Handle::null`], [`vertex_data`] **must**  be a valid [`Buffer`] handle
-  [`vertex_format`] **must**  be a valid [`Format`] value
-    If [`index_data`] is not [`crate::Handle::null`], [`index_data`] **must**  be a valid [`Buffer`] handle
-  [`index_type`] **must**  be a valid [`IndexType`] value
-    If [`transform_data`] is not [`crate::Handle::null`], [`transform_data`] **must**  be a valid [`Buffer`] handle
-    Each of [`index_data`], [`transform_data`], and [`vertex_data`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`VK_NV_ray_tracing`]
- [`Buffer`]
- [`DeviceSize`]
- [`Format`]
- [`GeometryDataNV`]
- [`IndexType`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        