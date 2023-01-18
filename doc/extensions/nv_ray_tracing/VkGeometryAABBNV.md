[VkGeometryAABBNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryAABBNV.html) - Structure specifying axis-aligned bounding box geometry in a bottom-level acceleration structure

# C Specifications
The [`GeometryAabbNV`] structure specifies axis-aligned bounding box
geometry in a bottom-level acceleration structure, and is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkGeometryAABBNV {
    VkStructureType    sType;
    const void*        pNext;
    VkBuffer           aabbData;
    uint32_t           numAABBs;
    uint32_t           stride;
    VkDeviceSize       offset;
} VkGeometryAABBNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`aabb_data`] is the buffer containing axis-aligned bounding box data.
- [`num_aab_bs`] is the number of AABBs in this geometry.
- [`stride`] is the stride in bytes between AABBs in [`aabb_data`].
- [`offset`] is the offset in bytes of the first AABB in [`aabb_data`].

# Description
The AABB data in memory is six 32-bit floats consisting of the minimum x, y,
and z values followed by the maximum x, y, and z values.
## Valid Usage
-  [`offset`] **must**  be less than the size of [`aabb_data`]
-  [`offset`] **must**  be a multiple of `8`
-  [`stride`] **must**  be a multiple of `8`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_GEOMETRY_AABB_NV`
-  [`p_next`] **must**  be `NULL`
-    If [`aabb_data`] is not [`crate::Handle::null`], [`aabb_data`] **must**  be a valid [`Buffer`] handle

# Related
- [`VK_NV_ray_tracing`]
- [`Buffer`]
- [`DeviceSize`]
- [`GeometryDataNV`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        