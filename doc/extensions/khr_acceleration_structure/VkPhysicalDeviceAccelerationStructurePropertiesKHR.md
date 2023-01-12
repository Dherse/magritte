[VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) - Properties of the physical device for acceleration structure

# C Specifications
The [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is
defined as:
```c
// Provided by VK_KHR_acceleration_structure
typedef struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    VkStructureType    sType;
    void*              pNext;
    uint64_t           maxGeometryCount;
    uint64_t           maxInstanceCount;
    uint64_t           maxPrimitiveCount;
    uint32_t           maxPerStageDescriptorAccelerationStructures;
    uint32_t           maxPerStageDescriptorUpdateAfterBindAccelerationStructures;
    uint32_t           maxDescriptorSetAccelerationStructures;
    uint32_t           maxDescriptorSetUpdateAfterBindAccelerationStructures;
    uint32_t           minAccelerationStructureScratchOffsetAlignment;
} VkPhysicalDeviceAccelerationStructurePropertiesKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`max_geometry_count`] is the maximum number of geometries in the bottom level acceleration structure.
- [`max_instance_count`] is the maximum number of instances in the top level acceleration structure.
- [`max_primitive_count`] is the maximum number of triangles or AABBs in all geometries in the bottom level acceleration structure.
- [`max_per_stage_descriptor_acceleration_structures`] is the maximum number of acceleration structure bindings that  **can**  be accessible to a single shader stage in a pipeline layout. Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this limit. Only descriptor bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
- [`max_per_stage_descriptor_update_after_bind_acceleration_structures`] is similar to [`max_per_stage_descriptor_acceleration_structures`] but counts descriptor bindings from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`max_descriptor_set_acceleration_structures`] is the maximum number of acceleration structure descriptors that  **can**  be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this limit. Only descriptor bindings in descriptor set layouts created without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
- [`max_descriptor_set_update_after_bind_acceleration_structures`] is similar to [`max_descriptor_set_acceleration_structures`] but counts descriptor bindings from descriptor sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
- [`min_acceleration_structure_scratch_offset_alignment`] is the minimum  **required**  alignment, in bytes, for scratch data passed in to an acceleration structure build command. The value  **must**  be a power of two.

# Description
Due to the fact that the geometry, instance, and primitive counts are
specified at acceleration structure creation as 32-bit values,
[[`max_geometry_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxGeometryCount),
[[`max_instance_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInstanceCount), and
[[`max_primitive_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxPrimitiveCount) **must**  not exceed
2<sup>32</sup>-1.If the [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is included in the [`p_next`] chain of the
[`PhysicalDeviceProperties2`] structure passed to
[`get_physical_device_properties2`], it is filled in with each
corresponding implementation-dependent property.Limits specified by this structure  **must**  match those specified with the same
name in [`PhysicalDeviceRayTracingPropertiesNV`].
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`

# Related
- [`khr_acceleration_structure`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        