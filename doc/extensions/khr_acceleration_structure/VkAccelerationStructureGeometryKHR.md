[VkAccelerationStructureGeometryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html) - Structure specifying geometries to be built into an acceleration structure

# C Specifications
The [`AccelerationStructureGeometryKHR`] structure is defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureGeometryKHR {
    VkStructureType                           sType;
    const void*                               pNext;
    VkGeometryTypeKHR                         geometryType;
    VkAccelerationStructureGeometryDataKHR    geometry;
    VkGeometryFlagsKHR                        flags;
} VkAccelerationStructureGeometryKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`geometry_type`] describes which type of geometry this [`AccelerationStructureGeometryKHR`] refers to.
- [`geometry`] is a [`AccelerationStructureGeometryDataKHR`] union describing the geometry data for the relevant geometry type.
- [`flags`] is a bitmask of [`GeometryFlagBitsKHR`] values describing additional properties of how the geometry should be built.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`geometry_type`] **must**  be a valid [`GeometryTypeKHR`] value
-    If [`geometry_type`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the `triangles` member of [`geometry`] **must**  be a valid [`AccelerationStructureGeometryTrianglesDataKHR`] structure
-    If [`geometry_type`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the `aabbs` member of [`geometry`] **must**  be a valid [`AccelerationStructureGeometryAabbsDataKHR`] structure
-    If [`geometry_type`] is `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the `instances` member of [`geometry`] **must**  be a valid [`AccelerationStructureGeometryInstancesDataKHR`] structure
-  [`flags`] **must**  be a valid combination of [`GeometryFlagBitsKHR`] values

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureGeometryDataKHR`]
- [`GeometryFlagsKHR`]
- [`GeometryTypeKHR`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        