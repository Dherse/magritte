[VK_KHR_acceleration_structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html) - device extension

# Description
In order to be efficient, rendering techniques such as ray tracing need a
quick way to identify which primitives may be intersected by a ray
traversing the geometries.
Acceleration structures are the most common way to represent the geometry
spatially sorted, in order to quickly identify such potential intersections.This extension adds new functionalities:
- Acceleration structure objects and build commands
- Structures to describe geometry inputs to acceleration structure builds
- Acceleration structure copy commands

# Registered extension number
151

# Revision
13

# Dependencies
- Requires Vulkan 1.1
- Requires `[`VK_EXT_descriptor_indexing`]`
- Requires `[`VK_KHR_buffer_device_address`]`
- Requires `[`VK_KHR_deferred_host_operations`]`

# Contacts
- Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_acceleration_structure] @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_acceleration_structure extension>>)

# New object types
- [`AccelerationStructureKHR`]

# New commands
- [`build_acceleration_structures_khr`]
- [`cmd_build_acceleration_structures_indirect_khr`]
- [`cmd_build_acceleration_structures_khr`]
- [`cmd_copy_acceleration_structure_khr`]
- [`cmd_copy_acceleration_structure_to_memory_khr`]
- [`cmd_copy_memory_to_acceleration_structure_khr`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`copy_acceleration_structure_khr`]
- [`copy_acceleration_structure_to_memory_khr`]
- [`copy_memory_to_acceleration_structure_khr`]
- [`create_acceleration_structure_khr`]
- [`destroy_acceleration_structure_khr`]
- [`get_acceleration_structure_build_sizes_khr`]
- [`get_acceleration_structure_device_address_khr`]
- [`get_device_acceleration_structure_compatibility_khr`]
- [`write_acceleration_structures_properties_khr`]

