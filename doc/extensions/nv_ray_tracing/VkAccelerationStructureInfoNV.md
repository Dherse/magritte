[VkAccelerationStructureInfoNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInfoNV.html) - Structure specifying the parameters of acceleration structure object

# C Specifications
The [`AccelerationStructureInfoNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkAccelerationStructureInfoNV {
    VkStructureType                        sType;
    const void*                            pNext;
    VkAccelerationStructureTypeNV          type;
    VkBuildAccelerationStructureFlagsNV    flags;
    uint32_t                               instanceCount;
    uint32_t                               geometryCount;
    const VkGeometryNV*                    pGeometries;
} VkAccelerationStructureInfoNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] is a [`AccelerationStructureTypeNV`] value specifying the type of acceleration structure that will be created.
- [`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsNV`] specifying additional parameters of the acceleration structure.
- [`instance_count`] specifies the number of instances that will be in the new acceleration structure.
- [`geometry_count`] specifies the number of geometries that will be in the new acceleration structure.
- [`geometries`] is a pointer to an array of [`geometry_count`][`GeometryNV`] structures containing the scene data being passed into the acceleration structure.

# Description
[`AccelerationStructureInfoNV`] contains information that is used both
for acceleration structure creation with
[`create_acceleration_structure_nv`] and in combination with the actual
geometric data to build the acceleration structure with
[`cmd_build_acceleration_structure_nv`].
## Valid Usage
-  [`geometry_count`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
-  [`instance_count`] **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_instance_count`]
-    The total number of triangles in all geometries  **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_triangle_count`]
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV` then [`geometry_count`] **must**  be `0`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then [`instance_count`] **must**  be `0`
-    If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV` then the `geometryType` member of each geometry in [`geometries`] **must**  be the same
-  [`type_`] **must**  not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
-    If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV` bit set, then it  **must**  not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV` bit set
-  `scratch` **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
-    If `instanceData` is not [`crate::Handle::null`], `instanceData` **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_INFO_NV`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`AccelerationStructureTypeNV`] value
-  [`flags`] **must**  be a valid combination of [`BuildAccelerationStructureFlagBitsNV`] values
-    If [`geometry_count`] is not `0`, [`geometries`] **must**  be a valid pointer to an array of [`geometry_count`] valid [`GeometryNV`] structures

# Related
- [`nv_ray_tracing`]
- [`AccelerationStructureCreateInfoNV`]
- [`AccelerationStructureTypeNV`]
- [`BuildAccelerationStructureFlagsNV`]
- [`GeometryNV`]
- [`StructureType`]
- [`cmd_build_acceleration_structure_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        