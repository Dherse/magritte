[vkBuildAccelerationStructuresKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBuildAccelerationStructuresKHR.html) - Build an acceleration structure on the host

# C Specifications
To build acceleration structures on the host, call:
```c
// Provided by VK_KHR_acceleration_structure
VkResult vkBuildAccelerationStructuresKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      deferredOperation,
    uint32_t                                    infoCount,
    const VkAccelerationStructureBuildGeometryInfoKHR* pInfos,
    const VkAccelerationStructureBuildRangeInfoKHR* const* ppBuildRangeInfos);
```

# Parameters
- [`device`] is the [`Device`] for which the acceleration structures are being built.
- [`deferred_operation`] is an optional [`DeferredOperationKHR`] to [request deferral](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#deferred-host-operations-requesting) for this command.
- [`info_count`] is the number of acceleration structures to build. It specifies the number of the [`p_infos`] structures and [`pp_build_range_infos`] pointers that  **must**  be provided.
- [`p_infos`] is a pointer to an array of [`info_count`][`AccelerationStructureBuildGeometryInfoKHR`] structures defining the geometry used to build each acceleration structure.
- [`pp_build_range_infos`] is a pointer to an array of [`info_count`] pointers to arrays of [`AccelerationStructureBuildRangeInfoKHR`] structures. Each [`pp_build_range_infos`][i] is a pointer to an array of [`p_infos`][i].`geometryCount`[`AccelerationStructureBuildRangeInfoKHR`] structures defining dynamic offsets to the addresses where geometry data is stored, as defined by [`p_infos`][i].

# Description
This command fulfills the same task as
[`cmd_build_acceleration_structures_khr`] but is executed by the host.The [`build_acceleration_structures_khr`] command provides the ability to
initiate multiple acceleration structures builds, however there is no
ordering or synchronization implied between any of the individual
acceleration structure builds.
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

-    For each [`p_infos`][i], `dstAccelerationStructure` **must**  have been created with a value of [`AccelerationStructureCreateInfoKHR::size`] greater than or equal to the memory size required by the build operation, as returned by [`get_acceleration_structure_build_sizes_khr`] with `pBuildInfo` = [`p_infos`][i] and with each element of the `pMaxPrimitiveCounts` array greater than or equal to the equivalent [`pp_build_range_infos`][i][j].`primitiveCount` values for `j` in [0,[`p_infos`][i].`geometryCount`)
-    Each element of [`pp_build_range_infos`][i]  **must**  be a valid pointer to an array of [`p_infos`][i].`geometryCount`[`AccelerationStructureBuildRangeInfoKHR`] structures

-    If [`deferred_operation`] is not [`crate::Handle::null`], it  **must**  be a valid [`DeferredOperationKHR`] object
-    Any previous deferred operation that was associated with [`deferred_operation`] **must**  be complete
-    For each element of [`p_infos`], the `buffer` used to create its `dstAccelerationStructure` member  **must**  be bound to host-visible device memory
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` the `buffer` used to create its `srcAccelerationStructure` member  **must**  be bound to host-visible device memory
-    For each element of [`p_infos`], the `buffer` used to create each acceleration structure referenced by the `geometry.instances.data` member of any element of `pGeometries` or `ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` **must**  be bound to host-visible device memory
-    The [[`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_host_commands`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureHostCommands) feature  **must**  be enabled
-    If [`p_infos`][i].`mode` is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR`, all addresses between [`p_infos`][i].`scratchData.hostAddress` and [`p_infos`][i].`scratchData.hostAddress` + N - 1  **must**  be valid host memory, where N is given by the `buildScratchSize` member of the [`AccelerationStructureBuildSizesInfoKHR`] structure returned from a call to [`get_acceleration_structure_build_sizes_khr`] with an identical [`AccelerationStructureBuildGeometryInfoKHR`] structure and primitive count
-    If [`p_infos`][i].`mode` is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`, all addresses between [`p_infos`][i].`scratchData.hostAddress` and [`p_infos`][i].`scratchData.hostAddress` + N - 1  **must**  be valid host memory, where N is given by the `updateScratchSize` member of the [`AccelerationStructureBuildSizesInfoKHR`] structure returned from a call to [`get_acceleration_structure_build_sizes_khr`] with an identical [`AccelerationStructureBuildGeometryInfoKHR`] structure and primitive count
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, `geometry.triangles.vertexData.hostAddress` **must**  be a valid host address
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.indexType` is not `VK_INDEX_TYPE_NONE_KHR`, `geometry.triangles.indexData.hostAddress` **must**  be a valid host address
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if `geometry.triangles.transformData.hostAddress` is not `0`, it  **must**  be a valid host address
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_AABBS_KHR`, `geometry.aabbs.data.hostAddress` **must**  be a valid host address
-    For each element of [`p_infos`], the `buffer` used to create its `dstAccelerationStructure` member  **must**  be bound to memory that was not allocated with multiple instances
-    For each element of [`p_infos`], if its `mode` member is `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` the `buffer` used to create its `srcAccelerationStructure` member  **must**  be bound to memory that was not allocated with multiple instances
-    For each element of [`p_infos`], the `buffer` used to create each acceleration structure referenced by the `geometry.instances.data` member of any element of `pGeometries` or `ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` **must**  be bound to memory that was not allocated with multiple instances
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, `geometry.instances.data.hostAddress` **must**  be a valid host address
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, each [`AccelerationStructureInstanceKHR::acceleration_structure_reference`] value in `geometry.instances.data.hostAddress` must be a valid [`AccelerationStructureKHR`] object
-    For any element of [`p_infos`][i].`pGeometries` or [`p_infos`][i].`ppGeometries` with a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR` with `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` set, each `accelerationStructureReference` in any structure in [`AccelerationStructureMotionInstanceNV`] value in `geometry.instances.data.hostAddress` must be a valid [`AccelerationStructureKHR`] object

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`deferred_operation`] is not [`crate::Handle::null`], [`deferred_operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-  [`p_infos`] **must**  be a valid pointer to an array of [`info_count`] valid [`AccelerationStructureBuildGeometryInfoKHR`] structures
-  [`pp_build_range_infos`] **must**  be a valid pointer to an array of [`info_count`][`AccelerationStructureBuildRangeInfoKHR`] structures
-  [`info_count`] **must**  be greater than `0`
-    If [`deferred_operation`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_OPERATION_DEFERRED_KHR`  - `VK_OPERATION_NOT_DEFERRED_KHR` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`khr_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureBuildRangeInfoKHR`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        