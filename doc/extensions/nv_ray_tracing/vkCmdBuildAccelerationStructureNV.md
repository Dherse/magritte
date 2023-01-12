[vkCmdBuildAccelerationStructureNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructureNV.html) - Build an acceleration structure

# C Specifications
To build an acceleration structure call:
```c
// Provided by VK_NV_ray_tracing
void vkCmdBuildAccelerationStructureNV(
    VkCommandBuffer                             commandBuffer,
    const VkAccelerationStructureInfoNV*        pInfo,
    VkBuffer                                    instanceData,
    VkDeviceSize                                instanceOffset,
    VkBool32                                    update,
    VkAccelerationStructureNV                   dst,
    VkAccelerationStructureNV                   src,
    VkBuffer                                    scratch,
    VkDeviceSize                                scratchOffset);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`p_info`] contains the shared information for the acceleration structure’s structure.
- [`instance_data`] is the buffer containing an array of [`AccelerationStructureInstanceKHR`] structures defining acceleration structures. This parameter  **must**  be `NULL` for bottom level acceleration structures.
- [`instance_offset`] is the offset in bytes (relative to the start of [`instance_data`]) at which the instance data is located.
- [`update`] specifies whether to update the [`dst`] acceleration structure with the data in [`src`].
- [`dst`] is a pointer to the target acceleration structure for the build.
- [`src`] is a pointer to an existing acceleration structure that is to be used to update the [`dst`] acceleration structure.
- [`scratch`] is the [`Buffer`] that will be used as scratch memory for the build.
- [`scratch_offset`] is the offset in bytes relative to the start of [`scratch`] that will be used as a scratch memory.

# Description
Accesses to [`dst`], [`src`], and [`scratch`] **must**  be
[synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`.
## Valid Usage
-  `geometryCount` **must**  be less than or equal to [`PhysicalDeviceRayTracingPropertiesNV::max_geometry_count`]
-  [`dst`] **must**  have been created with compatible [`AccelerationStructureInfoNV`] where [`AccelerationStructureInfoNV::type_`] and [`AccelerationStructureInfoNV::flags`] are identical, [`AccelerationStructureInfoNV::instance_count`] and [`AccelerationStructureInfoNV::geometry_count`] for [`dst`] are greater than or equal to the build size and each geometry in [`AccelerationStructureInfoNV::geometries`] for [`dst`] has greater than or equal to the number of vertices, indices, and AABBs
-    If [`update`] is `VK_TRUE`, [`src`] **must**  not be [`crate::Handle::null`]
-    If [`update`] is `VK_TRUE`, [`src`] **must**  have previously been constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV` set in [`AccelerationStructureInfoNV::flags`] in the original build
-    If [`update`] is `VK_FALSE`, the `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`] and [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV` **must**  be less than or equal to the size of [`scratch`] minus [`scratch_offset`]
-    If [`update`] is `VK_TRUE`, the `size` member of the [`MemoryRequirements`] structure returned from a call to [`get_acceleration_structure_memory_requirements_nv`] with [`AccelerationStructureMemoryRequirementsInfoNV::acceleration_structure`] set to [`dst`] and [`AccelerationStructureMemoryRequirementsInfoNV::type_`] set to `VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV` **must**  be less than or equal to the size of [`scratch`] minus [`scratch_offset`]
-  [`scratch`] **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
-    If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  have been created with `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` usage flag
-    Each [`AccelerationStructureInstanceKHR::acceleration_structure_reference`] value in [`instance_data`] **must**  be a valid device address containing a value obtained from [`get_acceleration_structure_handle_nv`]
-    If [`update`] is `VK_TRUE`, then objects that were previously active  **must**  not be made inactive as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
-    If [`update`] is `VK_TRUE`, then objects that were previously inactive  **must**  not be made active as per [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
-    If [`update`] is `VK_TRUE`, the [`src`] and [`dst`] objects  **must**  either be the same object or not have any [memory aliasing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-memory-aliasing)

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_info`] **must**  be a valid pointer to a valid [`AccelerationStructureInfoNV`] structure
-    If [`instance_data`] is not [`crate::Handle::null`], [`instance_data`] **must**  be a valid [`Buffer`] handle
-  [`dst`] **must**  be a valid [`AccelerationStructureNV`] handle
-    If [`src`] is not [`crate::Handle::null`], [`src`] **must**  be a valid [`AccelerationStructureNV`] handle
-  [`scratch`] **must**  be a valid [`Buffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Each of [`command_buffer`], [`dst`], [`instance_data`], [`scratch`], and [`src`] that are valid handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_ray_tracing`]
- [`AccelerationStructureInfoNV`]
- [`AccelerationStructureNV`]
- [`Bool32`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        