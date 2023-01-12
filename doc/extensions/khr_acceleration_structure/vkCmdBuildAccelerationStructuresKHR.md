[vkCmdBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBuildAccelerationStructuresKHR.html) - Build an acceleration structure

# C Specifications
To build acceleration structures call:
```c
// Provided by VK_KHR_acceleration_structure
void vkCmdBuildAccelerationStructuresKHR(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    infoCount,
    const VkAccelerationStructureBuildGeometryInfoKHR* pInfos,
    const VkAccelerationStructureBuildRangeInfoKHR* const* ppBuildRangeInfos);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`info_count`] is the number of acceleration structures to build. It specifies the number of the [`p_infos`] structures and [`pp_build_range_infos`] pointers that  **must**  be provided.
- [`p_infos`] is a pointer to an array of [`info_count`][`AccelerationStructureBuildGeometryInfoKHR`] structures defining the geometry used to build each acceleration structure.
- [`pp_build_range_infos`] is a pointer to an array of [`info_count`] pointers to arrays of [`AccelerationStructureBuildRangeInfoKHR`] structures. Each [`pp_build_range_infos`][i] is a pointer to an array of [`p_infos`][i].`geometryCount`[`AccelerationStructureBuildRangeInfoKHR`] structures defining dynamic offsets to the addresses where geometry data is stored, as defined by [`p_infos`][i].

# Description
The [`cmd_build_acceleration_structures_khr`] command provides the ability
to initiate multiple acceleration structures builds, however there is no
ordering or synchronization implied between any of the individual
acceleration structure builds.Accesses to the acceleration structure scratch buffers as identified by the
[`AccelerationStructureBuildGeometryInfoKHR::scratch_data`] buffer
device addresses  **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with
the `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR` or
`VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`.
Similarly for accesses to each
[`AccelerationStructureBuildGeometryInfoKHR::src_acceleration_structure`]
and
[`AccelerationStructureBuildGeometryInfoKHR::dst_acceleration_structure`].Accesses to other input buffers as identified by any used values of
[`AccelerationStructureGeometryMotionTrianglesDataNV::vertex_data`],
[`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`],
[`AccelerationStructureGeometryTrianglesDataKHR::index_data`],
[`AccelerationStructureGeometryTrianglesDataKHR::transform_data`],
[`AccelerationStructureGeometryAabbsDataKHR::data`], and
[`AccelerationStructureGeometryInstancesDataKHR::data`] **must**  be
[synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_SHADER_READ_BIT`.
## Valid Usage
-    The `mode` member of each element of [`p_infos`] **must**  be a valid [`BuildAccelerationStructureModeKHR`] value
-    If the `srcAccelerationStructure` member of any element of [`p_infos`] is not [`crate::Handle::null`], the `srcAccelerationStructure` member  **must**  be a valid [`AccelerationStructureKHR`] handle
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `srcAccelerationStructure` member  **must**  not be [`crate::Handle::null`]
-    The `srcAccelerationStructure` member of any element of [`p_infos`] **must**  not be the same acceleration structure as the `dstAccelerationStructure` member of any other element of [`p_infos`]
-    The `dstAccelerationStructure` member of any element of [`p_infos`] **must**  not be the same acceleration structure as the `dstAccelerationStructure` member of any other element of [`p_infos`]
-    The `dstAccelerationStructure` member of any element of [`p_infos`] **must**  be a valid [`AccelerationStructureKHR`] handle
-    For each element of [`p_infos`], if its `type` member is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, its `dstAccelerationStructure` member  **must**  have been created with a value of [`AccelerationStructureCreateInfoKHR::type_`] equal to either `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR` or `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
-    For each element of [`p_infos`], if its `type` member is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR`, its `dstAccelerationStructure` member  **must**  have been created with a value of [`AccelerationStructureCreateInfoKHR::type_`] equal to either `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` or `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, [inactive primitives](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#acceleration-structure-inactive-prims) in its `srcAccelerationStructure` member  **must**  not be made active
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, active primitives in its `srcAccelerationStructure` member  **must**  not be made [inactive](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
-    The `dstAccelerationStructure` member of any element of [`p_infos`] **must**  not be referenced by the `geometry.instances.data` member of any element of `pGeometries` or `ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` in any other element of [`p_infos`]
-    The range of memory backing the `dstAccelerationStructure` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing the `srcAccelerationStructure` member of any other element of [`p_infos`] with a `mode` equal to `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, which is accessed by this command
-    The range of memory backing the `dstAccelerationStructure` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing the `dstAccelerationStructure` member of any other element of [`p_infos`], which is accessed by this command
-    The range of memory backing the `dstAccelerationStructure` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing the `scratchData` member of any element of [`p_infos`] (including the same element), which is accessed by this command
-    The range of memory backing the `scratchData` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing the `scratchData` member of any other element of [`p_infos`], which is accessed by this command
-    The range of memory backing the `scratchData` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing the `srcAccelerationStructure` member of any element of [`p_infos`] with a `mode` equal to `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` (including the same element), which is accessed by this command
-    The range of memory backing the `dstAccelerationStructure` member of any element of [`p_infos`] that is accessed by this command  **must**  not overlap the memory backing any acceleration structure referenced by the `geometry.instances.data` member of any element of `pGeometries` or `ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` in any other element of [`p_infos`], which is accessed by this command
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `srcAccelerationStructure` member  **must**  have previously been constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR` set in [`AccelerationStructureBuildGeometryInfoKHR::flags`] in the build
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `srcAccelerationStructure` and `dstAccelerationStructure` members  **must**  either be the same [`AccelerationStructureKHR`], or not have any [memory aliasing](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/html/vkspec.html#resources-memory-aliasing)
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `geometryCount` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `flags` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, its `type` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, its `geometryType` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, its `flags` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, its `geometry.triangles.vertexFormat` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, its `geometry.triangles.maxVertex` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, its `geometry.triangles.indexType` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if its `geometry.triangles.transformData` address was `NULL` when `srcAccelerationStructure` was last built, then it  **must**  be `NULL`
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if its `geometry.triangles.transformData` address was not `NULL` when `srcAccelerationStructure` was last built, then it  **must**  not be `NULL`
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, then for each [`AccelerationStructureGeometryKHR`] structure referred to by its `pGeometries` or `ppGeometries` members, if `geometryType` is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, and `geometry.triangles.indexType` is not `VK_INDEX_TYPE_NONE_KHR`, then the value of each index referenced  **must**  be the same as the corresponding index value when `srcAccelerationStructure` was last built
-    For each [`AccelerationStructureBuildRangeInfoKHR`] referenced by this command, its `primitiveCount` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each [`AccelerationStructureBuildRangeInfoKHR`] referenced by this command, if the corresponding geometry uses indices, its `firstVertex` member  **must**  have the same value which was specified when `srcAccelerationStructure` was last built
-    For each element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the corresponding [`pp_build_range_infos`][i][j].`primitiveCount` **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_instance_count`]