# New structures
- [`AabbPositionsKHR`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureBuildRangeInfoKHR`]
- [`AccelerationStructureBuildSizesInfoKHR`]
- [`AccelerationStructureCreateInfoKHR`]
- [`AccelerationStructureDeviceAddressInfoKHR`]
- [`AccelerationStructureGeometryAabbsDataKHR`]
- [`AccelerationStructureGeometryInstancesDataKHR`]
- [`AccelerationStructureGeometryKHR`]
- [`AccelerationStructureGeometryTrianglesDataKHR`]
- [`AccelerationStructureInstanceKHR`]
- [`AccelerationStructureVersionInfoKHR`]
- [`CopyAccelerationStructureInfoKHR`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]
- [`TransformMatrixKHR`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceAccelerationStructureFeaturesKHR`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceAccelerationStructurePropertiesKHR`] 
- Extending [`WriteDescriptorSet`]:  - [`WriteDescriptorSetAccelerationStructureKHR`]

# New unions
- [`AccelerationStructureGeometryDataKHR`]
- [`DeviceOrHostAddressConstKHR`]
- [`DeviceOrHostAddressKHR`]

# New enums
- [`AccelerationStructureBuildTypeKHR`]
- [`AccelerationStructureCompatibilityKHR`]
- [`AccelerationStructureCreateFlagBitsKHR`]
- [`AccelerationStructureTypeKHR`]
- [`BuildAccelerationStructureFlagBitsKHR`]
- [`BuildAccelerationStructureModeKHR`]
- [`CopyAccelerationStructureModeKHR`]
- [`GeometryFlagBitsKHR`]
- [`GeometryInstanceFlagBitsKHR`]
- [`GeometryTypeKHR`]

# New bitmasks
- [`AccelerationStructureCreateFlagsKHR`]
- [`BuildAccelerationStructureFlagsKHR`]
- [`GeometryFlagsKHR`]
- [`GeometryInstanceFlagsKHR`]

# New constants
- [`KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME`]
- [`KHR_ACCELERATION_STRUCTURE_SPEC_VERSION`]
- Extending [`AccessFlagBits`]:  - `VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`  - `VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR` 
- Extending [`BufferUsageFlagBits`]:  - `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR`  - `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR` 
- Extending [`DebugReportObjectTypeEXT`]:  - `VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT` 
- Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` 
- Extending [`FormatFeatureFlagBits`]:  - `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR` 
- Extending [`IndexType`]:  - `VK_INDEX_TYPE_NONE_KHR` 
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR` 
- Extending [`PipelineStageFlagBits`]:  - `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` 
- Extending [`QueryType`]:  - `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`  - `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`  - `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`  - `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`  - `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`  - `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`  - `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR` 
If [`VK_KHR_format_feature_flags2`] is supported:
- Extending [`FormatFeatureFlagBits2`]:  - `VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`

# Known issues & F.A.Q.
(1) How does this extension differ from VK_NV_ray_tracing? **DISCUSSION** :The following is a summary of the main functional differences between
VK_KHR_acceleration_structure and VK_NV_ray_tracing:
- added acceleration structure serialization / deserialization (`VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`, `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR`, [`cmd_copy_acceleration_structure_to_memory_khr`], [`cmd_copy_memory_to_acceleration_structure_khr`])
- document [inactive primitives and instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
- added [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure
- added indirect and batched acceleration structure builds ([`cmd_build_acceleration_structures_indirect_khr`])
- added [host acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#host-acceleration-structure) commands
- reworked geometry structures so they could be better shared between device, host, and indirect builds
- explicitly made [`AccelerationStructureKHR`] use device addresses
- added acceleration structure compatibility check function ([`get_device_acceleration_structure_compatibility_khr`])
- add parameter for requesting memory requirements for host and/or device build
- added format feature for acceleration structure build vertex formats (`VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`)
(2) Can you give a more detailed comparision of differences and similarities
between VK_NV_ray_tracing and VK_KHR_acceleration_structure? **DISCUSSION** :The following is a more detailed comparision of which commands, structures,
and enums are aliased, changed, or removed.
- Aliased functionality — enums, structures, and commands that are considered equivalent:  - [`GeometryTypeNV`] ↔ [`GeometryTypeKHR`]  - [`AccelerationStructureTypeNV`] ↔ [`AccelerationStructureTypeKHR`]  - [`CopyAccelerationStructureModeNV`] ↔ [`CopyAccelerationStructureModeKHR`]  - [`GeometryFlagsNV`] ↔ [`GeometryFlagsKHR`]  - [`GeometryFlagBitsNV`] ↔ [`GeometryFlagBitsKHR`]  - [`GeometryInstanceFlagsNV`] ↔ [`GeometryInstanceFlagsKHR`]  - [`GeometryInstanceFlagBitsNV`] ↔ [`GeometryInstanceFlagBitsKHR`]  - [`BuildAccelerationStructureFlagsNV`] ↔ [`BuildAccelerationStructureFlagsKHR`]  - [`BuildAccelerationStructureFlagBitsNV`] ↔ [`BuildAccelerationStructureFlagBitsKHR`]  - [`TransformMatrixNV`] ↔ [`TransformMatrixKHR`] (added to VK_NV_ray_tracing for descriptive purposes)  - [`AabbPositionsNV`] ↔ [`AabbPositionsKHR`] (added to VK_NV_ray_tracing for descriptive purposes)  - [`AccelerationStructureInstanceNV`] ↔ [`AccelerationStructureInstanceKHR`] (added to VK_NV_ray_tracing for descriptive purposes) 
- Changed enums, structures, and commands:  - renamed `VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV` → `VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR` in [`GeometryInstanceFlagBitsKHR`]  - [`GeometryTrianglesNV`] → [`AccelerationStructureGeometryTrianglesDataKHR`] (device or host address instead of buffer+offset)  - [`GeometryAabbNV`] → [`AccelerationStructureGeometryAabbsDataKHR`] (device or host address instead of buffer+offset)  - [`GeometryDataNV`] → [`AccelerationStructureGeometryDataKHR`] (union of triangle/aabbs/instances)  - [`GeometryNV`] → [`AccelerationStructureGeometryKHR`] (changed type of geometry)  - [`AccelerationStructureCreateInfoNV`] → [`AccelerationStructureCreateInfoKHR`] (reshuffle geometry layout/information)  - [`PhysicalDeviceRayTracingPropertiesNV`] → [`PhysicalDeviceAccelerationStructurePropertiesKHR`] (for acceleration structure properties, renamed `maxTriangleCount` to `maxPrimitiveCount`, added per stage and update after bind limits) and [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] (for ray tracing pipeline properties)  - [`AccelerationStructureMemoryRequirementsInfoNV`] (deleted - replaced by allocating on top of [`Buffer`])  - [`WriteDescriptorSetAccelerationStructureNV`] → [`WriteDescriptorSetAccelerationStructureKHR`] (different acceleration structure type)  - [`create_acceleration_structure_nv`] → [`create_acceleration_structure_khr`] (device address, different geometry layout/information)  - [`get_acceleration_structure_memory_requirements_nv`] (deleted - replaced by allocating on top of [`Buffer`])  - [`cmd_build_acceleration_structure_nv`] → [`cmd_build_acceleration_structures_khr`] (params moved to structs, layout differences)  - [`cmd_copy_acceleration_structure_nv`] → [`cmd_copy_acceleration_structure_khr`] (params to struct, extendable)  - [`get_acceleration_structure_handle_nv`] → [`get_acceleration_structure_device_address_khr`] (device address instead of handle)  - [`AccelerationStructureMemoryRequirementsTypeNV`] → size queries for scratch space moved to [`get_acceleration_structure_build_sizes_khr`]  - [`destroy_acceleration_structure_nv`] → [`destroy_acceleration_structure_khr`] (different acceleration structure types)  - [`cmd_write_acceleration_structures_properties_nv`] → [`cmd_write_acceleration_structures_properties_khr`] (different acceleration structure types) 
- Added enums, structures and commands:  - `VK_GEOMETRY_TYPE_INSTANCES_KHR` to [`GeometryTypeKHR`] enum  - `VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`, `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR` to [`CopyAccelerationStructureModeKHR`] enum  - [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure  - [`AccelerationStructureBuildTypeKHR`] enum  - [`BuildAccelerationStructureModeKHR`] enum  - [`DeviceOrHostAddressKHR`] and [`DeviceOrHostAddressConstKHR`] unions  - [`AccelerationStructureBuildRangeInfoKHR`] struct  - [`AccelerationStructureGeometryInstancesDataKHR`] struct  - [`AccelerationStructureDeviceAddressInfoKHR`] struct  - [`AccelerationStructureVersionInfoKHR`] struct  - [`StridedDeviceAddressRegionKHR`] struct  - [`CopyAccelerationStructureToMemoryInfoKHR`] struct  - [`CopyMemoryToAccelerationStructureInfoKHR`] struct  - [`CopyAccelerationStructureInfoKHR`] struct  - [`build_acceleration_structures_khr`] command (host build)  - [`copy_acceleration_structure_khr`] command (host copy)  - [`copy_acceleration_structure_to_memory_khr`] (host serialize)  - [`copy_memory_to_acceleration_structure_khr`] (host deserialize)  - [`write_acceleration_structures_properties_khr`] (host properties)  - [`cmd_copy_acceleration_structure_to_memory_khr`] (device serialize)  - [`cmd_copy_memory_to_acceleration_structure_khr`] (device deserialize)  - [`get_device_acceleration_structure_compatibility_khr`] (serialization) 
(3) What are the changes between the public provisional (VK_KHR_ray_tracing
v8) release and the internal provisional (VK_KHR_ray_tracing v9) release?
- added `geometryFlags` to `VkAccelerationStructureCreateGeometryTypeInfoKHR` (later reworked to obsolete this)
- added `minAccelerationStructureScratchOffsetAlignment` property to VkPhysicalDeviceRayTracingPropertiesKHR
- fix naming and return enum from [`get_device_acceleration_structure_compatibility_khr`]  - renamed `VkAccelerationStructureVersionKHR` to [`AccelerationStructureVersionInfoKHR`]  - renamed `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_KHR` to `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`  - removed `VK_ERROR_INCOMPATIBLE_VERSION_KHR`  - added [`AccelerationStructureCompatibilityKHR`] enum  - remove return value from [`get_device_acceleration_structure_compatibility_khr`] and added return enum parameter 
- Require Vulkan 1.1
- added creation time capture and replay flags  - added [`AccelerationStructureCreateFlagBitsKHR`] and [`AccelerationStructureCreateFlagsKHR`]  - renamed the `flags` member of [`AccelerationStructureCreateInfoKHR`] to `buildFlags` (later removed) and added the `createFlags` member 
- change [`cmd_build_acceleration_structures_indirect_khr`] to use buffer device address for indirect parameter
- make `[`VK_KHR_deferred_host_operations`]` an interaction instead of a required extension (later went back on this)
- renamed `VkAccelerationStructureBuildOffsetInfoKHR` to [`AccelerationStructureBuildRangeInfoKHR`]  - renamed the `ppOffsetInfos` parameter of [`cmd_build_acceleration_structures_khr`] to `ppBuildRangeInfos` 
- Re-unify geometry description between build and create  - remove `VkAccelerationStructureCreateGeometryTypeInfoKHR` and `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR`  - added `VkAccelerationStructureCreateSizeInfoKHR` structure (later removed)  - change type of the `pGeometryInfos` member of [`AccelerationStructureCreateInfoKHR`] from `VkAccelerationStructureCreateGeometryTypeInfoKHR` to [`AccelerationStructureGeometryKHR`] (later removed)  - added `pCreateSizeInfos` member to [`AccelerationStructureCreateInfoKHR`] (later removed) 
- Fix ppGeometries ambiguity, add pGeometries  - remove `geometryArrayOfPointers` member of VkAccelerationStructureBuildGeometryInfoKHR  - disambiguate two meanings of `ppGeometries` by explicitly adding `pGeometries` to the [`AccelerationStructureBuildGeometryInfoKHR`] structure and require one of them be `NULL` 
- added [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor) support for acceleration structures
- changed the `update` member of [`AccelerationStructureBuildGeometryInfoKHR`] from a bool to the `mode`[`BuildAccelerationStructureModeKHR`] enum which allows future extensibility in update types
- Clarify deferred host ops for pipeline creation  - [`DeferredOperationKHR`] is now a top-level parameter for [`build_acceleration_structures_khr`], [`create_ray_tracing_pipelines_khr`], [`copy_acceleration_structure_to_memory_khr`], [`copy_acceleration_structure_khr`], and [`copy_memory_to_acceleration_structure_khr`]  - removed `VkDeferredOperationInfoKHR` structure  - change deferred host creation/return parameter behavior such that the implementation can modify such parameters until the deferred host operation completes  - `[`VK_KHR_deferred_host_operations`]` is required again 
- Change acceleration structure build to always be sized  - de-alias [`AccelerationStructureMemoryRequirementsTypeNV`] and `VkAccelerationStructureMemoryRequirementsTypeKHR`, and remove `VkAccelerationStructureMemoryRequirementsTypeKHR`  - add [`get_acceleration_structure_build_sizes_khr`] command and [`AccelerationStructureBuildSizesInfoKHR`] structure and `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR` enum to query sizes for acceleration structures and scratch storage  - move size queries for scratch space to [`get_acceleration_structure_build_sizes_khr`]  - remove `compactedSize`, `buildFlags`, `maxGeometryCount`, `pGeometryInfos`, `pCreateSizeInfos` members of [`AccelerationStructureCreateInfoKHR`] and add the `size` member  - add `maxVertex` member to [`AccelerationStructureGeometryTrianglesDataKHR`] structure  - remove `VkAccelerationStructureCreateSizeInfoKHR` structure 
(4) What are the changes between the internal provisional
(VK_KHR_ray_tracing v9) release and the final (VK_KHR_acceleration_structure
v11) release?
- refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation flexibility and decoupling ray query support from ray pipelines:  - `[`VK_KHR_acceleration_structure`]` (for acceleration structure operations)  - `[`VK_KHR_ray_tracing_pipeline`]` (for ray tracing pipeline and shader stages)  - `[`VK_KHR_ray_query`]` (for ray queries in existing shader stages) 
- clarify buffer usage flags for ray tracing  - `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` is left alone in `[`VK_NV_ray_tracing`]` (required on `scratch` and `instanceData`)  - `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` is added as an alias of `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` in `[`VK_KHR_ray_tracing_pipeline`]` and is required on shader binding table buffers  - `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR` is added in `[`VK_KHR_acceleration_structure`]` for all vertex, index, transform, aabb, and instance buffer data referenced by device build commands  - `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` is used for `scratchData` 
- add max primitive counts (`ppMaxPrimitiveCounts`) to [`cmd_build_acceleration_structures_indirect_khr`]
- Allocate acceleration structures from `VkBuffers` and add a mode to constrain the device address  - de-alias [`BindAccelerationStructureMemoryInfoNV`] and [`bind_acceleration_structure_memory_nv`], and remove `VkBindAccelerationStructureMemoryInfoKHR`, `VkAccelerationStructureMemoryRequirementsInfoKHR`, and `vkGetAccelerationStructureMemoryRequirementsKHR`  - acceleration structures now take a [`Buffer`] and offset at creation time for memory placement  - add a new `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR` buffer usage for such buffers  - add a new `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR` acceleration structure type for layering 
- move `VK_GEOMETRY_TYPE_INSTANCES_KHR` to main enum instead of being added via extension
- make build commands more consistent - all now build multiple acceleration structures and are named plurally ([`cmd_build_acceleration_structures_indirect_khr`], [`cmd_build_acceleration_structures_khr`], [`build_acceleration_structures_khr`])
- add interactions with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` for acceleration structures, including a new feature (`descriptorBindingAccelerationStructureUpdateAfterBind`) and 3 new properties (`maxPerStageDescriptorAccelerationStructures`, `maxPerStageDescriptorUpdateAfterBindAccelerationStructures`, `maxDescriptorSetUpdateAfterBindAccelerationStructures`)
- extension is no longer provisional
- define synchronization requirements for builds, traces, and copies
- define synchronization requirements for AS build inputs and indirect build buffer
(5) What is `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR` for? **RESOLVED** : It is primarily intended for API layering.
In DXR, the acceleration structure is basically just a buffer in a special
layout, and you do not know at creation time whether it will be used as a
top or bottom level acceleration structure.
We thus added a generic acceleration structure type whose type is unknown at
creation time, but is specified at build time instead.
Applications which are written directly for Vulkan should not use it.

# Version history
- Revision 1, 2019-12-05 (Members of the Vulkan Ray Tracing TSG)  - Internal revisions (forked from VK_NV_ray_tracing) 
- Revision 2, 2019-12-20 (Daniel Koch, Eric Werness)  - Add const version of DeviceOrHostAddress (!3515)  - Add VU to clarify that only handles in the current pipeline are valid (!3518)  - Restore some missing VUs and add in-place update language (#1902, !3522)  - rename VkAccelerationStructureInstanceKHR member from accelerationStructure to accelerationStructureReference to better match its type (!3523)  - Allow VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS for pipeline creation if shader group handles cannot be reused (!3523)  - update documentation for the VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS error code and add missing documentation for new return codes from VK_KHR_deferred_host_operations (!3523)  - list new query types for VK_KHR_ray_tracing (!3523)  - Fix VU statements for VkAccelerationStructureGeometryKHR referring to correct union members and update to use more current wording (!3523) 
- Revision 3, 2020-01-10 (Daniel Koch, Jon Leech, Christoph Kubisch)  - Fix 'instance of' and 'that/which contains/defines' markup issues (!3528)  - factor out VK_KHR_pipeline_library as stand-alone extension (!3540)  - Resolve Vulkan-hpp issues (!3543)  - add missing require for VkGeometryInstanceFlagsKHR  - de-alias VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV since the KHR structure is no longer equivalent  - add len to pDataSize attribute for vkWriteAccelerationStructuresPropertiesKHR 
- Revision 4, 2020-01-23 (Daniel Koch, Eric Werness)  - Improve vkWriteAccelerationStructuresPropertiesKHR, add return value and VUs (#1947)  - Clarify language to allow multiple raygen shaders (#1959)  - Various editorial feedback (!3556)  - Add language to help deal with looped self-intersecting fans (#1901)  - Change vkCmdTraceRays{Indirect}KHR args to pointers (!3559)  - Add scratch address validation language (#1941, !3551)  - Fix definition and add hierarchy information for shader call scope (#1977, !3571) 
- Revision 5, 2020-02-04 (Eric Werness, Jeff Bolz, Daniel Koch)  - remove vestigial accelerationStructureUUID (!3582)  - update definition of repack instructions and improve memory model interactions (#1910, #1913, !3584)  - Fix wrong sType for VkPhysicalDeviceRayTracingFeaturesKHR (#1988)  - Use provisional SPIR-V capabilities (#1987)  - require rayTraversalPrimitiveCulling if rayQuery is supported (#1927)  - Miss shaders do not have object parameters (!3592)  - Fix missing required types in XML (!3592)  - clarify matching conditions for update (!3592)  - add goal that host and device builds be similar (!3592)  - clarify that `maxPrimitiveCount` limit should apply to triangles and AABBs (!3592)  - Require alignment for instance arrayOfPointers (!3592)  - Zero is a valid value for instance flags (!3592)  - Add some alignment VUs that got lost in refactoring (!3592)  - Recommend TMin epsilon rather than culling (!3592)  - Get angle from dot product not cross product (!3592)  - Clarify that AH can access the payload and attributes (!3592)  - Match DXR behavior for inactive primitive definition (!3592)  - Use a more generic term than degenerate for inactive to avoid confusion (!3592) 
- Revision 6, 2020-02-20 (Daniel Koch)  - fix some dangling NV references (#1996)  - rename VkCmdTraceRaysIndirectCommandKHR to VkTraceRaysIndirectCommandKHR (!3607)  - update contributor list (!3611)  - use uint64_t instead of VkAccelerationStructureReferenceKHR in VkAccelerationStructureInstanceKHR (#2004) 
- Revision 7, 2020-02-28 (Tobias Hector)  - remove HitTKHR SPIR-V builtin (spirv/spirv-extensions#7) 
- Revision 8, 2020-03-06 (Tobias Hector, Dae Kim, Daniel Koch, Jeff Bolz, Eric Werness)  - explicitly state that Tmax is updated when new closest intersection is accepted (#2020,!3536)  - Made references to min and max t values consistent (!3644)  - finish enumerating differences relative to VK_NV_ray_tracing in issues (1) and (2) (#1974,!3642)  - fix formatting in some math equations (!3642)  - Restrict the Hit Kind operand of `OpReportIntersectionKHR` to 7-bits (spirv/spirv-extensions#8,!3646)  - Say ray tracing ' **should** ' be watertight (#2008,!3631)  - Clarify memory requirements for ray tracing buffers (#2005,!3649)  - Add callable size limits (#1997,!3652) 
- Revision 9, 2020-04-15 (Eric Werness, Daniel Koch, Tobias Hector, Joshua Barczak)  - Add geometry flags to acceleration structure creation (!3672)  - add build scratch memory alignment (minAccelerationStructureScratchOffsetAlignment) (#2065,!3725)  - fix naming and return enum from vkGetDeviceAccelerationStructureCompatibilityKHR (#2051,!3726)  - require SPIR-V 1.4 (#2096,!3777)  - added creation time capture/replay flags (#2104,!3774)  - require Vulkan 1.1 (#2133,!3806)  - use device addresses instead of VkBuffers for ray tracing commands (#2074,!3815)  - add interactions with Vulkan 1.2 and VK_KHR_vulkan_memory_model (#2133,!3830)  - make VK_KHR_pipeline_library an interaction instead of required (#2045,#2108,!3830)  - make VK_KHR_deferred_host_operations an interaction instead of required (#2045,!3830)  - removed maxCallableSize and added explicit stack size management for ray pipelines (#1997,!3817,!3772,!3844)  - improved documentation for VkAccelerationStructureVersionInfoKHR (#2135,3835)  - rename VkAccelerationStructureBuildOffsetInfoKHR to VkAccelerationStructureBuildRangeInfoKHR (#2058,!3754)  - Re-unify geometry description between build and create (!3754)  - Fix ppGeometries ambiguity, add pGeometries (#2032,!3811)  - add interactions with VK_EXT_robustness2 and allow nullDescriptor support for acceleration structures (#1920,!3848)  - added future extensibility for AS updates (#2114,!3849)  - Fix VU for dispatchrays and add a limit on the size of the full grid (#2160,!3851)  - Add shaderGroupHandleAlignment property (#2180,!3875)  - Clarify deferred host ops for pipeline creation (#2067,!3813)  - Change acceleration structure build to always be sized (#2131,#2197,#2198,!3854,!3883,!3880) 
- Revision 10, 2020-07-03 (Mathieu Robart, Daniel Koch, Eric Werness, Tobias Hector)  - Decomposition of the specification, from VK_KHR_ray_tracing to VK_KHR_acceleration_structure (#1918,!3912)  - clarify buffer usage flags for ray tracing (#2181,!3939)  - add max primitive counts to build indirect command (#2233,!3944)  - Allocate acceleration structures from VkBuffers and add a mode to constrain the device address (#2131,!3936)  - Move VK_GEOMETRY_TYPE_INSTANCES_KHR to main enum (#2243,!3952)  - make build commands more consistent (#2247,!3958)  - add interactions with UPDATE_AFTER_BIND (#2128,!3986)  - correct and expand build command VUs (!4020)  - fix copy command VUs (!4018)  - added various alignment requirements (#2229,!3943)  - fix valid usage for arrays of geometryCount items (#2198,!4010)  - define what is allowed to change on RTAS updates and relevant VUs (#2177,!3961) 
- Revision 11, 2020-11-12 (Eric Werness, Josh Barczak, Daniel Koch, Tobias Hector)  - de-alias NV and KHR acceleration structure types and associated commands (#2271,!4035)  - specify alignment for host copy commands (#2273,!4037)  - document `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`  - specify that acceleration structures are non-linear (#2289,!4068)  - add several missing VUs for strides, vertexFormat, and indexType (#2315,!4069)  - restore VUs for VkAccelerationStructureBuildGeometryInfoKHR (#2337,!4098)  - ban multi-instance memory for host operations (#2324,!4102)  - allow dstAccelerationStructure to be null for vkGetAccelerationStructureBuildSizesKHR (#2330,!4111)  - more build VU cleanup (#2138,#4130)  - specify host endianness for AS serialization (#2261,!4136)  - add invertible transform matrix VU (#1710,!4140)  - require geometryCount to be 1 for TLAS builds (!4145)  - improved validity conditions for build addresses (#4142)  - add single statement SPIR-V VUs, build limit VUs (!4158)  - document limits for vertex and aabb strides (#2390,!4184)  - specify that `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` applies to AS copies (#2382,#4173)  - define sync for AS build inputs and indirect buffer (#2407,!4208) 
- Revision 12, 2021-08-06 (Samuel Bourasseau)  - rename VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR to VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR (keep previous as alias).  - Clarify description and add note. 
- Revision 13, 2021-09-30 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]` to `vk.xml`

# Other information
* 2021-09-30
*   - Samuel Bourasseau, Adobe  - Matthäus Chajdas, AMD  - Greg Grebe, AMD  - Nicolai Hähnle, AMD  - Tobias Hector, AMD  - Dave Oldcorn, AMD  - Skyler Saleh, AMD  - Mathieu Robart, Arm  - Marius Bjorge, Arm  - Tom Olson, Arm  - Sebastian Tafuri, EA  - Henrik Rydgard, Embark  - Juan Cañada, Epic Games  - Patrick Kelly, Epic Games  - Yuriy O’Donnell, Epic Games  - Michael Doggett, Facebook/Oculus  - Ricardo Garcia, Igalia  - Andrew Garrard, Imagination  - Don Scorgie, Imagination  - Dae Kim, Imagination  - Joshua Barczak, Intel  - Slawek Grajewski, Intel  - Jeff Bolz, NVIDIA  - Pascal Gautron, NVIDIA  - Daniel Koch, NVIDIA  - Christoph Kubisch, NVIDIA  - Ashwin Lele, NVIDIA  - Robert Stepinski, NVIDIA  - Martin Stich, NVIDIA  - Nuno Subtil, NVIDIA  - Eric Werness, NVIDIA  - Jon Leech, Khronos  - Jeroen van Schijndel, OTOY  - Juul Joosten, OTOY  - Alex Bourd, Qualcomm  - Roman Larionov, Qualcomm  - David McAllister, Qualcomm  - Lewis Gordon, Samsung  - Ralph Potter, Samsung  - Jasper Bekkers, Traverse Research  - Jesse Barker, Unity  - Baldur Karlsson, Valve

# Related
- [`AabbPositionsKHR`]
- [`AccelerationStructureBuildGeometryInfoKHR`]
- [`AccelerationStructureBuildRangeInfoKHR`]
- [`AccelerationStructureBuildSizesInfoKHR`]
- [`AccelerationStructureBuildTypeKHR`]
- [`AccelerationStructureCompatibilityKHR`]
- [`AccelerationStructureCreateFlagBitsKHR`]
- [`AccelerationStructureCreateFlagsKHR`]
- [`AccelerationStructureCreateInfoKHR`]
- [`AccelerationStructureDeviceAddressInfoKHR`]
- [`AccelerationStructureGeometryAabbsDataKHR`]
- [`AccelerationStructureGeometryDataKHR`]
- [`AccelerationStructureGeometryInstancesDataKHR`]
- [`AccelerationStructureGeometryKHR`]
- [`AccelerationStructureGeometryTrianglesDataKHR`]
- [`AccelerationStructureInstanceKHR`]
- [`AccelerationStructureKHR`]
- [`AccelerationStructureTypeKHR`]
- [`AccelerationStructureVersionInfoKHR`]
- [`BuildAccelerationStructureFlagBitsKHR`]
- [`BuildAccelerationStructureFlagsKHR`]
- [`BuildAccelerationStructureModeKHR`]
- [`CopyAccelerationStructureInfoKHR`]
- [`CopyAccelerationStructureModeKHR`]
- [`CopyAccelerationStructureToMemoryInfoKHR`]
- [`CopyMemoryToAccelerationStructureInfoKHR`]
- [`DeviceOrHostAddressConstKHR`]
- [`DeviceOrHostAddressKHR`]
- [`GeometryFlagBitsKHR`]
- [`GeometryFlagsKHR`]
- [`GeometryInstanceFlagBitsKHR`]
- [`GeometryInstanceFlagsKHR`]
- [`GeometryTypeKHR`]
- [`PhysicalDeviceAccelerationStructureFeaturesKHR`]
- [`PhysicalDeviceAccelerationStructurePropertiesKHR`]
- [`TransformMatrixKHR`]
- [`WriteDescriptorSetAccelerationStructureKHR`]
- [`build_acceleration_structures_khr`]
- [`cmd_build_acceleration_structures_indirect_khr`]
- [`cmd_build_acceleration_structures_khr`]
- [`cmd_copy_acceleration_structure_khr`]
- [`cmd_copy_acceleration_structure_to_memory_khr`]
- [`cmd_copy_memory_to_acceleration_structure_khr`]
- [`cmd_write_acceleration_structures_properties_khr`]
- [`copy_acceleration_structure_khr`]
- [`copy_acceleration_structure_to_memory_khr`]
- [`copy_memory_to_acceleration_structure_khr`]
- [`create_acceleration_structure_khr`]
- [`destroy_acceleration_structure_khr`]
- [`get_acceleration_structure_build_sizes_khr`]
- [`get_acceleration_structure_device_address_khr`]
- [`get_device_acceleration_structure_compatibility_khr`]
- [`write_acceleration_structures_properties_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        