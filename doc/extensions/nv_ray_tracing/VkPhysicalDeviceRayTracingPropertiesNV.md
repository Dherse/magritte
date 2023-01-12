[VkPhysicalDeviceRayTracingPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceRayTracingPropertiesNV.html) - Properties of the physical device for ray tracing

# C Specifications
The [`PhysicalDeviceRayTracingPropertiesNV`] structure is defined as:
```c
// Provided by VK_NV_ray_tracing
typedef struct VkPhysicalDeviceRayTracingPropertiesNV {
    VkStructureType    sType;
    void*              pNext;
    uint32_t           shaderGroupHandleSize;
    uint32_t           maxRecursionDepth;
    uint32_t           maxShaderGroupStride;
    uint32_t           shaderGroupBaseAlignment;
    uint64_t           maxGeometryCount;
    uint64_t           maxInstanceCount;
    uint64_t           maxTriangleCount;
    uint32_t           maxDescriptorSetAccelerationStructures;
} VkPhysicalDeviceRayTracingPropertiesNV;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`shader_group_handle_size`] is the size in bytes of the shader header.
- [`max_recursion_depth`] is the maximum number of levels of recursion allowed in a trace command.
- [`max_shader_group_stride`] is the maximum stride in bytes allowed between shader groups in the shader binding table.
- [`shader_group_base_alignment`] is the  **required**  alignment in bytes for the base of the shader binding table.
- [`max_geometry_count`] is the maximum number of geometries in the bottom level acceleration structure.
- [`max_instance_count`] is the maximum number of instances in the top level acceleration structure.
- [`max_triangle_count`] is the maximum number of triangles in all geometries in the bottom level acceleration structure.
- [`max_descriptor_set_acceleration_structures`] is the maximum number of acceleration structure descriptors that are allowed in a descriptor set.

# Description
Due to the fact that the geometry, instance, and triangle counts are
specified at acceleration structure creation as 32-bit values,
[`max_geometry_count`], [`max_instance_count`], and [`max_triangle_count`] **must**  not exceed 2<sup>32</sup>-1.If the [`PhysicalDeviceRayTracingPropertiesNV`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.Limits specified by this structure  **must**  match those specified with the same
name in [`PhysicalDeviceAccelerationStructurePropertiesKHR`] and
[`PhysicalDeviceRayTracingPipelinePropertiesKHR`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RAY_TRACING_PROPERTIES_NV`

# Related
- [`nv_ray_tracing`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        