-    For each element of [`p_infos`], the `buffer` used to create its `dstAccelerationStructure` member  **must**  be bound to device memory
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` the `buffer` used to create its `srcAccelerationStructure` member  **must**  be bound to device memory
-    For each element of [`p_infos`], the `buffer` used to create each acceleration structure referenced by the `geometry.instances.data` member of any element of `pGeometries` or `ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` **must**  be bound to device memory
-    If [`p_infos`][i].`mode` is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR`, all addresses between [`p_infos`][i].`scratchData.deviceAddress` and [`p_infos`][i].`scratchData.deviceAddress` +  N - 1  **must**  be in the buffer device address range of the same buffer, where N is given by the `buildScratchSize` member of the [`AccelerationStructureBuildSizesInfoKHR`] structure returned from a call to [`get_acceleration_structure_build_sizes_khr`] with an identical [`AccelerationStructureBuildGeometryInfoKHR`] structure and primitive count
-    If [`p_infos`][i].`mode` is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, all addresses between [`p_infos`][i].`scratchData.deviceAddress` and [`p_infos`][i].`scratchData.deviceAddress` +  N - 1  **must**  be in the buffer device address range of the same buffer, where N is given by the `updateScratchSize` member of the [`AccelerationStructureBuildSizesInfoKHR`] structure returned from a call to [`get_acceleration_structure_build_sizes_khr`] with an identical [`AccelerationStructureBuildGeometryInfoKHR`] structure and primitive count
-    The buffers from which the buffer device addresses for all of the `geometry.triangles.vertexData`, `geometry.triangles.indexData`, `geometry.triangles.transformData`, `geometry.aabbs.data`, and `geometry.instances.data` members of all [`p_infos`][i].`pGeometries` and [`p_infos`][i].`ppGeometries` are queried  **must**  have been created with the `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR` usage flag
-    The buffer from which the buffer device address [`p_infos`][i].`scratchData.deviceAddress` is queried  **must**  have been created with `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` usage flag
-    For each element of [`p_infos`], its `scratchData.deviceAddress` member  **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For each element of [`p_infos`], if `scratchData.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For each element of [`p_infos`], its `scratchData.deviceAddress` member  **must**  be a multiple of [`PhysicalDeviceAccelerationStructurePropertiesKHR::min_acceleration_structure_scratch_offset_alignment`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, `geometry.triangles.vertexData.deviceAddress` **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.vertexData.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, `geometry.triangles.vertexData.deviceAddress` **must**  be aligned to the size in bytes of the smallest component of the format in `vertexFormat`
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.indexType` is not `VK_INDEX_TYPE_NONE_KHR`, `geometry.triangles.indexData.deviceAddress` **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.indexType` is not `VK_INDEX_TYPE_NONE_KHR`, if `geometry.triangles.indexData.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, and with `geometry.triangles.indexType` not equal to `VK_INDEX_TYPE_NONE_KHR`, `geometry.triangles.indexData.deviceAddress` **must**  be aligned to the size in bytes of the type in `indexType`
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.transformData.deviceAddress` is not `0`, it  **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.transformData.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.transformData.deviceAddress` is not `0`, it  **must**  be aligned to `16` bytes
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR`, `geometry.aabbs.data.deviceAddress` **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR`, if `geometry.aabbs.data.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR`, `geometry.aabbs.data.deviceAddress` **must**  be aligned to `8` bytes
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, if `geometry.arrayOfPointers` is `VK_FALSE`, `geometry.instances.data.deviceAddress` **must**  be aligned to `16` bytes
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, if `geometry.arrayOfPointers` is `VK_TRUE`, `geometry.instances.data.deviceAddress` **must**  be aligned to `8` bytes
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, if `geometry.arrayOfPointers` is `VK_TRUE`, each element of `geometry.instances.data.deviceAddress` in device memory  **must**  be aligned to `16` bytes
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, `geometry.instances.data.deviceAddress` **must**  be a valid device address obtained from [`get_buffer_device_address`]
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, if `geometry.instances.data.deviceAddress` is the address of a non-sparse buffer then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, each [`AccelerationStructureInstanceKHR::acceleration_structure_reference`] value in `geometry.instances.data.deviceAddress` **must**  be a valid device address containing a value obtained from [`get_acceleration_structure_device_address_khr`]

-    For each [`p_infos`][i], `dstAccelerationStructure` **must**  have been created with a value of [`AccelerationStructureCreateInfoKHR::size`] greater than or equal to the memory size required by the build operation, as returned by [`get_acceleration_structure_build_sizes_khr`] with `pBuildInfo` = [`p_infos`][i] and with each element of the `pMaxPrimitiveCounts` array greater than or equal to the equivalent [`pp_build_range_infos`][i][j].`primitiveCount` values for `j` in [0,[`p_infos`][i].`geometryCount`)
-    Each element of [`pp_build_range_infos`][i]  **must**  be a valid pointer to an array of [`p_infos`][i].`geometryCount`[`AccelerationStructureBuildRangeInfoKHR`] structures

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_infos`] **must**  be a valid pointer to an array of [`info_count`] valid [`AccelerationStructureBuildGeometryInfoKHR`] structures
-  [`pp_build_range_infos`] **must**  be a valid pointer to an array of [`info_count`][`AccelerationStructureBuildRangeInfoKHR`] structures
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`info_count`] **must**  be greater than `0`

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureBuildRangeInfoKHR`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        