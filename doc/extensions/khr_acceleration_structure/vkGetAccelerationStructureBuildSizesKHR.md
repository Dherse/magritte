[vkGetAccelerationStructureBuildSizesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetAccelerationStructureBuildSizesKHR.html) - Retrieve the required size for an acceleration structure

# C Specifications
To get the build sizes for an acceleration structure, call:
```c
// Provided by VK_KHR_acceleration_structure
void vkGetAccelerationStructureBuildSizesKHR(
    VkDevice                                    device,
    VkAccelerationStructureBuildTypeKHR         buildType,
    const VkAccelerationStructureBuildGeometryInfoKHR* pBuildInfo,
    const uint32_t*                             pMaxPrimitiveCounts,
    VkAccelerationStructureBuildSizesInfoKHR*   pSizeInfo);
```

# Parameters
- [`device`] is the logical device that will be used for creating the acceleration structure.
- [`build_type`] defines whether host or device operations (or both) are being queried for.
- [`p_build_info`] is a pointer to a [`AccelerationStructureBuildGeometryInfoKHR`] structure describing parameters of a build operation.
- [`p_max_primitive_counts`] is a pointer to an array of `pBuildInfo->geometryCount``uint32_t` values defining the number of primitives built into each geometry.
- [`p_size_info`] is a pointer to a [`AccelerationStructureBuildSizesInfoKHR`] structure which returns the size required for an acceleration structure and the sizes required for the scratch buffers, given the build parameters.

# Description
The `srcAccelerationStructure`, `dstAccelerationStructure`, and
`mode` members of [`p_build_info`] are ignored.
Any [`DeviceOrHostAddressKHR`] members of [`p_build_info`] are ignored
by this command, except that the `hostAddress` member of
[`AccelerationStructureGeometryTrianglesDataKHR::transform_data`]
will be examined to check if it is `NULL`.An acceleration structure created with the `accelerationStructureSize`
returned by this command supports any build or update with a
[`AccelerationStructureBuildGeometryInfoKHR`] structure and array of
[`AccelerationStructureBuildRangeInfoKHR`] structures subject to the
following properties:
- The build command is a host build command, and [`build_type`] is `VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR` or `VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR`
- The build command is a device build command, and [`build_type`] is `VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR` or `VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR`
- For [`AccelerationStructureBuildGeometryInfoKHR`]:  - Its `type`, and `flags` members are equal to `pBuildInfo->type` and `pBuildInfo->flags`, respectively.  - `geometryCount` is less than or equal to `pBuildInfo->geometryCount`.  - For each element of either `pGeometries` or `ppGeometries` at a given index, its `geometryType` member is equal to `pBuildInfo->geometryType`.  - For each element of either `pGeometries` or `ppGeometries` at a given index, with a `geometryType` member equal to `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the `vertexFormat` and `indexType` members of `geometry.triangles` are equal to the corresponding members of the same element in [`p_build_info`].  - For each element of either `pGeometries` or `ppGeometries` at a given index, with a `geometryType` member equal to `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the `maxVertex` member of `geometry.triangles` is less than or equal to the corresponding member of the same element in [`p_build_info`].  - For each element of either `pGeometries` or `ppGeometries` at a given index, with a `geometryType` member equal to `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the applicable address in the `transformData` member of `geometry.triangles` is not `NULL`, the corresponding `transformData.hostAddress` parameter in [`p_build_info`] is not `NULL`. 
- For each [`AccelerationStructureBuildRangeInfoKHR`] corresponding to the [`AccelerationStructureBuildGeometryInfoKHR`]:  - Its `primitiveCount` member is less than or equal to the corresponding element of [`p_max_primitive_counts`]. 
Similarly, the `updateScratchSize` value will support any build command
specifying the `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR``mode` under the above conditions, and the `buildScratchSize` value
will support any build command specifying the
`VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR``mode` under the
above conditions.
## Valid Usage
-    The [`rayTracingPipeline`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayTracingPipeline) or [`rayQuery`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-rayQuery) feature  **must**  be enabled
-    If [`device`] was created with multiple physical devices, then the [bufferDeviceAddressMultiDevice](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-bufferDeviceAddressMultiDevice) feature  **must**  be enabled
-    If `pBuildInfo->geometryCount` is not `0`, [`p_max_primitive_counts`] **must**  be a valid pointer to an array of `pBuildInfo->geometryCount``uint32_t` values
-    If `pBuildInfo->pGeometries` or `pBuildInfo->ppGeometries` has a `geometryType` of `VK_GEOMETRY_TYPE_INSTANCES_KHR`, each [`p_max_primitive_counts`][i]  **must**  be less than or equal to [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_instance_count`]

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`build_type`] **must**  be a valid [`AccelerationStructureBuildTypeKHR`] value
-  [`p_build_info`] **must**  be a valid pointer to a valid [`AccelerationStructureBuildGeometryInfoKHR`] structure
-    If [`p_max_primitive_counts`] is not `NULL`, [`p_max_primitive_counts`] **must**  be a valid pointer to an array of `pBuildInfo->geometryCount``uint32_t` values
-  [`p_size_info`] **must**  be a valid pointer to a [`AccelerationStructureBuildSizesInfoKHR`] structure

# Related
- [`VK_KHR_acceleration_structure`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureBuildSizesInfoKHR`]
- [`AccelerationStructureBuildTypeKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        