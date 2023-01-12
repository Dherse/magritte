[VkAccelerationStructureBuildGeometryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) - Structure specifying the geometry data used to build an acceleration structure

# C Specifications
The [`AccelerationStructureBuildGeometryInfoKHR`] structure is defined
as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkAccelerationStructureBuildGeometryInfoKHR {
    VkStructureType                                     sType;
    const void*                                         pNext;
    VkAccelerationStructureTypeKHR                      type;
    VkBuildAccelerationStructureFlagsKHR                flags;
    VkBuildAccelerationStructureModeKHR                 mode;
    VkAccelerationStructureKHR                          srcAccelerationStructure;
    VkAccelerationStructureKHR                          dstAccelerationStructure;
    uint32_t                                            geometryCount;
    const VkAccelerationStructureGeometryKHR*           pGeometries;
    const VkAccelerationStructureGeometryKHR* const*    ppGeometries;
    VkDeviceOrHostAddressKHR                            scratchData;
} VkAccelerationStructureBuildGeometryInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration structure being built.
- [`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsKHR`] specifying additional parameters of the acceleration structure.
- [`mode`] is a [`BuildAccelerationStructureModeKHR`] value specifying the type of operation to perform.
- [`src_acceleration_structure`] is a pointer to an existing acceleration structure that is to be used to update the `dst` acceleration structure when [`mode`] is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`.
- [`dst_acceleration_structure`] is a pointer to the target acceleration structure for the build.
- [`geometry_count`] specifies the number of geometries that will be built into [`dst_acceleration_structure`].
- [`geometries`] is a pointer to an array of [`AccelerationStructureGeometryKHR`] structures.
- [`pp_geometries`] is a pointer to an array of pointers to [`AccelerationStructureGeometryKHR`] structures.
- [`scratch_data`] is the device or host address to memory that will be used as scratch memory for the build.

# Description
Only one of [`geometries`] or [`pp_geometries`] **can**  be a valid pointer,
the other  **must**  be `NULL`.
Each element of the non-`NULL` array describes the data used to build each
acceleration structure geometry.The index of each element of the [`geometries`] or [`pp_geometries`]
members of [`AccelerationStructureBuildGeometryInfoKHR`] is used as the
*geometry index* during ray traversal.
The geometry index is available in ray shaders via the
[`RayGeometryIndexKHR`
built-in](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-builtin-variables-raygeometryindex), and is [used to
determine hit and intersection shaders executed during traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table-hit-shader-indexing).
The geometry index is available to ray queries via the
`OpRayQueryGetIntersectionGeometryIndexKHR` instruction.Setting `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`]
indicates that this build is a motion top level acceleration structure.
A motion top level uses instances of format
[`AccelerationStructureMotionInstanceNV`] if
[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
is `VK_FALSE`.If
[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
is `VK_TRUE`, the pointer for any given element of the array of instance
pointers consists of 4 bits of
[`AccelerationStructureMotionInstanceTypeNV`] in the low 4 bits of the
pointer identifying the type of structure at the pointer.
The device address accessed is the value in the array with the low 4 bits
set to zero.
The structure at the pointer is one of
[`AccelerationStructureInstanceKHR`],
[`AccelerationStructureMatrixMotionInstanceNV`] or
[`AccelerationStructureSrtMotionInstanceNV`], depending on the type
value encoded in the low 4 bits.A top level acceleration structure with either motion instances or vertex
motion in its instances  **must**  set
`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`].Members [`src_acceleration_structure`] and [`dst_acceleration_structure`] **may**  be the same or different for an update operation (when [`mode`] is
`VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`).
If they are the same, the update happens in-place.
Otherwise, the target acceleration structure is updated and the source is
not modified.
## Valid Usage
-  [`type_`] **must**  not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
-    Only one of [`geometries`] or [`pp_geometries`] **can**  be a valid pointer, the other  **must**  be `NULL`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, the `geometryType` member of elements of either [`geometries`] or [`pp_geometries`] **must**  be `VK_GEOMETRY_TYPE_INSTANCES_KHR`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, [`geometry_count`] **must**  be `1`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` the `geometryType` member of elements of either [`geometries`] or [`pp_geometries`] **must**  not be `VK_GEOMETRY_TYPE_INSTANCES_KHR`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then the `geometryType` member of each geometry in either [`geometries`] or [`pp_geometries`] **must**  be the same
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then [`geometry_count`] **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_geometry_count`]
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType` member of either [`geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the total number of AABBs in all geometries  **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType` member of either [`geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the total number of triangles in all geometries  **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
-    If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR` bit set, then it  **must**  not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR` bit set
-    If [`dst_acceleration_structure`] was created with `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in [`AccelerationStructureCreateInfoKHR`]::[`flags`], `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` **must**  be set in [`flags`]
-    If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`], [`dst_acceleration_structure`] **must**  have been created with `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in [`AccelerationStructureCreateInfoKHR`]::[`flags`]
-    If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`], [`type_`] **must**  not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`AccelerationStructureTypeKHR`] value
-  [`flags`] **must**  be a valid combination of [`BuildAccelerationStructureFlagBitsKHR`] values
-    If [`geometry_count`] is not `0`, and [`geometries`] is not `NULL`, [`geometries`] **must**  be a valid pointer to an array of [`geometry_count`] valid [`AccelerationStructureGeometryKHR`] structures
-    If [`geometry_count`] is not `0`, and [`pp_geometries`] is not `NULL`, [`pp_geometries`] **must**  be a valid pointer to an array of [`geometry_count`] valid pointers to valid [`AccelerationStructureGeometryKHR`] structures
-    Both of [`dst_acceleration_structure`], and [`src_acceleration_structure`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureGeometryKHR`]
- [`AccelerationStructureKHR`]
- [`AccelerationStructureTypeKHR`]
- [VkBuildAccelerationStructureFlagsKHR]()
- [`BuildAccelerationStructureModeKHR`]
- [`DeviceOrHostAddressKHR`]
- [`StructureType`]
- [`build_acceleration_structures_khr`]
- [`cmd_build_acceleration_structures_indirect_khr`]
- [`cmd_build_acceleration_structures_khr`]
- [`get_acceleration_structure_build_sizes_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        