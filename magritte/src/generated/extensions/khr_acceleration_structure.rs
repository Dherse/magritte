//![VK_KHR_acceleration_structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_acceleration_structure.html) - device extension
//!# Description
//!In order to be efficient, rendering techniques such as ray tracing need a
//!quick way to identify which primitives may be intersected by a ray
//!traversing the geometries.
//!Acceleration structures are the most common way to represent the geometry
//!spatially sorted, in order to quickly identify such potential intersections.This extension adds
//! new functionalities:
//! - Acceleration structure objects and build commands
//! - Structures to describe geometry inputs to acceleration structure builds
//! - Acceleration structure copy commands
//!# Revision
//!13
//!# Dependencies
//! - Requires Vulkan 1.1
//! - Requires `[`VK_EXT_descriptor_indexing`]`
//! - Requires `[`VK_KHR_buffer_device_address`]`
//! - Requires `[`VK_KHR_deferred_host_operations`]`
//!# Contacts
//! - Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_acceleration_structure]
//!   @dgkoch%0A<<Here describe the issue or question you have about the
//!   VK_KHR_acceleration_structure extension>>)
//!# New handles
//! - [`AccelerationStructureKHR`]
//!# New functions & commands
//! - [`BuildAccelerationStructuresKHR`]
//! - [`CmdBuildAccelerationStructuresIndirectKHR`]
//! - [`CmdBuildAccelerationStructuresKHR`]
//! - [`CmdCopyAccelerationStructureKHR`]
//! - [`CmdCopyAccelerationStructureToMemoryKHR`]
//! - [`CmdCopyMemoryToAccelerationStructureKHR`]
//! - [`CmdWriteAccelerationStructuresPropertiesKHR`]
//! - [`CopyAccelerationStructureKHR`]
//! - [`CopyAccelerationStructureToMemoryKHR`]
//! - [`CopyMemoryToAccelerationStructureKHR`]
//! - [`CreateAccelerationStructureKHR`]
//! - [`DestroyAccelerationStructureKHR`]
//! - [`GetAccelerationStructureBuildSizesKHR`]
//! - [`GetAccelerationStructureDeviceAddressKHR`]
//! - [`GetDeviceAccelerationStructureCompatibilityKHR`]
//! - [`WriteAccelerationStructuresPropertiesKHR`]
//!# New structures
//! - [`AabbPositionsKHR`]
//! - [`AccelerationStructureBuildGeometryInfoKHR`]
//! - [`AccelerationStructureBuildRangeInfoKHR`]
//! - [`AccelerationStructureBuildSizesInfoKHR`]
//! - [`AccelerationStructureCreateInfoKHR`]
//! - [`AccelerationStructureDeviceAddressInfoKHR`]
//! - [`AccelerationStructureGeometryAabbsDataKHR`]
//! - [`AccelerationStructureGeometryInstancesDataKHR`]
//! - [`AccelerationStructureGeometryKHR`]
//! - [`AccelerationStructureGeometryTrianglesDataKHR`]
//! - [`AccelerationStructureInstanceKHR`]
//! - [`AccelerationStructureVersionInfoKHR`]
//! - [`CopyAccelerationStructureInfoKHR`]
//! - [`CopyAccelerationStructureToMemoryInfoKHR`]
//! - [`CopyMemoryToAccelerationStructureInfoKHR`]
//! - [`TransformMatrixKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  -
//!   [`PhysicalDeviceAccelerationStructureFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:  -
//!   [`PhysicalDeviceAccelerationStructurePropertiesKHR`]
//! - Extending [`WriteDescriptorSet`]:  - [`WriteDescriptorSetAccelerationStructureKHR`]
//!# New enums
//! - [`AccelerationStructureBuildTypeKHR`]
//! - [`AccelerationStructureCompatibilityKHR`]
//! - [`AccelerationStructureCreateFlagBitsKHR`]
//! - [`AccelerationStructureTypeKHR`]
//! - [`BuildAccelerationStructureFlagBitsKHR`]
//! - [`BuildAccelerationStructureModeKHR`]
//! - [`CopyAccelerationStructureModeKHR`]
//! - [`GeometryFlagBitsKHR`]
//! - [`GeometryInstanceFlagBitsKHR`]
//! - [`GeometryTypeKHR`]
//!# New bitmasks
//! - [`AccelerationStructureCreateFlagsKHR`]
//! - [`BuildAccelerationStructureFlagsKHR`]
//! - [`GeometryFlagsKHR`]
//! - [`GeometryInstanceFlagsKHR`]
//!# New constants
//! - [`KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME`]
//! - [`KHR_ACCELERATION_STRUCTURE_SPEC_VERSION`]
//! - Extending [`AccessFlagBits`]:  - `VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`  -
//!   `VK_ACCESS_ACCELERATION_STRUCTURE_WRITE_BIT_KHR`
//! - Extending [`BufferUsageFlagBits`]:  -
//!   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR`  -
//!   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR`
//! - Extending [`DebugReportObjectTypeEXT`]:  -
//!   `VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT`
//! - Extending [`DescriptorType`]:  - `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
//! - Extending [`FormatFeatureFlagBits`]:  -
//!   `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`
//! - Extending [`IndexType`]:  - `VK_INDEX_TYPE_NONE_KHR`
//! - Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR`
//! - Extending [`PipelineStageFlagBits`]:  -
//!   `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`
//! - Extending [`QueryType`]:  - `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR`  -
//!   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
//! - Extending [`StructureType`]:  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`  -
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`  -
//!   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`  -
//!   `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
//!If [`VK_KHR_format_feature_flags2`] is supported:
//! - Extending [`FormatFeatureFlagBits2`]:  -
//!   `VK_FORMAT_FEATURE_2_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`
//!# Known issues & F.A.Q
//!(1) How does this extension differ from VK_NV_ray_tracing? **DISCUSSION** :The following is a
//! summary of the main functional differences between
//!VK_KHR_acceleration_structure and VK_NV_ray_tracing:
//! - added acceleration structure serialization / deserialization
//!   (`VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`,
//!   `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR`,
//!   [`CmdCopyAccelerationStructureToMemoryKHR`], [`CmdCopyMemoryToAccelerationStructureKHR`])
//! - document [inactive primitives and instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-inactive-prims)
//! - added [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure
//! - added indirect and batched acceleration structure builds
//!   ([`CmdBuildAccelerationStructuresIndirectKHR`])
//! - added [host acceleration structure](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#host-acceleration-structure)
//!   commands
//! - reworked geometry structures so they could be better shared between device, host, and indirect
//!   builds
//! - explicitly made [`AccelerationStructureKHR`] use device addresses
//! - added acceleration structure compatibility check function
//!   ([`GetDeviceAccelerationStructureCompatibilityKHR`])
//! - add parameter for requesting memory requirements for host and/or device build
//! - added format feature for acceleration structure build vertex formats
//!   (`VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`)
//!(2) Can you give a more detailed comparision of differences and similarities
//!between VK_NV_ray_tracing and VK_KHR_acceleration_structure? **DISCUSSION** :The following is a
//! more detailed comparision of which commands, structures,
//!and enums are aliased, changed, or removed.
//! - Aliased functionality — enums, structures, and commands that are considered equivalent:  -
//!   [`GeometryTypeNV`] ↔ [`GeometryTypeKHR`]  - [`AccelerationStructureTypeNV`] ↔
//!   [`AccelerationStructureTypeKHR`]  - [`CopyAccelerationStructureModeNV`] ↔
//!   [`CopyAccelerationStructureModeKHR`]  - [`GeometryFlagsNV`] ↔ [`GeometryFlagsKHR`]  -
//!   [`GeometryFlagBitsNV`] ↔ [`GeometryFlagBitsKHR`]  - [`GeometryInstanceFlagsNV`] ↔
//!   [`GeometryInstanceFlagsKHR`]  - [`GeometryInstanceFlagBitsNV`] ↔
//!   [`GeometryInstanceFlagBitsKHR`]  - [`BuildAccelerationStructureFlagsNV`] ↔
//!   [`BuildAccelerationStructureFlagsKHR`]  - [`BuildAccelerationStructureFlagBitsNV`] ↔
//!   [`BuildAccelerationStructureFlagBitsKHR`]  - [`TransformMatrixNV`] ↔ [`TransformMatrixKHR`]
//!   (added to VK_NV_ray_tracing for descriptive purposes)  - [`AabbPositionsNV`] ↔
//!   [`AabbPositionsKHR`] (added to VK_NV_ray_tracing for descriptive purposes)  -
//!   [`AccelerationStructureInstanceNV`] ↔ [`AccelerationStructureInstanceKHR`] (added to
//!   VK_NV_ray_tracing for descriptive purposes)
//! - Changed enums, structures, and commands:  - renamed
//!   `VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV` →
//!   `VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR` in [`GeometryInstanceFlagBitsKHR`]
//!   - [`GeometryTrianglesNV`] → [`AccelerationStructureGeometryTrianglesDataKHR`] (device or host
//!   address instead of buffer+offset)  - [`GeometryAabbNV`] →
//!   [`AccelerationStructureGeometryAabbsDataKHR`] (device or host address instead of
//!   buffer+offset)  - [`GeometryDataNV`] → [`AccelerationStructureGeometryDataKHR`] (union of
//!   triangle/aabbs/instances)  - [`GeometryNV`] → [`AccelerationStructureGeometryKHR`] (changed
//!   type of geometry)  - [`AccelerationStructureCreateInfoNV`] →
//!   [`AccelerationStructureCreateInfoKHR`] (reshuffle geometry layout/information)  -
//!   [`PhysicalDeviceRayTracingPropertiesNV`] →
//!   [`PhysicalDeviceAccelerationStructurePropertiesKHR`] (for acceleration structure properties,
//!   renamed `maxTriangleCount` to `maxPrimitiveCount`, added per stage and update after bind
//!   limits) and [`PhysicalDeviceRayTracingPipelinePropertiesKHR`] (for ray tracing pipeline
//!   properties)  - [`AccelerationStructureMemoryRequirementsInfoNV`] (deleted - replaced by
//!   allocating on top of [`Buffer`])  - [`WriteDescriptorSetAccelerationStructureNV`] →
//!   [`WriteDescriptorSetAccelerationStructureKHR`] (different acceleration structure type)  -
//!   [`CreateAccelerationStructureNV`] → [`CreateAccelerationStructureKHR`] (device address,
//!   different geometry layout/information)  - [`GetAccelerationStructureMemoryRequirementsNV`]
//!   (deleted - replaced by allocating on top of [`Buffer`])  - [`CmdBuildAccelerationStructureNV`]
//!   → [`CmdBuildAccelerationStructuresKHR`] (params moved to structs, layout differences)  -
//!   [`CmdCopyAccelerationStructureNV`] → [`CmdCopyAccelerationStructureKHR`] (params to struct,
//!   extendable)  - [`GetAccelerationStructureHandleNV`] →
//!   [`GetAccelerationStructureDeviceAddressKHR`] (device address instead of handle)  -
//!   [`AccelerationStructureMemoryRequirementsTypeNV`] → size queries for scratch space moved to
//!   [`GetAccelerationStructureBuildSizesKHR`]  - [`DestroyAccelerationStructureNV`] →
//!   [`DestroyAccelerationStructureKHR`] (different acceleration structure types)  -
//!   [`CmdWriteAccelerationStructuresPropertiesNV`] →
//!   [`CmdWriteAccelerationStructuresPropertiesKHR`] (different acceleration structure types)
//! - Added enums, structures and commands:  - `VK_GEOMETRY_TYPE_INSTANCES_KHR` to
//!   [`GeometryTypeKHR`] enum  - `VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`,
//!   `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR` to [`CopyAccelerationStructureModeKHR`]
//!   enum  - [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure  -
//!   [`AccelerationStructureBuildTypeKHR`] enum  - [`BuildAccelerationStructureModeKHR`] enum  -
//!   [`DeviceOrHostAddressKHR`] and [`DeviceOrHostAddressConstKHR`] unions  -
//!   [`AccelerationStructureBuildRangeInfoKHR`] struct  -
//!   [`AccelerationStructureGeometryInstancesDataKHR`] struct  -
//!   [`AccelerationStructureDeviceAddressInfoKHR`] struct  -
//!   [`AccelerationStructureVersionInfoKHR`] struct  - [`StridedDeviceAddressRegionKHR`] struct  -
//!   [`CopyAccelerationStructureToMemoryInfoKHR`] struct  -
//!   [`CopyMemoryToAccelerationStructureInfoKHR`] struct  - [`CopyAccelerationStructureInfoKHR`]
//!   struct  - [`BuildAccelerationStructuresKHR`] command (host build)  -
//!   [`CopyAccelerationStructureKHR`] command (host copy)  -
//!   [`CopyAccelerationStructureToMemoryKHR`] (host serialize)  -
//!   [`CopyMemoryToAccelerationStructureKHR`] (host deserialize)  -
//!   [`WriteAccelerationStructuresPropertiesKHR`] (host properties)  -
//!   [`CmdCopyAccelerationStructureToMemoryKHR`] (device serialize)  -
//!   [`CmdCopyMemoryToAccelerationStructureKHR`] (device deserialize)  -
//!   [`GetDeviceAccelerationStructureCompatibilityKHR`] (serialization)
//!(3) What are the changes between the public provisional (VK_KHR_ray_tracing
//!v8) release and the internal provisional (VK_KHR_ray_tracing v9) release?
//! - added `geometryFlags` to `VkAccelerationStructureCreateGeometryTypeInfoKHR` (later reworked to
//!   obsolete this)
//! - added `minAccelerationStructureScratchOffsetAlignment` property to
//!   VkPhysicalDeviceRayTracingPropertiesKHR
//! - fix naming and return enum from [`GetDeviceAccelerationStructureCompatibilityKHR`]  - renamed
//!   `VkAccelerationStructureVersionKHR` to [`AccelerationStructureVersionInfoKHR`]  - renamed
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_KHR` to
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`  - removed
//!   `VK_ERROR_INCOMPATIBLE_VERSION_KHR`  - added [`AccelerationStructureCompatibilityKHR`] enum  -
//!   remove return value from [`GetDeviceAccelerationStructureCompatibilityKHR`] and added return
//!   enum parameter
//! - Require Vulkan 1.1
//! - added creation time capture and replay flags  - added
//!   [`AccelerationStructureCreateFlagBitsKHR`] and [`AccelerationStructureCreateFlagsKHR`]  -
//!   renamed the `flags` member of [`AccelerationStructureCreateInfoKHR`] to `buildFlags` (later
//!   removed) and added the `createFlags` member
//! - change [`CmdBuildAccelerationStructuresIndirectKHR`] to use buffer device address for indirect
//!   parameter
//! - make `[`VK_KHR_deferred_host_operations`]` an interaction instead of a required extension
//!   (later went back on this)
//! - renamed `VkAccelerationStructureBuildOffsetInfoKHR` to
//!   [`AccelerationStructureBuildRangeInfoKHR`]  - renamed the `ppOffsetInfos` parameter of
//!   [`CmdBuildAccelerationStructuresKHR`] to `ppBuildRangeInfos`
//! - Re-unify geometry description between build and create  - remove
//!   `VkAccelerationStructureCreateGeometryTypeInfoKHR` and
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_GEOMETRY_TYPE_INFO_KHR`  - added
//!   `VkAccelerationStructureCreateSizeInfoKHR` structure (later removed)  - change type of the
//!   `pGeometryInfos` member of [`AccelerationStructureCreateInfoKHR`] from
//!   `VkAccelerationStructureCreateGeometryTypeInfoKHR` to [`AccelerationStructureGeometryKHR`]
//!   (later removed)  - added `pCreateSizeInfos` member to [`AccelerationStructureCreateInfoKHR`]
//!   (later removed)
//! - Fix ppGeometries ambiguity, add pGeometries  - remove `geometryArrayOfPointers` member of
//!   VkAccelerationStructureBuildGeometryInfoKHR  - disambiguate two meanings of `ppGeometries` by
//!   explicitly adding `pGeometries` to the [`AccelerationStructureBuildGeometryInfoKHR`] structure
//!   and require one of them be `NULL`
//! - added [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor)
//!   support for acceleration structures
//! - changed the `update` member of [`AccelerationStructureBuildGeometryInfoKHR`] from a bool to
//!   the `mode`[`BuildAccelerationStructureModeKHR`] enum which allows future extensibility in
//!   update types
//! - Clarify deferred host ops for pipeline creation  - [`DeferredOperationKHR`] is now a top-level
//!   parameter for [`BuildAccelerationStructuresKHR`], [`CreateRayTracingPipelinesKHR`],
//!   [`CopyAccelerationStructureToMemoryKHR`], [`CopyAccelerationStructureKHR`], and
//!   [`CopyMemoryToAccelerationStructureKHR`]  - removed `VkDeferredOperationInfoKHR` structure  -
//!   change deferred host creation/return parameter behavior such that the implementation can
//!   modify such parameters until the deferred host operation completes  -
//!   `[`VK_KHR_deferred_host_operations`]` is required again
//! - Change acceleration structure build to always be sized  - de-alias
//!   [`AccelerationStructureMemoryRequirementsTypeNV`] and
//!   `VkAccelerationStructureMemoryRequirementsTypeKHR`, and remove
//!   `VkAccelerationStructureMemoryRequirementsTypeKHR`  - add
//!   [`GetAccelerationStructureBuildSizesKHR`] command and
//!   [`AccelerationStructureBuildSizesInfoKHR`] structure and
//!   `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR` enum to query sizes for
//!   acceleration structures and scratch storage  - move size queries for scratch space to
//!   [`GetAccelerationStructureBuildSizesKHR`]  - remove `compactedSize`, `buildFlags`,
//!   `maxGeometryCount`, `pGeometryInfos`, `pCreateSizeInfos` members of
//!   [`AccelerationStructureCreateInfoKHR`] and add the `size` member  - add `maxVertex` member to
//!   [`AccelerationStructureGeometryTrianglesDataKHR`] structure  - remove
//!   `VkAccelerationStructureCreateSizeInfoKHR` structure
//!(4) What are the changes between the internal provisional
//!(VK_KHR_ray_tracing v9) release and the final (VK_KHR_acceleration_structure
//!v11) release?
//! - refactor VK_KHR_ray_tracing into 3 extensions, enabling implementation flexibility and
//!   decoupling ray query support from ray pipelines:  - `[`VK_KHR_acceleration_structure`]` (for
//!   acceleration structure operations)  - `[`VK_KHR_ray_tracing_pipeline`]` (for ray tracing
//!   pipeline and shader stages)  - `[`VK_KHR_ray_query`]` (for ray queries in existing shader
//!   stages)
//! - clarify buffer usage flags for ray tracing  - `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` is left
//!   alone in `[`VK_NV_ray_tracing`]` (required on `scratch` and `instanceData`)  -
//!   `VK_BUFFER_USAGE_SHADER_BINDING_TABLE_BIT_KHR` is added as an alias of
//!   `VK_BUFFER_USAGE_RAY_TRACING_BIT_NV` in `[`VK_KHR_ray_tracing_pipeline`]` and is required on
//!   shader binding table buffers  -
//!   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_BUILD_INPUT_READ_ONLY_BIT_KHR` is added in
//!   `[`VK_KHR_acceleration_structure`]` for all vertex, index, transform, aabb, and instance
//!   buffer data referenced by device build commands  - `VK_BUFFER_USAGE_STORAGE_BUFFER_BIT` is
//!   used for `scratchData`
//! - add max primitive counts (`ppMaxPrimitiveCounts`) to
//!   [`CmdBuildAccelerationStructuresIndirectKHR`]
//! - Allocate acceleration structures from `VkBuffers` and add a mode to constrain the device
//!   address  - de-alias [`BindAccelerationStructureMemoryInfoNV`] and
//!   [`BindAccelerationStructureMemoryNV`], and remove `VkBindAccelerationStructureMemoryInfoKHR`,
//!   `VkAccelerationStructureMemoryRequirementsInfoKHR`, and
//!   `vkGetAccelerationStructureMemoryRequirementsKHR`  - acceleration structures now take a
//!   [`Buffer`] and offset at creation time for memory placement  - add a new
//!   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR` buffer usage for such buffers  - add
//!   a new `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR` acceleration structure type for layering
//! - move `VK_GEOMETRY_TYPE_INSTANCES_KHR` to main enum instead of being added via extension
//! - make build commands more consistent - all now build multiple acceleration structures and are
//!   named plurally ([`CmdBuildAccelerationStructuresIndirectKHR`],
//!   [`CmdBuildAccelerationStructuresKHR`], [`BuildAccelerationStructuresKHR`])
//! - add interactions with `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` for
//!   acceleration structures, including a new feature
//!   (`descriptorBindingAccelerationStructureUpdateAfterBind`) and 3 new properties
//!   (`maxPerStageDescriptorAccelerationStructures`,
//!   `maxPerStageDescriptorUpdateAfterBindAccelerationStructures`,
//!   `maxDescriptorSetUpdateAfterBindAccelerationStructures`)
//! - extension is no longer provisional
//! - define synchronization requirements for builds, traces, and copies
//! - define synchronization requirements for AS build inputs and indirect build buffer
//!(5) What is `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR` for? **RESOLVED** : It is primarily
//! intended for API layering.
//!In DXR, the acceleration structure is basically just a buffer in a special
//!layout, and you do not know at creation time whether it will be used as a
//!top or bottom level acceleration structure.
//!We thus added a generic acceleration structure type whose type is unknown at
//!creation time, but is specified at build time instead.
//!Applications which are written directly for Vulkan should not use it.
//!# Version History
//! - Revision 1, 2019-12-05 (Members of the Vulkan Ray Tracing TSG)  - Internal revisions (forked
//!   from VK_NV_ray_tracing)
//! - Revision 2, 2019-12-20 (Daniel Koch, Eric Werness)  - Add const version of DeviceOrHostAddress
//!   (!3515)  - Add VU to clarify that only handles in the current pipeline are valid (!3518)  -
//!   Restore some missing VUs and add in-place update language (#1902, !3522)  - rename
//!   VkAccelerationStructureInstanceKHR member from accelerationStructure to
//!   accelerationStructureReference to better match its type (!3523)  - Allow
//!   VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS for pipeline creation if shader group handles cannot
//!   be reused (!3523)  - update documentation for the VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS
//!   error code and add missing documentation for new return codes from
//!   VK_KHR_deferred_host_operations (!3523)  - list new query types for VK_KHR_ray_tracing (!3523)
//!   - Fix VU statements for VkAccelerationStructureGeometryKHR referring to correct union members
//!   and update to use more current wording (!3523)
//! - Revision 3, 2020-01-10 (Daniel Koch, Jon Leech, Christoph Kubisch)  - Fix 'instance of' and
//!   'that/which contains/defines' markup issues (!3528)  - factor out VK_KHR_pipeline_library as
//!   stand-alone extension (!3540)  - Resolve Vulkan-hpp issues (!3543)  - add missing require for
//!   VkGeometryInstanceFlagsKHR  - de-alias VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_NV
//!   since the KHR structure is no longer equivalent  - add len to pDataSize attribute for
//!   vkWriteAccelerationStructuresPropertiesKHR
//! - Revision 4, 2020-01-23 (Daniel Koch, Eric Werness)  - Improve
//!   vkWriteAccelerationStructuresPropertiesKHR, add return value and VUs (#1947)  - Clarify
//!   language to allow multiple raygen shaders (#1959)  - Various editorial feedback (!3556)  - Add
//!   language to help deal with looped self-intersecting fans (#1901)  - Change
//!   vkCmdTraceRays{Indirect}KHR args to pointers (!3559)  - Add scratch address validation
//!   language (#1941, !3551)  - Fix definition and add hierarchy information for shader call scope
//!   (#1977, !3571)
//! - Revision 5, 2020-02-04 (Eric Werness, Jeff Bolz, Daniel Koch)  - remove vestigial
//!   accelerationStructureUUID (!3582)  - update definition of repack instructions and improve
//!   memory model interactions (#1910, #1913, !3584)  - Fix wrong sType for
//!   VkPhysicalDeviceRayTracingFeaturesKHR (#1988)  - Use provisional SPIR-V capabilities (#1987)
//!   - require rayTraversalPrimitiveCulling if rayQuery is supported (#1927)  - Miss shaders do not
//!   have object parameters (!3592)  - Fix missing required types in XML (!3592)  - clarify
//!   matching conditions for update (!3592)  - add goal that host and device builds be similar
//!   (!3592)  - clarify that `maxPrimitiveCount` limit should apply to triangles and AABBs (!3592)
//!   - Require alignment for instance arrayOfPointers (!3592)  - Zero is a valid value for instance
//!   flags (!3592)  - Add some alignment VUs that got lost in refactoring (!3592)  - Recommend TMin
//!   epsilon rather than culling (!3592)  - Get angle from dot product not cross product (!3592)  -
//!   Clarify that AH can access the payload and attributes (!3592)  - Match DXR behavior for
//!   inactive primitive definition (!3592)  - Use a more generic term than degenerate for inactive
//!   to avoid confusion (!3592)
//! - Revision 6, 2020-02-20 (Daniel Koch)  - fix some dangling NV references (#1996)  - rename
//!   VkCmdTraceRaysIndirectCommandKHR to VkTraceRaysIndirectCommandKHR (!3607)  - update
//!   contributor list (!3611)  - use uint64_t instead of VkAccelerationStructureReferenceKHR in
//!   VkAccelerationStructureInstanceKHR (#2004)
//! - Revision 7, 2020-02-28 (Tobias Hector)  - remove HitTKHR SPIR-V builtin
//!   (spirv/spirv-extensions#7)
//! - Revision 8, 2020-03-06 (Tobias Hector, Dae Kim, Daniel Koch, Jeff Bolz, Eric Werness)  -
//!   explicitly state that Tmax is updated when new closest intersection is accepted (#2020,!3536)
//!   - Made references to min and max t values consistent (!3644)  - finish enumerating differences
//!   relative to VK_NV_ray_tracing in issues (1) and (2) (#1974,!3642)  - fix formatting in some
//!   math equations (!3642)  - Restrict the Hit Kind operand of `OpReportIntersectionKHR` to 7-bits
//!   (spirv/spirv-extensions#8,!3646)  - Say ray tracing ' **should** ' be watertight (#2008,!3631)
//!   - Clarify memory requirements for ray tracing buffers (#2005,!3649)  - Add callable size
//!   limits (#1997,!3652)
//! - Revision 9, 2020-04-15 (Eric Werness, Daniel Koch, Tobias Hector, Joshua Barczak)  - Add
//!   geometry flags to acceleration structure creation (!3672)  - add build scratch memory
//!   alignment (minAccelerationStructureScratchOffsetAlignment) (#2065,!3725)  - fix naming and
//!   return enum from vkGetDeviceAccelerationStructureCompatibilityKHR (#2051,!3726)  - require
//!   SPIR-V 1.4 (#2096,!3777)  - added creation time capture/replay flags (#2104,!3774)  - require
//!   Vulkan 1.1 (#2133,!3806)  - use device addresses instead of VkBuffers for ray tracing commands
//!   (#2074,!3815)  - add interactions with Vulkan 1.2 and VK_KHR_vulkan_memory_model (#2133,!3830)
//!   - make VK_KHR_pipeline_library an interaction instead of required (#2045,#2108,!3830)  - make
//!   VK_KHR_deferred_host_operations an interaction instead of required (#2045,!3830)  - removed
//!   maxCallableSize and added explicit stack size management for ray pipelines
//!   (#1997,!3817,!3772,!3844)  - improved documentation for VkAccelerationStructureVersionInfoKHR
//!   (#2135,3835)  - rename VkAccelerationStructureBuildOffsetInfoKHR to
//!   VkAccelerationStructureBuildRangeInfoKHR (#2058,!3754)  - Re-unify geometry description
//!   between build and create (!3754)  - Fix ppGeometries ambiguity, add pGeometries (#2032,!3811)
//!   - add interactions with VK_EXT_robustness2 and allow nullDescriptor support for acceleration
//!   structures (#1920,!3848)  - added future extensibility for AS updates (#2114,!3849)  - Fix VU
//!   for dispatchrays and add a limit on the size of the full grid (#2160,!3851)  - Add
//!   shaderGroupHandleAlignment property (#2180,!3875)  - Clarify deferred host ops for pipeline
//!   creation (#2067,!3813)  - Change acceleration structure build to always be sized
//!   (#2131,#2197,#2198,!3854,!3883,!3880)
//! - Revision 10, 2020-07-03 (Mathieu Robart, Daniel Koch, Eric Werness, Tobias Hector)  -
//!   Decomposition of the specification, from VK_KHR_ray_tracing to VK_KHR_acceleration_structure
//!   (#1918,!3912)  - clarify buffer usage flags for ray tracing (#2181,!3939)  - add max primitive
//!   counts to build indirect command (#2233,!3944)  - Allocate acceleration structures from
//!   VkBuffers and add a mode to constrain the device address (#2131,!3936)  - Move
//!   VK_GEOMETRY_TYPE_INSTANCES_KHR to main enum (#2243,!3952)  - make build commands more
//!   consistent (#2247,!3958)  - add interactions with UPDATE_AFTER_BIND (#2128,!3986)  - correct
//!   and expand build command VUs (!4020)  - fix copy command VUs (!4018)  - added various
//!   alignment requirements (#2229,!3943)  - fix valid usage for arrays of geometryCount items
//!   (#2198,!4010)  - define what is allowed to change on RTAS updates and relevant VUs
//!   (#2177,!3961)
//! - Revision 11, 2020-11-12 (Eric Werness, Josh Barczak, Daniel Koch, Tobias Hector)  - de-alias
//!   NV and KHR acceleration structure types and associated commands (#2271,!4035)  - specify
//!   alignment for host copy commands (#2273,!4037)  - document
//!   `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR`  - specify that acceleration
//!   structures are non-linear (#2289,!4068)  - add several missing VUs for strides, vertexFormat,
//!   and indexType (#2315,!4069)  - restore VUs for VkAccelerationStructureBuildGeometryInfoKHR
//!   (#2337,!4098)  - ban multi-instance memory for host operations (#2324,!4102)  - allow
//!   dstAccelerationStructure to be null for vkGetAccelerationStructureBuildSizesKHR (#2330,!4111)
//!   - more build VU cleanup (#2138,#4130)  - specify host endianness for AS serialization
//!   (#2261,!4136)  - add invertible transform matrix VU (#1710,!4140)  - require geometryCount to
//!   be 1 for TLAS builds (!4145)  - improved validity conditions for build addresses (#4142)  -
//!   add single statement SPIR-V VUs, build limit VUs (!4158)  - document limits for vertex and
//!   aabb strides (#2390,!4184)  - specify that
//!   `VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR` applies to AS copies (#2382,#4173)  -
//!   define sync for AS build inputs and indirect buffer (#2407,!4208)
//! - Revision 12, 2021-08-06 (Samuel Bourasseau)  - rename
//!   VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR to
//!   VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR (keep previous as alias).  - Clarify
//!   description and add note.
//! - Revision 13, 2021-09-30 (Jon Leech)  - Add interaction with `[`VK_KHR_format_feature_flags2`]`
//!   to `vk.xml`
//!# Other info
//! * 2021-09-30
//! * - Samuel Bourasseau, Adobe  - Matthäus Chajdas, AMD  - Greg Grebe, AMD  - Nicolai Hähnle, AMD
//!   - Tobias Hector, AMD  - Dave Oldcorn, AMD  - Skyler Saleh, AMD  - Mathieu Robart, Arm  -
//!   Marius Bjorge, Arm  - Tom Olson, Arm  - Sebastian Tafuri, EA  - Henrik Rydgard, Embark  - Juan
//!   Cañada, Epic Games  - Patrick Kelly, Epic Games  - Yuriy O’Donnell, Epic Games  - Michael
//!   Doggett, Facebook/Oculus  - Ricardo Garcia, Igalia  - Andrew Garrard, Imagination  - Don
//!   Scorgie, Imagination  - Dae Kim, Imagination  - Joshua Barczak, Intel  - Slawek Grajewski,
//!   Intel  - Jeff Bolz, NVIDIA  - Pascal Gautron, NVIDIA  - Daniel Koch, NVIDIA  - Christoph
//!   Kubisch, NVIDIA  - Ashwin Lele, NVIDIA  - Robert Stepinski, NVIDIA  - Martin Stich, NVIDIA  -
//!   Nuno Subtil, NVIDIA  - Eric Werness, NVIDIA  - Jon Leech, Khronos  - Jeroen van Schijndel,
//!   OTOY  - Juul Joosten, OTOY  - Alex Bourd, Qualcomm  - Roman Larionov, Qualcomm  - David
//!   McAllister, Qualcomm  - Lewis Gordon, Samsung  - Ralph Potter, Samsung  - Jasper Bekkers,
//!   Traverse Research  - Jesse Barker, Unity  - Baldur Karlsson, Valve
//!# Related
//! - [`AabbPositionsKHR`]
//! - [`AccelerationStructureBuildGeometryInfoKHR`]
//! - [`AccelerationStructureBuildRangeInfoKHR`]
//! - [`AccelerationStructureBuildSizesInfoKHR`]
//! - [`AccelerationStructureBuildTypeKHR`]
//! - [`AccelerationStructureCompatibilityKHR`]
//! - [`AccelerationStructureCreateFlagBitsKHR`]
//! - [`AccelerationStructureCreateFlagsKHR`]
//! - [`AccelerationStructureCreateInfoKHR`]
//! - [`AccelerationStructureDeviceAddressInfoKHR`]
//! - [`AccelerationStructureGeometryAabbsDataKHR`]
//! - [`AccelerationStructureGeometryDataKHR`]
//! - [`AccelerationStructureGeometryInstancesDataKHR`]
//! - [`AccelerationStructureGeometryKHR`]
//! - [`AccelerationStructureGeometryTrianglesDataKHR`]
//! - [`AccelerationStructureInstanceKHR`]
//! - [`AccelerationStructureKHR`]
//! - [`AccelerationStructureTypeKHR`]
//! - [`AccelerationStructureVersionInfoKHR`]
//! - [`BuildAccelerationStructureFlagBitsKHR`]
//! - [`BuildAccelerationStructureFlagsKHR`]
//! - [`BuildAccelerationStructureModeKHR`]
//! - [`CopyAccelerationStructureInfoKHR`]
//! - [`CopyAccelerationStructureModeKHR`]
//! - [`CopyAccelerationStructureToMemoryInfoKHR`]
//! - [`CopyMemoryToAccelerationStructureInfoKHR`]
//! - [`DeviceOrHostAddressConstKHR`]
//! - [`DeviceOrHostAddressKHR`]
//! - [`GeometryFlagBitsKHR`]
//! - [`GeometryFlagsKHR`]
//! - [`GeometryInstanceFlagBitsKHR`]
//! - [`GeometryInstanceFlagsKHR`]
//! - [`GeometryTypeKHR`]
//! - [`PhysicalDeviceAccelerationStructureFeaturesKHR`]
//! - [`PhysicalDeviceAccelerationStructurePropertiesKHR`]
//! - [`TransformMatrixKHR`]
//! - [`WriteDescriptorSetAccelerationStructureKHR`]
//! - [`BuildAccelerationStructuresKHR`]
//! - [`CmdBuildAccelerationStructuresIndirectKHR`]
//! - [`CmdBuildAccelerationStructuresKHR`]
//! - [`CmdCopyAccelerationStructureKHR`]
//! - [`CmdCopyAccelerationStructureToMemoryKHR`]
//! - [`CmdCopyMemoryToAccelerationStructureKHR`]
//! - [`CmdWriteAccelerationStructuresPropertiesKHR`]
//! - [`CopyAccelerationStructureKHR`]
//! - [`CopyAccelerationStructureToMemoryKHR`]
//! - [`CopyMemoryToAccelerationStructureKHR`]
//! - [`CreateAccelerationStructureKHR`]
//! - [`DestroyAccelerationStructureKHR`]
//! - [`GetAccelerationStructureBuildSizesKHR`]
//! - [`GetAccelerationStructureDeviceAddressKHR`]
//! - [`GetDeviceAccelerationStructureCompatibilityKHR`]
//! - [`WriteAccelerationStructuresPropertiesKHR`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Buffer, DeviceAddress, DeviceSize, Format, IndexType, StructureType,
};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{
    ffi::{c_void, CStr},
    iter::{Extend, FromIterator, IntoIterator},
    marker::PhantomData,
};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_SPEC_VERSION")]
pub const KHR_ACCELERATION_STRUCTURE_SPEC_VERSION: u32 = 13;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME")]
pub const KHR_ACCELERATION_STRUCTURE_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_acceleration_structure");
///[VkCopyAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureModeKHR.html) - Acceleration structure copy mode
///# C Specifications
///Possible values of `mode` specifying additional operations to perform
///during the copy, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkCopyAccelerationStructureModeKHR {
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR = 2,
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR = 3,
///  // Provided by VK_NV_ray_tracing
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_NV = VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_NV =
/// VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR,
///} VkCopyAccelerationStructureModeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkCopyAccelerationStructureModeKHR VkCopyAccelerationStructureModeNV;
///```
///# Description
/// - [`CopyAccelerationStructureModeCloneKhr`] creates a direct copy of the acceleration structure
///   specified in `src` into the one specified by `dst`. The `dst` acceleration structure  **must**
///   have been created with the same parameters as `src`. If `src` contains references to other
///   acceleration structures, `dst` will reference the same acceleration structures.
/// - [`CopyAccelerationStructureModeCompactKhr`] creates a more compact version of an acceleration
///   structure `src` into `dst`. The acceleration structure `dst` **must**  have been created with
///   a size at least as large as that returned by [`CmdWriteAccelerationStructuresPropertiesKHR`]
///   or [`WriteAccelerationStructuresPropertiesKHR`] after the build of the acceleration structure
///   specified by `src`. If `src` contains references to other acceleration structures, `dst` will
///   reference the same acceleration structures.
/// - [`CopyAccelerationStructureModeSerializeKhr`] serializes the acceleration structure to a
///   semi-opaque format which can be reloaded on a compatible implementation.
/// - [`CopyAccelerationStructureModeDeserializeKhr`] deserializes the semi-opaque serialization
///   format in the buffer to the acceleration structure.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`CopyAccelerationStructureInfoKHR`]
/// - [`CopyAccelerationStructureToMemoryInfoKHR`]
/// - [`CopyMemoryToAccelerationStructureInfoKHR`]
/// - [`CmdCopyAccelerationStructureNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyAccelerationStructureModeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum CopyAccelerationStructureModeKHR {
    ///[`CopyAccelerationStructureModeCloneKhr`] creates a direct
    ///copy of the acceleration structure specified in `src` into the one
    ///specified by `dst`.
    ///The `dst` acceleration structure  **must**  have been created with the
    ///same parameters as `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    CopyAccelerationStructureModeCloneKhr = 0,
    ///[`CopyAccelerationStructureModeCompactKhr`] creates a more
    ///compact version of an acceleration structure `src` into `dst`.
    ///The acceleration structure `dst` **must**  have been created with a size
    ///at least as large as that returned by
    ///[`CmdWriteAccelerationStructuresPropertiesKHR`]
    ///or [`WriteAccelerationStructuresPropertiesKHR`]
    ///after the build of the acceleration structure specified by `src`.
    ///If `src` contains references to other acceleration structures,
    ///`dst` will reference the same acceleration structures.
    CopyAccelerationStructureModeCompactKhr = 1,
    ///[`CopyAccelerationStructureModeSerializeKhr`] serializes the
    ///acceleration structure to a semi-opaque format which can be reloaded on
    ///a compatible implementation.
    CopyAccelerationStructureModeSerializeKhr = 2,
    ///[`CopyAccelerationStructureModeDeserializeKhr`] deserializes
    ///the semi-opaque serialization format in the buffer to the acceleration
    ///structure.
    CopyAccelerationStructureModeDeserializeKhr = 3,
}
impl const Default for CopyAccelerationStructureModeKHR {
    fn default() -> Self {
        Self::CopyAccelerationStructureModeCloneKhr
    }
}
impl CopyAccelerationStructureModeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkBuildAccelerationStructureModeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureModeKHR.html) - Enum specifying the type of build operation to perform
///# C Specifications
///The [`BuildAccelerationStructureModeKHR`] enumeration is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkBuildAccelerationStructureModeKHR {
///    VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0,
///    VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1,
///} VkBuildAccelerationStructureModeKHR;
///```
///# Description
/// - [`BuildAccelerationStructureModeBuildKhr`] specifies that the destination acceleration
///   structure will be built using the specified geometries.
/// - [`BuildAccelerationStructureModeUpdateKhr`] specifies that the destination acceleration
///   structure will be built using data in a source acceleration structure, updated by the
///   specified geometries.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBuildAccelerationStructureModeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum BuildAccelerationStructureModeKHR {
    ///[`BuildAccelerationStructureModeBuildKhr`] specifies that the
    ///destination acceleration structure will be built using the specified
    ///geometries.
    BuildAccelerationStructureModeBuildKhr = 0,
    ///[`BuildAccelerationStructureModeUpdateKhr`] specifies that the
    ///destination acceleration structure will be built using data in a source
    ///acceleration structure, updated by the specified geometries.
    BuildAccelerationStructureModeUpdateKhr = 1,
}
impl const Default for BuildAccelerationStructureModeKHR {
    fn default() -> Self {
        Self::BuildAccelerationStructureModeBuildKhr
    }
}
impl BuildAccelerationStructureModeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkAccelerationStructureTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureTypeKHR.html) - Type of acceleration structure
///# C Specifications
///Values which  **can**  be set in
///[`AccelerationStructureCreateInfoKHR::type_`]
///or
///[`AccelerationStructureInfoNV::type_`]
///specifying the type of acceleration structure, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureTypeKHR {
///    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1,
///    VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_NV = VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_NV =
/// VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR,
///} VkAccelerationStructureTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAccelerationStructureTypeKHR VkAccelerationStructureTypeNV;
///```
///# Description
/// - [`AccelerationStructureTypeTopLevelKhr`] is a top-level acceleration structure containing
///   instance data referring to bottom-level acceleration structures.
/// - [`AccelerationStructureTypeBottomLevelKhr`] is a bottom-level acceleration structure
///   containing the AABBs or geometry to be intersected.
/// - [`AccelerationStructureTypeGenericKhr`] is an acceleration structure whose type is determined
///   at build time used for special circumstances.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`AccelerationStructureCreateInfoKHR`]
/// - [`AccelerationStructureInfoNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum AccelerationStructureTypeKHR {
    ///[`AccelerationStructureTypeTopLevelKhr`] is a top-level
    ///acceleration structure containing instance data referring to
    ///bottom-level acceleration structures.
    AccelerationStructureTypeTopLevelKhr = 0,
    ///[`AccelerationStructureTypeBottomLevelKhr`] is a bottom-level
    ///acceleration structure containing the AABBs or geometry to be
    ///intersected.
    AccelerationStructureTypeBottomLevelKhr = 1,
    ///[`AccelerationStructureTypeGenericKhr`] is an acceleration
    ///structure whose type is determined at build time used for special
    ///circumstances.
    AccelerationStructureTypeGenericKhr = 2,
}
impl const Default for AccelerationStructureTypeKHR {
    fn default() -> Self {
        Self::AccelerationStructureTypeTopLevelKhr
    }
}
impl AccelerationStructureTypeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkGeometryTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryTypeKHR.html) - Enum specifying which type of geometry is provided
///# C Specifications
///Geometry types are specified by [`GeometryTypeKHR`], which takes values:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryTypeKHR {
///    VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0,
///    VK_GEOMETRY_TYPE_AABBS_KHR = 1,
///    VK_GEOMETRY_TYPE_INSTANCES_KHR = 2,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_TYPE_TRIANGLES_NV = VK_GEOMETRY_TYPE_TRIANGLES_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_TYPE_AABBS_NV = VK_GEOMETRY_TYPE_AABBS_KHR,
///} VkGeometryTypeKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryTypeKHR VkGeometryTypeNV;
///```
///# Description
/// - [`GeometryTypeTrianglesKhr`] specifies a geometry type consisting of triangles.
/// - [`GeometryTypeAabbsKhr`] specifies a geometry type consisting of axis-aligned bounding boxes.
/// - [`GeometryTypeInstancesKhr`] specifies a geometry type consisting of acceleration structure
///   instances.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureGeometryKHR`]
/// - [`GeometryNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum GeometryTypeKHR {
    ///[`GeometryTypeTrianglesKhr`] specifies a geometry type
    ///consisting of triangles.
    GeometryTypeTrianglesKhr = 0,
    ///[`GeometryTypeAabbsKhr`] specifies a geometry type consisting of
    ///axis-aligned bounding boxes.
    GeometryTypeAabbsKhr = 1,
    ///[`GeometryTypeInstancesKhr`] specifies a geometry type
    ///consisting of acceleration structure instances.
    GeometryTypeInstancesKhr = 2,
}
impl const Default for GeometryTypeKHR {
    fn default() -> Self {
        Self::GeometryTypeTrianglesKhr
    }
}
impl GeometryTypeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkAccelerationStructureBuildTypeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildTypeKHR.html) - Acceleration structure build type
///# C Specifications
///Possible values of `buildType` in
///[`GetAccelerationStructureBuildSizesKHR`] are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureBuildTypeKHR {
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1,
///    VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2,
///} VkAccelerationStructureBuildTypeKHR;
///```
///# Description
/// - [`AccelerationStructureBuildTypeHostKhr`] requests the memory requirement for operations
///   performed by the host.
/// - [`AccelerationStructureBuildTypeDeviceKhr`] requests the memory requirement for operations
///   performed by the device.
/// - [`AccelerationStructureBuildTypeHostOrDeviceKhr`] requests the memory requirement for
///   operations performed by either the host, or the device.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureBuildTypeKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum AccelerationStructureBuildTypeKHR {
    ///[`AccelerationStructureBuildTypeHostKhr`] requests the memory
    ///requirement for operations performed by the host.
    AccelerationStructureBuildTypeHostKhr = 0,
    ///[`AccelerationStructureBuildTypeDeviceKhr`] requests the
    ///memory requirement for operations performed by the device.
    AccelerationStructureBuildTypeDeviceKhr = 1,
    ///[`AccelerationStructureBuildTypeHostOrDeviceKhr`] requests
    ///the memory requirement for operations performed by either the host, or
    ///the device.
    AccelerationStructureBuildTypeHostOrDeviceKhr = 2,
}
impl const Default for AccelerationStructureBuildTypeKHR {
    fn default() -> Self {
        Self::AccelerationStructureBuildTypeHostKhr
    }
}
impl AccelerationStructureBuildTypeKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkAccelerationStructureCompatibilityKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCompatibilityKHR.html) - Acceleration structure compatibility
///# C Specifications
///Possible values of `pCompatibility` returned by
///[`GetDeviceAccelerationStructureCompatibilityKHR`] are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureCompatibilityKHR {
///    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR = 0,
///    VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR = 1,
///} VkAccelerationStructureCompatibilityKHR;
///```
///# Description
/// - [`AccelerationStructureCompatibilityCompatibleKhr`] if the `pVersionData` version acceleration
///   structure is compatible with `device`.
/// - [`AccelerationStructureCompatibilityIncompatibleKhr`] if the `pVersionData` version
///   acceleration structure is not compatible with `device`.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`GetDeviceAccelerationStructureCompatibilityKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCompatibilityKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(i32)]
pub enum AccelerationStructureCompatibilityKHR {
    ///[`AccelerationStructureCompatibilityCompatibleKhr`] if the
    ///`pVersionData` version acceleration structure is compatible with
    ///`device`.
    AccelerationStructureCompatibilityCompatibleKhr = 0,
    ///[`AccelerationStructureCompatibilityIncompatibleKhr`] if the
    ///`pVersionData` version acceleration structure is not compatible with
    ///`device`.
    AccelerationStructureCompatibilityIncompatibleKhr = 1,
}
impl const Default for AccelerationStructureCompatibilityKHR {
    fn default() -> Self {
        Self::AccelerationStructureCompatibilityCompatibleKhr
    }
}
impl AccelerationStructureCompatibilityKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
    }
}
///[VkGeometryInstanceFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) - Instance flag bits
///# C Specifications
///Possible values of `flags` in the instance modifying the behavior of
///that instance are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryInstanceFlagBitsKHR {
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR = 0x00000002,
///    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 0x00000004,
///    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR,
///} VkGeometryInstanceFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryInstanceFlagBitsKHR VkGeometryInstanceFlagBitsNV;
///```
///# Description
/// - [`GeometryInstanceTriangleFacingCullDisableKhr`] disables face culling for this instance.
/// - [`GeometryInstanceTriangleFlipFacingKhr`] indicates that the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face)
///   for geometry in this instance is inverted. Because the facing is determined in object space,
///   an instance transform does not change the winding, but a geometry transform does.
/// - [`GeometryInstanceForceOpaqueKhr`] causes this instance to act as though
///   `VK_GEOMETRY_OPAQUE_BIT_KHR` were specified on all geometries referenced by this instance.
///   This behavior  **can**  be overridden by the SPIR-V `NoOpaqueKHR` ray flag.
/// - [`GeometryInstanceForceNoOpaqueKhr`] causes this instance to act as though
///   `VK_GEOMETRY_OPAQUE_BIT_KHR` were not specified on all geometries referenced by this instance.
///   This behavior  **can**  be overridden by the SPIR-V `OpaqueKHR` ray flag.
///[`GeometryInstanceForceNoOpaqueKhr`] and
///[`GeometryInstanceForceOpaqueKhr`] **must**  not be used in the
///same flag.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`GeometryInstanceFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryInstanceFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum GeometryInstanceFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`GeometryInstanceTriangleFacingCullDisableKhr`] disables
    ///face culling for this instance.
    GeometryInstanceTriangleFacingCullDisableKhr = 1,
    ///[`GeometryInstanceTriangleFlipFacingKhr`] indicates that
    ///the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face) for geometry in
    ///this instance is inverted.
    ///Because the facing is determined in object space, an instance transform
    ///does not change the winding, but a geometry transform does.
    GeometryInstanceTriangleFlipFacingKhr = 2,
    ///[`GeometryInstanceForceOpaqueKhr`] causes this instance to
    ///act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were specified on all
    ///geometries referenced by this instance.
    ///This behavior  **can**  be overridden by the SPIR-V `NoOpaqueKHR` ray
    ///flag.
    GeometryInstanceForceOpaqueKhr = 4,
    ///[`GeometryInstanceForceNoOpaqueKhr`] causes this instance
    ///to act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were not specified on
    ///all geometries referenced by this instance.
    ///This behavior  **can**  be overridden by the SPIR-V `OpaqueKHR` ray flag.
    GeometryInstanceForceNoOpaqueKhr = 8,
}
impl const Default for GeometryInstanceFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl GeometryInstanceFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkGeometryFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) - Bitmask specifying additional parameters for a geometry
///# C Specifications
///Bits specifying additional parameters for geometries in acceleration
///structure builds, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryFlagBitsKHR {
///    VK_GEOMETRY_OPAQUE_BIT_KHR = 0x00000001,
///    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_OPAQUE_BIT_NV = VK_GEOMETRY_OPAQUE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV =
/// VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR,
///} VkGeometryFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryFlagBitsKHR VkGeometryFlagBitsNV;
///```
///# Description
/// - [`GeometryOpaqueKhr`] indicates that this geometry does not invoke the any-hit shaders even if
///   present in a hit group.
/// - [`GeometryNoDuplicateAnyHitInvocationKhr`] indicates that the implementation  **must**  only
///   call the any-hit shader a single time for each primitive in this geometry. If this bit is
///   absent an implementation  **may**  invoke the any-hit shader more than once for this geometry.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`GeometryFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum GeometryFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`GeometryOpaqueKhr`] indicates that this geometry does not
    ///invoke the any-hit shaders even if present in a hit group.
    GeometryOpaqueKhr = 1,
    ///[`GeometryNoDuplicateAnyHitInvocationKhr`] indicates that
    ///the implementation  **must**  only call the any-hit shader a single time for
    ///each primitive in this geometry.
    ///If this bit is absent an implementation  **may**  invoke the any-hit shader
    ///more than once for this geometry.
    GeometryNoDuplicateAnyHitInvocationKhr = 2,
}
impl const Default for GeometryFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl GeometryFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkBuildAccelerationStructureFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) - Bitmask specifying additional parameters for acceleration structure builds
///# C Specifications
///Bits which  **can**  be set in
///[`AccelerationStructureBuildGeometryInfoKHR::flags`]
///or
///[`AccelerationStructureInfoNV::flags`]
///specifying additional parameters for acceleration structure builds, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkBuildAccelerationStructureFlagBitsKHR {
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = 0x00000001,
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = 0x00000002,
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = 0x00000004,
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = 0x00000008,
///    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = 0x00000010,
///  // Provided by VK_NV_ray_tracing_motion_blur
///    VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV = 0x00000020,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR,
///} VkBuildAccelerationStructureFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkBuildAccelerationStructureFlagBitsKHR VkBuildAccelerationStructureFlagBitsNV;
///```
///# Description
/// - [`BuildAccelerationStructureAllowUpdateKhr`] indicates     that the specified acceleration
///   structure  **can**  be updated with     a `mode` of
///   `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` in
///   [`AccelerationStructureBuildGeometryInfoKHR`] or     an `update` of [`TRUE`] in
///   [`CmdBuildAccelerationStructureNV`]     .
/// - [`BuildAccelerationStructureAllowCompactionKhr`] indicates that the specified acceleration
///   structure  **can**  act as the source for a copy acceleration structure command with `mode` of
///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` to produce a compacted acceleration
///   structure.
/// - [`BuildAccelerationStructurePreferFastTraceKhr`] indicates that the given acceleration
///   structure build  **should**  prioritize trace performance over build time.
/// - [`BuildAccelerationStructurePreferFastBuildKhr`] indicates that the given acceleration
///   structure build  **should**  prioritize build time over trace performance.
/// - [`BuildAccelerationStructureLowMemoryKhr`] indicates that this acceleration structure
///   **should**  minimize the size of the scratch memory and the final result acceleration
///   structure, potentially at the expense of build time or trace performance.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`BuildAccelerationStructureFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBuildAccelerationStructureFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum BuildAccelerationStructureFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`BuildAccelerationStructureAllowUpdateKhr`] indicates
    ///    that the specified acceleration structure  **can**  be updated with
    ///    a `mode` of `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` in
    ///    [`AccelerationStructureBuildGeometryInfoKHR`]
    ///or
    ///    an `update` of [`TRUE`] in
    ///    [`CmdBuildAccelerationStructureNV`]
    ///    .
    BuildAccelerationStructureAllowUpdateKhr = 1,
    ///[`BuildAccelerationStructureAllowCompactionKhr`] indicates
    ///that the specified acceleration structure  **can**  act as the source for a
    ///copy acceleration structure command with `mode` of
    ///`VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` to produce a
    ///compacted acceleration structure.
    BuildAccelerationStructureAllowCompactionKhr = 2,
    ///[`BuildAccelerationStructurePreferFastTraceKhr`]
    ///indicates that the given acceleration structure build  **should**  prioritize
    ///trace performance over build time.
    BuildAccelerationStructurePreferFastTraceKhr = 4,
    ///[`BuildAccelerationStructurePreferFastBuildKhr`]
    ///indicates that the given acceleration structure build  **should**  prioritize
    ///build time over trace performance.
    BuildAccelerationStructurePreferFastBuildKhr = 8,
    ///[`BuildAccelerationStructureLowMemoryKhr`] indicates that
    ///this acceleration structure  **should**  minimize the size of the scratch
    ///memory and the final result acceleration structure, potentially at the
    ///expense of build time or trace performance.
    BuildAccelerationStructureLowMemoryKhr = 16,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]
    BuildAccelerationStructureMotionNv = 32,
}
impl const Default for BuildAccelerationStructureFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl BuildAccelerationStructureFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkAccelerationStructureCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) - Bitmask specifying additional creation parameters for acceleration structure
///# C Specifications
///Bits which  **can**  be set in
///[`AccelerationStructureCreateInfoKHR::create_flags`], specifying
///additional creation parameters for acceleration structures, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureCreateFlagBitsKHR {
///    VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 0x00000001,
///  // Provided by VK_NV_ray_tracing_motion_blur
///    VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV = 0x00000004,
///} VkAccelerationStructureCreateFlagBitsKHR;
///```
///# Description
/// - [`AccelerationStructureCreateDeviceAddressCaptureReplayKhr`] specifies that the acceleration
///   structure’s address  **can**  be saved and reused on a subsequent run.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCreateFlagBitsKHR")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
#[repr(u32)]
pub enum AccelerationStructureCreateFlagBitsKHR {
    #[doc(hidden)]
    Empty = 0,
    ///[`AccelerationStructureCreateDeviceAddressCaptureReplayKhr`]
    ///specifies that the acceleration structure’s address  **can**  be saved and
    ///reused on a subsequent run.
    AccelerationStructureCreateDeviceAddressCaptureReplayKhr = 1,
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]
    AccelerationStructureCreateMotionNv = 4,
}
impl const Default for AccelerationStructureCreateFlagBitsKHR {
    fn default() -> Self {
        Self::Empty
    }
}
impl AccelerationStructureCreateFlagBitsKHR {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> u32 {
        *self as u32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: u32) -> u32 {
        std::mem::transmute(bits)
    }
}
///[VkGeometryFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryFlagBitsKHR.html) - Bitmask specifying additional parameters for a geometry
///# C Specifications
///Bits specifying additional parameters for geometries in acceleration
///structure builds, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryFlagBitsKHR {
///    VK_GEOMETRY_OPAQUE_BIT_KHR = 0x00000001,
///    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 0x00000002,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_OPAQUE_BIT_NV = VK_GEOMETRY_OPAQUE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_NV =
/// VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR,
///} VkGeometryFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryFlagBitsKHR VkGeometryFlagBitsNV;
///```
///# Description
/// - [`GeometryOpaqueKhr`] indicates that this geometry does not invoke the any-hit shaders even if
///   present in a hit group.
/// - [`GeometryNoDuplicateAnyHitInvocationKhr`] indicates that the implementation  **must**  only
///   call the any-hit shader a single time for each primitive in this geometry. If this bit is
///   absent an implementation  **may**  invoke the any-hit shader more than once for this geometry.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`GeometryFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct GeometryFlagsKHR(u32);
impl const Default for GeometryFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from(from: GeometryFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl GeometryFlagsKHR {
    ///[`GeometryOpaqueKhr`] indicates that this geometry does not
    ///invoke the any-hit shaders even if present in a hit group.
    pub const GEOMETRY_OPAQUE_KHR: Self = Self(1);
    ///[`GeometryNoDuplicateAnyHitInvocationKhr`] indicates that
    ///the implementation  **must**  only call the any-hit shader a single time for
    ///each primitive in this geometry.
    ///If this bit is absent an implementation  **may**  invoke the any-hit shader
    ///more than once for this geometry.
    pub const GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_KHR: Self = Self(2);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty() | Self::GEOMETRY_OPAQUE_KHR | Self::GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_KHR
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for GeometryFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for GeometryFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for GeometryFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for GeometryFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for GeometryFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<GeometryFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<GeometryFlagsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<GeometryFlagBitsKHR> for GeometryFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryFlagBitsKHR>>(iterator: T) -> GeometryFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryFlagsKHR);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self.0.contains(GeometryFlagsKHR::GEOMETRY_OPAQUE_KHR) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_OPAQUE_KHR))?;
                    }
                    if self
                        .0
                        .contains(GeometryFlagsKHR::GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkGeometryInstanceFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkGeometryInstanceFlagBitsKHR.html) - Instance flag bits
///# C Specifications
///Possible values of `flags` in the instance modifying the behavior of
///that instance are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkGeometryInstanceFlagBitsKHR {
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 0x00000001,
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR = 0x00000002,
///    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 0x00000004,
///    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 0x00000008,
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_TRIANGLE_CULL_DISABLE_BIT_NV =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_NV =
/// VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_NV = VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR,
///} VkGeometryInstanceFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkGeometryInstanceFlagBitsKHR VkGeometryInstanceFlagBitsNV;
///```
///# Description
/// - [`GeometryInstanceTriangleFacingCullDisableKhr`] disables face culling for this instance.
/// - [`GeometryInstanceTriangleFlipFacingKhr`] indicates that the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face)
///   for geometry in this instance is inverted. Because the facing is determined in object space,
///   an instance transform does not change the winding, but a geometry transform does.
/// - [`GeometryInstanceForceOpaqueKhr`] causes this instance to act as though
///   `VK_GEOMETRY_OPAQUE_BIT_KHR` were specified on all geometries referenced by this instance.
///   This behavior  **can**  be overridden by the SPIR-V `NoOpaqueKHR` ray flag.
/// - [`GeometryInstanceForceNoOpaqueKhr`] causes this instance to act as though
///   `VK_GEOMETRY_OPAQUE_BIT_KHR` were not specified on all geometries referenced by this instance.
///   This behavior  **can**  be overridden by the SPIR-V `OpaqueKHR` ray flag.
///[`GeometryInstanceForceNoOpaqueKhr`] and
///[`GeometryInstanceForceOpaqueKhr`] **must**  not be used in the
///same flag.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`GeometryInstanceFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkGeometryInstanceFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct GeometryInstanceFlagsKHR(u32);
impl const Default for GeometryInstanceFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from(from: GeometryInstanceFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl GeometryInstanceFlagsKHR {
    ///[`GeometryInstanceTriangleFacingCullDisableKhr`] disables
    ///face culling for this instance.
    pub const GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_KHR: Self = Self(1);
    ///[`GeometryInstanceTriangleFlipFacingKhr`] indicates that
    ///the [facing determination](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#ray-traversal-culling-face) for geometry in
    ///this instance is inverted.
    ///Because the facing is determined in object space, an instance transform
    ///does not change the winding, but a geometry transform does.
    pub const GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_KHR: Self = Self(2);
    ///[`GeometryInstanceForceOpaqueKhr`] causes this instance to
    ///act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were specified on all
    ///geometries referenced by this instance.
    ///This behavior  **can**  be overridden by the SPIR-V `NoOpaqueKHR` ray
    ///flag.
    pub const GEOMETRY_INSTANCE_FORCE_OPAQUE_KHR: Self = Self(4);
    ///[`GeometryInstanceForceNoOpaqueKhr`] causes this instance
    ///to act as though `VK_GEOMETRY_OPAQUE_BIT_KHR` were not specified on
    ///all geometries referenced by this instance.
    ///This behavior  **can**  be overridden by the SPIR-V `OpaqueKHR` ray flag.
    pub const GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_KHR: Self = Self(8);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_KHR
            | Self::GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_KHR
            | Self::GEOMETRY_INSTANCE_FORCE_OPAQUE_KHR
            | Self::GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_KHR
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for GeometryInstanceFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for GeometryInstanceFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn extend<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<GeometryInstanceFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<GeometryInstanceFlagsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<GeometryInstanceFlagBitsKHR> for GeometryInstanceFlagsKHR {
    fn from_iter<T: IntoIterator<Item = GeometryInstanceFlagBitsKHR>>(iterator: T) -> GeometryInstanceFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<GeometryInstanceFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for GeometryInstanceFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(GeometryInstanceFlagsKHR);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == GeometryInstanceFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_KHR))?;
                    }
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_KHR))?;
                    }
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::GEOMETRY_INSTANCE_FORCE_OPAQUE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_INSTANCE_FORCE_OPAQUE_KHR))?;
                    }
                    if self
                        .0
                        .contains(GeometryInstanceFlagsKHR::GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_KHR))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(GeometryInstanceFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkBuildAccelerationStructureFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkBuildAccelerationStructureFlagBitsKHR.html) - Bitmask specifying additional parameters for acceleration structure builds
///# C Specifications
///Bits which  **can**  be set in
///[`AccelerationStructureBuildGeometryInfoKHR::flags`]
///or
///[`AccelerationStructureInfoNV::flags`]
///specifying additional parameters for acceleration structure builds, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkBuildAccelerationStructureFlagBitsKHR {
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = 0x00000001,
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = 0x00000002,
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = 0x00000004,
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = 0x00000008,
///    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = 0x00000010,
///  // Provided by VK_NV_ray_tracing_motion_blur
///    VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV = 0x00000020,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR,
///  // Provided by VK_NV_ray_tracing
///    VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_NV =
/// VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR,
///} VkBuildAccelerationStructureFlagBitsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkBuildAccelerationStructureFlagBitsKHR VkBuildAccelerationStructureFlagBitsNV;
///```
///# Description
/// - [`BuildAccelerationStructureAllowUpdateKhr`] indicates     that the specified acceleration
///   structure  **can**  be updated with     a `mode` of
///   `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` in
///   [`AccelerationStructureBuildGeometryInfoKHR`] or     an `update` of [`TRUE`] in
///   [`CmdBuildAccelerationStructureNV`]     .
/// - [`BuildAccelerationStructureAllowCompactionKhr`] indicates that the specified acceleration
///   structure  **can**  act as the source for a copy acceleration structure command with `mode` of
///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` to produce a compacted acceleration
///   structure.
/// - [`BuildAccelerationStructurePreferFastTraceKhr`] indicates that the given acceleration
///   structure build  **should**  prioritize trace performance over build time.
/// - [`BuildAccelerationStructurePreferFastBuildKhr`] indicates that the given acceleration
///   structure build  **should**  prioritize build time over trace performance.
/// - [`BuildAccelerationStructureLowMemoryKhr`] indicates that this acceleration structure
///   **should**  minimize the size of the scratch memory and the final result acceleration
///   structure, potentially at the expense of build time or trace performance.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`BuildAccelerationStructureFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkBuildAccelerationStructureFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct BuildAccelerationStructureFlagsKHR(u32);
impl const Default for BuildAccelerationStructureFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from(from: BuildAccelerationStructureFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl BuildAccelerationStructureFlagsKHR {
    ///[`BuildAccelerationStructureAllowUpdateKhr`] indicates
    ///    that the specified acceleration structure  **can**  be updated with
    ///    a `mode` of `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR` in
    ///    [`AccelerationStructureBuildGeometryInfoKHR`]
    ///or
    ///    an `update` of [`TRUE`] in
    ///    [`CmdBuildAccelerationStructureNV`]
    ///    .
    pub const BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_KHR: Self = Self(1);
    ///[`BuildAccelerationStructureAllowCompactionKhr`] indicates
    ///that the specified acceleration structure  **can**  act as the source for a
    ///copy acceleration structure command with `mode` of
    ///`VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` to produce a
    ///compacted acceleration structure.
    pub const BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_KHR: Self = Self(2);
    ///[`BuildAccelerationStructurePreferFastTraceKhr`]
    ///indicates that the given acceleration structure build  **should**  prioritize
    ///trace performance over build time.
    pub const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_KHR: Self = Self(4);
    ///[`BuildAccelerationStructurePreferFastBuildKhr`]
    ///indicates that the given acceleration structure build  **should**  prioritize
    ///build time over trace performance.
    pub const BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_KHR: Self = Self(8);
    ///[`BuildAccelerationStructureLowMemoryKhr`] indicates that
    ///this acceleration structure  **should**  minimize the size of the scratch
    ///memory and the final result acceleration structure, potentially at the
    ///expense of build time or trace performance.
    pub const BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_KHR: Self = Self(16);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]
    pub const BUILD_ACCELERATION_STRUCTURE_MOTION_NV: Self = Self(32);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_KHR
            | Self::BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_KHR
            | Self::BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_KHR
            | Self::BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_KHR
            | Self::BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_KHR
            | Self::BUILD_ACCELERATION_STRUCTURE_MOTION_NV
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for BuildAccelerationStructureFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for BuildAccelerationStructureFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn extend<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<BuildAccelerationStructureFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<BuildAccelerationStructureFlagsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<BuildAccelerationStructureFlagBitsKHR> for BuildAccelerationStructureFlagsKHR {
    fn from_iter<T: IntoIterator<Item = BuildAccelerationStructureFlagBitsKHR>>(
        iterator: T,
    ) -> BuildAccelerationStructureFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<BuildAccelerationStructureFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for BuildAccelerationStructureFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(BuildAccelerationStructureFlagsKHR);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == BuildAccelerationStructureFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_KHR))?;
                    }
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_KHR))?;
                    }
                    if self.0.contains(
                        BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_KHR,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_KHR))?;
                    }
                    if self.0.contains(
                        BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_KHR,
                    ) {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_KHR))?;
                    }
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_KHR)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_KHR))?;
                    }
                    if self
                        .0
                        .contains(BuildAccelerationStructureFlagsKHR::BUILD_ACCELERATION_STRUCTURE_MOTION_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(BUILD_ACCELERATION_STRUCTURE_MOTION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(BuildAccelerationStructureFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkAccelerationStructureCreateFlagBitsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateFlagBitsKHR.html) - Bitmask specifying additional creation parameters for acceleration structure
///# C Specifications
///Bits which  **can**  be set in
///[`AccelerationStructureCreateInfoKHR::create_flags`], specifying
///additional creation parameters for acceleration structures, are:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef enum VkAccelerationStructureCreateFlagBitsKHR {
///    VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 0x00000001,
///  // Provided by VK_NV_ray_tracing_motion_blur
///    VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV = 0x00000004,
///} VkAccelerationStructureCreateFlagBitsKHR;
///```
///# Description
/// - [`AccelerationStructureCreateDeviceAddressCaptureReplayKhr`] specifies that the acceleration
///   structure’s address  **can**  be saved and reused on a subsequent run.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureCreateFlagsKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCreateFlagsKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct AccelerationStructureCreateFlagsKHR(u32);
impl const Default for AccelerationStructureCreateFlagsKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl From<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from(from: AccelerationStructureCreateFlagBitsKHR) -> Self {
        unsafe { Self::from_bits_unchecked(from as u32) }
    }
}
impl AccelerationStructureCreateFlagsKHR {
    ///[`AccelerationStructureCreateDeviceAddressCaptureReplayKhr`]
    ///specifies that the acceleration structure’s address  **can**  be saved and
    ///reused on a subsequent run.
    pub const ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_KHR: Self = Self(1);
    ///No documentation found
    ///
    ///Provided by [`crate::extensions::nv_ray_tracing_motion_blur`]
    pub const ACCELERATION_STRUCTURE_CREATE_MOTION_NV: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Returns a value with all of the flags enabled
    #[inline]
    pub const fn all() -> Self {
        Self::empty()
            | Self::ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_KHR
            | Self::ACCELERATION_STRUCTURE_CREATE_MOTION_NV
    }
    ///Returns the raw bits
    #[inline]
    pub const fn bits(&self) -> u32 {
        self.0
    }
    ///Convert raw bits into a bit flags checking that only valid
    ///bits are contained.
    #[inline]
    pub const fn from_bits(bits: u32) -> Option<Self> {
        if (bits & !Self::all().bits()) == 0 {
            Some(Self(bits))
        } else {
            None
        }
    }
    ///Convert raw bits into a bit flags truncating all invalid
    ///bits that may be contained.
    #[inline]
    pub const fn from_bits_truncate(bits: u32) -> Self {
        Self(Self::all().0 & bits)
    }
    ///Convert raw bits into a bit preserving all bits
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: u32) -> Self {
        Self(bits)
    }
    ///Returns `true` if no flags are currently set
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.bits() == Self::empty().bits()
    }
    ///Returns `true` if all flags are currently set
    #[inline]
    pub const fn is_all(&self) -> bool {
        self.bits() == Self::all().bits()
    }
    ///Returns `true` if there are flags in common to `self` and `other`
    #[inline]
    pub const fn intersects(&self, other: Self) -> bool {
        !Self(self.bits() & other.bits()).is_empty()
    }
    ///Returns `true` if all of the flags in `other` are contained `self`
    #[inline]
    pub const fn contains(&self, other: Self) -> bool {
        (self.bits() & other.bits()) == other.bits()
    }
    ///Inserts a set of flags in place
    #[inline]
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.bits()
    }
    ///Removes a set of flags in place
    #[inline]
    pub fn remove(&mut self, other: Self) {
        self.0 &= !other.bits();
    }
    ///Toggles a set of flags in place
    #[inline]
    pub fn toggle(&mut self, other: Self) {
        self.0 ^= other.bits();
    }
    ///Inserts or removes the specified flags depending on the value of `is_insert`
    #[inline]
    pub fn set(&mut self, other: Self, is_insert: bool) {
        if is_insert {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }
    ///Returns the intersection between `self` and `other`
    #[inline]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.bits() & other.bits())
    }
    ///Returns the union between `self` and `other`
    #[inline]
    pub const fn union(self, other: Self) -> Self {
        Self(self.bits() | other.bits())
    }
    ///Returns the difference between `self` and `other`
    #[inline]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.bits() & !other.bits())
    }
    ///Returns the [symmetric difference][sym-diff] between `self` and `other`
    ///
    ///[sym-diff]: https://en.wikipedia.org/wiki/Symmetric_difference
    #[inline]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.bits() ^ other.bits())
    }
    ///Returns the complement of `self`.
    #[inline]
    pub const fn complement(self) -> Self {
        Self::from_bits_truncate(!self.bits())
    }
}
impl const std::ops::BitOr for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitor(self, other: Self) -> Self {
        self.union(other)
    }
}
impl std::ops::BitOrAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitor_assign(&mut self, other: Self) {
        *self = *self | other;
    }
}
impl const std::ops::BitXor for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitxor(self, other: Self) -> Self {
        self.symmetric_difference(other)
    }
}
impl std::ops::BitXorAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitxor_assign(&mut self, other: Self) {
        *self = *self ^ other;
    }
}
impl const std::ops::BitAnd for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn bitand(self, other: Self) -> Self {
        self.intersection(other)
    }
}
impl std::ops::BitAndAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn bitand_assign(&mut self, other: Self) {
        *self = *self & other;
    }
}
impl const std::ops::Sub for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.difference(other)
    }
}
impl std::ops::SubAssign for AccelerationStructureCreateFlagsKHR {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl const std::ops::Not for AccelerationStructureCreateFlagsKHR {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        self.complement()
    }
}
impl Extend<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, i);
        }
    }
}
impl Extend<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn extend<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(&mut self, iterator: T) {
        for i in iterator {
            Self::insert(self, <Self as From<AccelerationStructureCreateFlagBitsKHR>>::from(i));
        }
    }
}
impl FromIterator<AccelerationStructureCreateFlagsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl FromIterator<AccelerationStructureCreateFlagBitsKHR> for AccelerationStructureCreateFlagsKHR {
    fn from_iter<T: IntoIterator<Item = AccelerationStructureCreateFlagBitsKHR>>(
        iterator: T,
    ) -> AccelerationStructureCreateFlagsKHR {
        let mut out = Self::empty();
        <Self as Extend<AccelerationStructureCreateFlagBitsKHR>>::extend(&mut out, iterator);
        out
    }
}
impl std::fmt::Debug for AccelerationStructureCreateFlagsKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        struct Flags(AccelerationStructureCreateFlagsKHR);
        impl std::fmt::Debug for Flags {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                if self.0 == AccelerationStructureCreateFlagsKHR::empty() {
                    f.write_str("empty")?;
                } else {
                    let mut first = true;
                    if self . 0 . contains (AccelerationStructureCreateFlagsKHR :: ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_KHR) { if ! first { first = false ; f . write_str (" | ") ? ; } f . write_str (stringify ! (ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_KHR)) ? ; }
                    if self
                        .0
                        .contains(AccelerationStructureCreateFlagsKHR::ACCELERATION_STRUCTURE_CREATE_MOTION_NV)
                    {
                        if !first {
                            first = false;
                            f.write_str(" | ")?;
                        }
                        f.write_str(stringify!(ACCELERATION_STRUCTURE_CREATE_MOTION_NV))?;
                    }
                }
                Ok(())
            }
        }
        f.debug_tuple(stringify!(AccelerationStructureCreateFlagsKHR))
            .field(&Flags(*self))
            .finish()
    }
}
///[VkWriteDescriptorSetAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkWriteDescriptorSetAccelerationStructureKHR.html) - Structure specifying acceleration structure descriptor information
///# C Specifications
///The [`WriteDescriptorSetAccelerationStructureKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkWriteDescriptorSetAccelerationStructureKHR {
///    VkStructureType                      sType;
///    const void*                          pNext;
///    uint32_t                             accelerationStructureCount;
///    const VkAccelerationStructureKHR*    pAccelerationStructures;
///} VkWriteDescriptorSetAccelerationStructureKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure_count`] is the number of elements in [`acceleration_structures`].
/// - [`acceleration_structures`] is a pointer to an array of [`AccelerationStructureKHR`]
///   structures specifying the acceleration structures to update.
///# Description
///## Valid Usage
/// - [`acceleration_structure_count`] **must**  be equal to `descriptorCount` in the extended
///   structure
/// - Each acceleration structure in [`acceleration_structures`] **must**  have been created with a
///   `type` of `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR` or
///   `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - If the [nullDescriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-nullDescriptor)
///   feature is not enabled, each element of [`acceleration_structures`] **must**  not be
///   [`crate::utils::Handle::null`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_ACCELERATION_STRUCTURE_KHR`
/// - [`acceleration_structures`] **must**  be a valid pointer to an array of
///   [`acceleration_structure_count`] valid or
///   [`crate::utils::Handle::null`][`AccelerationStructureKHR`] handles
/// - [`acceleration_structure_count`] **must**  be greater than `0`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkWriteDescriptorSetAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct WriteDescriptorSetAccelerationStructureKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`acceleration_structure_count`] is the number of elements in
    ///[`acceleration_structures`].
    pub acceleration_structure_count: u32,
    ///[`acceleration_structures`] is a pointer to an array of
    ///[`AccelerationStructureKHR`] structures specifying the acceleration
    ///structures to update.
    pub acceleration_structures: *const AccelerationStructureKHR,
}
impl<'lt> Default for WriteDescriptorSetAccelerationStructureKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            acceleration_structure_count: 0,
            acceleration_structures: std::ptr::null(),
        }
    }
}
impl<'lt> WriteDescriptorSetAccelerationStructureKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::acceleration_structures`]
    pub fn acceleration_structures_raw(&self) -> *const AccelerationStructureKHR {
        self.acceleration_structures
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structures`]
    pub fn set_acceleration_structures_raw(&mut self, value: *const AccelerationStructureKHR) -> &mut Self {
        self.acceleration_structures = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::acceleration_structure_count`]
    pub fn acceleration_structure_count(&self) -> u32 {
        self.acceleration_structure_count
    }
    ///Gets the value of [`Self::acceleration_structures`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn acceleration_structures(&self) -> &[AccelerationStructureKHR] {
        std::slice::from_raw_parts(self.acceleration_structures, self.acceleration_structure_count as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_count`]
    pub fn acceleration_structure_count_mut(&mut self) -> &mut u32 {
        &mut self.acceleration_structure_count
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_count`]
    pub fn set_acceleration_structure_count(&mut self, value: u32) -> &mut Self {
        self.acceleration_structure_count = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structures`]
    pub fn set_acceleration_structures(
        &mut self,
        value: &'lt [crate::extensions::khr_acceleration_structure::AccelerationStructureKHR],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.acceleration_structures = value.as_ptr();
        self.acceleration_structure_count = len_;
        self
    }
}
///[VkPhysicalDeviceAccelerationStructureFeaturesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructureFeaturesKHR.html) - Structure describing the acceleration structure features that can be supported by an implementation
///# C Specifications
///The [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    VkBool32           accelerationStructure;
///    VkBool32           accelerationStructureCaptureReplay;
///    VkBool32           accelerationStructureIndirectBuild;
///    VkBool32           accelerationStructureHostCommands;
///    VkBool32           descriptorBindingAccelerationStructureUpdateAfterBind;
///} VkPhysicalDeviceAccelerationStructureFeaturesKHR;
///```
///# Members
///This structure describes the following features:
///# Description
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure`] indicates whether the implementation supports the acceleration structure functionality. See [Acceleration Structures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
/// - [`acceleration_structure_capture_replay`] indicates whether the implementation supports saving
///   and reusing acceleration structure device addresses, e.g. for trace capture and replay.
/// - [`acceleration_structure_indirect_build`] indicates whether the implementation supports
///   indirect acceleration structure build commands, e.g.
///   [`CmdBuildAccelerationStructuresIndirectKHR`].
/// - [`acceleration_structure_host_commands`] indicates whether the implementation supports host
///   side acceleration structure commands, e.g. [`BuildAccelerationStructuresKHR`],
///   [`CopyAccelerationStructureKHR`], [`CopyAccelerationStructureToMemoryKHR`],
///   [`CopyMemoryToAccelerationStructureKHR`], [`WriteAccelerationStructuresPropertiesKHR`].
/// - [`descriptor_binding_acceleration_structure_update_after_bind`] indicates whether the
///   implementation supports updating acceleration structure descriptors after a set is bound. If
///   this feature is not enabled, `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be
///   used with `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`.
///If the [`PhysicalDeviceAccelerationStructureFeaturesKHR`] structure is included in the
/// [`p_next`] chain of the
///[`PhysicalDeviceFeatures2`] structure passed to
///[`GetPhysicalDeviceFeatures2`], it is filled in to indicate whether each
///corresponding feature is supported.
///[`PhysicalDeviceAccelerationStructureFeaturesKHR`] **can**  also be used in the [`p_next`] chain
/// of
///[`DeviceCreateInfo`] to selectively enable these features.
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_FEATURES_KHR`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`Bool32`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceAccelerationStructureFeaturesKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructureFeaturesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`acceleration_structure`] indicates
    ///whether the implementation supports the acceleration structure
    ///functionality.
    ///See [Acceleration Structures](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure).
    pub acceleration_structure: Bool32,
    ///[`acceleration_structure_capture_replay`] indicates whether the
    ///implementation supports saving and reusing acceleration structure device
    ///addresses, e.g. for trace capture and replay.
    pub acceleration_structure_capture_replay: Bool32,
    ///[`acceleration_structure_indirect_build`] indicates whether the
    ///implementation supports indirect acceleration structure build commands,
    ///e.g. [`CmdBuildAccelerationStructuresIndirectKHR`].
    pub acceleration_structure_indirect_build: Bool32,
    ///[`acceleration_structure_host_commands`] indicates whether the
    ///implementation supports host side acceleration structure commands, e.g.
    ///[`BuildAccelerationStructuresKHR`],
    ///[`CopyAccelerationStructureKHR`],
    ///[`CopyAccelerationStructureToMemoryKHR`],
    ///[`CopyMemoryToAccelerationStructureKHR`],
    ///[`WriteAccelerationStructuresPropertiesKHR`].
    pub acceleration_structure_host_commands: Bool32,
    ///[`descriptor_binding_acceleration_structure_update_after_bind`] indicates
    ///whether the implementation supports updating acceleration structure
    ///descriptors after a set is bound.
    ///If this feature is not enabled,
    ///`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`.
    pub descriptor_binding_acceleration_structure_update_after_bind: Bool32,
}
impl<'lt> Default for PhysicalDeviceAccelerationStructureFeaturesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            acceleration_structure: 0,
            acceleration_structure_capture_replay: 0,
            acceleration_structure_indirect_build: 0,
            acceleration_structure_host_commands: 0,
            descriptor_binding_acceleration_structure_update_after_bind: 0,
        }
    }
}
impl<'lt> PhysicalDeviceAccelerationStructureFeaturesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Gets the raw value of [`Self::acceleration_structure`]
    pub fn acceleration_structure_raw(&self) -> Bool32 {
        self.acceleration_structure
    }
    ///Gets the raw value of [`Self::acceleration_structure_capture_replay`]
    pub fn acceleration_structure_capture_replay_raw(&self) -> Bool32 {
        self.acceleration_structure_capture_replay
    }
    ///Gets the raw value of [`Self::acceleration_structure_indirect_build`]
    pub fn acceleration_structure_indirect_build_raw(&self) -> Bool32 {
        self.acceleration_structure_indirect_build
    }
    ///Gets the raw value of [`Self::acceleration_structure_host_commands`]
    pub fn acceleration_structure_host_commands_raw(&self) -> Bool32 {
        self.acceleration_structure_host_commands
    }
    ///Gets the raw value of [`Self::descriptor_binding_acceleration_structure_update_after_bind`]
    pub fn descriptor_binding_acceleration_structure_update_after_bind_raw(&self) -> Bool32 {
        self.descriptor_binding_acceleration_structure_update_after_bind
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure`]
    pub fn set_acceleration_structure_raw(&mut self, value: Bool32) -> &mut Self {
        self.acceleration_structure = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_capture_replay`]
    pub fn set_acceleration_structure_capture_replay_raw(&mut self, value: Bool32) -> &mut Self {
        self.acceleration_structure_capture_replay = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_indirect_build`]
    pub fn set_acceleration_structure_indirect_build_raw(&mut self, value: Bool32) -> &mut Self {
        self.acceleration_structure_indirect_build = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_host_commands`]
    pub fn set_acceleration_structure_host_commands_raw(&mut self, value: Bool32) -> &mut Self {
        self.acceleration_structure_host_commands = value;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_acceleration_structure_update_after_bind`]
    pub fn set_descriptor_binding_acceleration_structure_update_after_bind_raw(&mut self, value: Bool32) -> &mut Self {
        self.descriptor_binding_acceleration_structure_update_after_bind = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure(&self) -> bool {
        unsafe { std::mem::transmute(self.acceleration_structure as u8) }
    }
    ///Gets the value of [`Self::acceleration_structure_capture_replay`]
    pub fn acceleration_structure_capture_replay(&self) -> bool {
        unsafe { std::mem::transmute(self.acceleration_structure_capture_replay as u8) }
    }
    ///Gets the value of [`Self::acceleration_structure_indirect_build`]
    pub fn acceleration_structure_indirect_build(&self) -> bool {
        unsafe { std::mem::transmute(self.acceleration_structure_indirect_build as u8) }
    }
    ///Gets the value of [`Self::acceleration_structure_host_commands`]
    pub fn acceleration_structure_host_commands(&self) -> bool {
        unsafe { std::mem::transmute(self.acceleration_structure_host_commands as u8) }
    }
    ///Gets the value of [`Self::descriptor_binding_acceleration_structure_update_after_bind`]
    pub fn descriptor_binding_acceleration_structure_update_after_bind(&self) -> bool {
        unsafe { std::mem::transmute(self.descriptor_binding_acceleration_structure_update_after_bind as u8) }
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.acceleration_structure as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.acceleration_structure as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_capture_replay`]
    pub fn acceleration_structure_capture_replay_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.acceleration_structure_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.acceleration_structure_capture_replay as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_indirect_build`]
    pub fn acceleration_structure_indirect_build_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.acceleration_structure_indirect_build as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.acceleration_structure_indirect_build as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_host_commands`]
    pub fn acceleration_structure_host_commands_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.acceleration_structure_host_commands as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.acceleration_structure_host_commands as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of
    /// [`Self::descriptor_binding_acceleration_structure_update_after_bind`]
    pub fn descriptor_binding_acceleration_structure_update_after_bind_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.descriptor_binding_acceleration_structure_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.descriptor_binding_acceleration_structure_update_after_bind as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure`]
    pub fn set_acceleration_structure(&mut self, value: bool) -> &mut Self {
        self.acceleration_structure = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_capture_replay`]
    pub fn set_acceleration_structure_capture_replay(&mut self, value: bool) -> &mut Self {
        self.acceleration_structure_capture_replay = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_indirect_build`]
    pub fn set_acceleration_structure_indirect_build(&mut self, value: bool) -> &mut Self {
        self.acceleration_structure_indirect_build = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_host_commands`]
    pub fn set_acceleration_structure_host_commands(&mut self, value: bool) -> &mut Self {
        self.acceleration_structure_host_commands = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::descriptor_binding_acceleration_structure_update_after_bind`]
    pub fn set_descriptor_binding_acceleration_structure_update_after_bind(&mut self, value: bool) -> &mut Self {
        self.descriptor_binding_acceleration_structure_update_after_bind = value as u8 as u32;
        self
    }
}
///[VkPhysicalDeviceAccelerationStructurePropertiesKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceAccelerationStructurePropertiesKHR.html) - Properties of the physical device for acceleration structure
///# C Specifications
///The [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
///    VkStructureType    sType;
///    void*              pNext;
///    uint64_t           maxGeometryCount;
///    uint64_t           maxInstanceCount;
///    uint64_t           maxPrimitiveCount;
///    uint32_t           maxPerStageDescriptorAccelerationStructures;
///    uint32_t           maxPerStageDescriptorUpdateAfterBindAccelerationStructures;
///    uint32_t           maxDescriptorSetAccelerationStructures;
///    uint32_t           maxDescriptorSetUpdateAfterBindAccelerationStructures;
///    uint32_t           minAccelerationStructureScratchOffsetAlignment;
///} VkPhysicalDeviceAccelerationStructurePropertiesKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`max_geometry_count`] is the maximum number of geometries in the bottom level acceleration
///   structure.
/// - [`max_instance_count`] is the maximum number of instances in the top level acceleration
///   structure.
/// - [`max_primitive_count`] is the maximum number of triangles or AABBs in all geometries in the
///   bottom level acceleration structure.
/// - [`max_per_stage_descriptor_acceleration_structures`] is the maximum number of acceleration
///   structure bindings that  **can**  be accessible to a single shader stage in a pipeline layout.
///   Descriptor bindings with a descriptor type of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
///   count against this limit. Only descriptor bindings in descriptor set layouts created without
///   the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this
///   limit.
/// - [`max_per_stage_descriptor_update_after_bind_acceleration_structures`] is similar to
///   [`max_per_stage_descriptor_acceleration_structures`] but counts descriptor bindings from
///   descriptor sets created with or without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set.
/// - [`max_descriptor_set_acceleration_structures`] is the maximum number of acceleration structure
///   descriptors that  **can**  be included in descriptor bindings in a pipeline layout across all
///   pipeline shader stages and descriptor set numbers. Descriptor bindings with a descriptor type
///   of `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this limit. Only descriptor
///   bindings in descriptor set layouts created without the
///   `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set count against this limit.
/// - [`max_descriptor_set_update_after_bind_acceleration_structures`] is similar to
///   [`max_descriptor_set_acceleration_structures`] but counts descriptor bindings from descriptor
///   sets created with or without the `VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT`
///   bit set.
/// - [`min_acceleration_structure_scratch_offset_alignment`] is the minimum  **required**
///   alignment, in bytes, for scratch data passed in to an acceleration structure build command.
///   The value  **must**  be a power of two.
///# Description
///Due to the fact that the geometry, instance, and primitive counts are
///specified at acceleration structure creation as 32-bit values,
///[[`max_geometry_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxGeometryCount),
///[[`max_instance_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxInstanceCount), and
///[[`max_primitive_count`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-maxPrimitiveCount) **must**  not exceed
///2<sup>32</sup>-1.If the [`PhysicalDeviceAccelerationStructurePropertiesKHR`] structure is
/// included in the [`p_next`] chain of the
///[`PhysicalDeviceProperties2`] structure passed to
///[`GetPhysicalDeviceProperties2`], it is filled in with each
///corresponding implementation-dependent property.Limits specified by this structure  **must**
/// match those specified with the same
///name in [`PhysicalDeviceRayTracingPropertiesNV`].
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be
///   `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ACCELERATION_STRUCTURE_PROPERTIES_KHR`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPhysicalDeviceAccelerationStructurePropertiesKHR")]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct PhysicalDeviceAccelerationStructurePropertiesKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *mut BaseOutStructure<'lt>,
    ///[`max_geometry_count`] is the maximum number
    ///of geometries in the bottom level acceleration structure.
    pub max_geometry_count: u64,
    ///[`max_instance_count`] is the maximum number
    ///of instances in the top level acceleration structure.
    pub max_instance_count: u64,
    ///[`max_primitive_count`] is the maximum
    ///number of triangles or AABBs in all geometries in the bottom level
    ///acceleration structure.
    pub max_primitive_count: u64,
    ///[`max_per_stage_descriptor_acceleration_structures`] is the maximum number
    ///of acceleration structure bindings that  **can**  be accessible to a single
    ///shader stage in a pipeline layout.
    ///Descriptor bindings with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this
    ///limit.
    ///Only descriptor bindings in descriptor set layouts created without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
    ///count against this limit.
    pub max_per_stage_descriptor_acceleration_structures: u32,
    ///[`max_per_stage_descriptor_update_after_bind_acceleration_structures`] is
    ///similar to [`max_per_stage_descriptor_acceleration_structures`] but counts
    ///descriptor bindings from descriptor sets created with or without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit
    ///set.
    pub max_per_stage_descriptor_update_after_bind_acceleration_structures: u32,
    ///[`max_descriptor_set_acceleration_structures`] is the maximum number of
    ///acceleration structure descriptors that  **can**  be included in descriptor
    ///bindings in a pipeline layout across all pipeline shader stages and
    ///descriptor set numbers.
    ///Descriptor bindings with a descriptor type of
    ///`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this
    ///limit.
    ///Only descriptor bindings in descriptor set layouts created without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
    ///count against this limit.
    pub max_descriptor_set_acceleration_structures: u32,
    ///[`max_descriptor_set_update_after_bind_acceleration_structures`] is similar
    ///to [`max_descriptor_set_acceleration_structures`] but counts descriptor
    ///bindings from descriptor sets created with or without the
    ///`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit
    ///set.
    pub max_descriptor_set_update_after_bind_acceleration_structures: u32,
    ///[`min_acceleration_structure_scratch_offset_alignment`] is the minimum
    /// **required**  alignment, in bytes, for scratch data passed in to an
    ///acceleration structure build command.
    ///The value  **must**  be a power of two.
    pub min_acceleration_structure_scratch_offset_alignment: u32,
}
impl<'lt> Default for PhysicalDeviceAccelerationStructurePropertiesKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null_mut(),
            max_geometry_count: 0,
            max_instance_count: 0,
            max_primitive_count: 0,
            max_per_stage_descriptor_acceleration_structures: 0,
            max_per_stage_descriptor_update_after_bind_acceleration_structures: 0,
            max_descriptor_set_acceleration_structures: 0,
            max_descriptor_set_update_after_bind_acceleration_structures: 0,
            min_acceleration_structure_scratch_offset_alignment: 0,
        }
    }
}
impl<'lt> PhysicalDeviceAccelerationStructurePropertiesKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> &*mut BaseOutStructure<'lt> {
        &self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *mut BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseOutStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::max_geometry_count`]
    pub fn max_geometry_count(&self) -> u64 {
        self.max_geometry_count
    }
    ///Gets the value of [`Self::max_instance_count`]
    pub fn max_instance_count(&self) -> u64 {
        self.max_instance_count
    }
    ///Gets the value of [`Self::max_primitive_count`]
    pub fn max_primitive_count(&self) -> u64 {
        self.max_primitive_count
    }
    ///Gets the value of [`Self::max_per_stage_descriptor_acceleration_structures`]
    pub fn max_per_stage_descriptor_acceleration_structures(&self) -> u32 {
        self.max_per_stage_descriptor_acceleration_structures
    }
    ///Gets the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_acceleration_structures`]
    pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures(&self) -> u32 {
        self.max_per_stage_descriptor_update_after_bind_acceleration_structures
    }
    ///Gets the value of [`Self::max_descriptor_set_acceleration_structures`]
    pub fn max_descriptor_set_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_acceleration_structures
    }
    ///Gets the value of [`Self::max_descriptor_set_update_after_bind_acceleration_structures`]
    pub fn max_descriptor_set_update_after_bind_acceleration_structures(&self) -> u32 {
        self.max_descriptor_set_update_after_bind_acceleration_structures
    }
    ///Gets the value of [`Self::min_acceleration_structure_scratch_offset_alignment`]
    pub fn min_acceleration_structure_scratch_offset_alignment(&self) -> u32 {
        self.min_acceleration_structure_scratch_offset_alignment
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next_mut(&mut self) -> &mut BaseOutStructure<'lt> {
        &mut *self.p_next
    }
    ///Gets a mutable reference to the value of [`Self::max_geometry_count`]
    pub fn max_geometry_count_mut(&mut self) -> &mut u64 {
        &mut self.max_geometry_count
    }
    ///Gets a mutable reference to the value of [`Self::max_instance_count`]
    pub fn max_instance_count_mut(&mut self) -> &mut u64 {
        &mut self.max_instance_count
    }
    ///Gets a mutable reference to the value of [`Self::max_primitive_count`]
    pub fn max_primitive_count_mut(&mut self) -> &mut u64 {
        &mut self.max_primitive_count
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_acceleration_structures`]
    pub fn max_per_stage_descriptor_acceleration_structures_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_acceleration_structures
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_acceleration_structures`]
    pub fn max_per_stage_descriptor_update_after_bind_acceleration_structures_mut(&mut self) -> &mut u32 {
        &mut self.max_per_stage_descriptor_update_after_bind_acceleration_structures
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_acceleration_structures`]
    pub fn max_descriptor_set_acceleration_structures_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_acceleration_structures
    }
    ///Gets a mutable reference to the value of
    /// [`Self::max_descriptor_set_update_after_bind_acceleration_structures`]
    pub fn max_descriptor_set_update_after_bind_acceleration_structures_mut(&mut self) -> &mut u32 {
        &mut self.max_descriptor_set_update_after_bind_acceleration_structures
    }
    ///Gets a mutable reference to the value of
    /// [`Self::min_acceleration_structure_scratch_offset_alignment`]
    pub fn min_acceleration_structure_scratch_offset_alignment_mut(&mut self) -> &mut u32 {
        &mut self.min_acceleration_structure_scratch_offset_alignment
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt mut crate::vulkan1_0::BaseOutStructure<'lt>) -> &mut Self {
        self.p_next = value as *mut _;
        self
    }
    ///Sets the raw value of [`Self::max_geometry_count`]
    pub fn set_max_geometry_count(&mut self, value: u64) -> &mut Self {
        self.max_geometry_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_instance_count`]
    pub fn set_max_instance_count(&mut self, value: u64) -> &mut Self {
        self.max_instance_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_primitive_count`]
    pub fn set_max_primitive_count(&mut self, value: u64) -> &mut Self {
        self.max_primitive_count = value;
        self
    }
    ///Sets the raw value of [`Self::max_per_stage_descriptor_acceleration_structures`]
    pub fn set_max_per_stage_descriptor_acceleration_structures(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_acceleration_structures = value;
        self
    }
    ///Sets the raw value of
    /// [`Self::max_per_stage_descriptor_update_after_bind_acceleration_structures`]
    pub fn set_max_per_stage_descriptor_update_after_bind_acceleration_structures(&mut self, value: u32) -> &mut Self {
        self.max_per_stage_descriptor_update_after_bind_acceleration_structures = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_acceleration_structures`]
    pub fn set_max_descriptor_set_acceleration_structures(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_acceleration_structures = value;
        self
    }
    ///Sets the raw value of [`Self::max_descriptor_set_update_after_bind_acceleration_structures`]
    pub fn set_max_descriptor_set_update_after_bind_acceleration_structures(&mut self, value: u32) -> &mut Self {
        self.max_descriptor_set_update_after_bind_acceleration_structures = value;
        self
    }
    ///Sets the raw value of [`Self::min_acceleration_structure_scratch_offset_alignment`]
    pub fn set_min_acceleration_structure_scratch_offset_alignment(&mut self, value: u32) -> &mut Self {
        self.min_acceleration_structure_scratch_offset_alignment = value;
        self
    }
}
///[VkAccelerationStructureGeometryTrianglesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryTrianglesDataKHR.html) - Structure specifying a triangle geometry in a bottom-level acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryTrianglesDataKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryTrianglesDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkFormat                         vertexFormat;
///    VkDeviceOrHostAddressConstKHR    vertexData;
///    VkDeviceSize                     vertexStride;
///    uint32_t                         maxVertex;
///    VkIndexType                      indexType;
///    VkDeviceOrHostAddressConstKHR    indexData;
///    VkDeviceOrHostAddressConstKHR    transformData;
///} VkAccelerationStructureGeometryTrianglesDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`vertex_format`] is the [`Format`] of each vertex element.
/// - [`vertex_data`] is a device or host address to memory containing vertex data for this
///   geometry.
/// - [`max_vertex`] is the highest index of a vertex that will be addressed by a build command
///   using this structure.
/// - [`vertex_stride`] is the stride in bytes between each vertex.
/// - [`index_type`] is the [`IndexType`] of each index element.
/// - [`index_data`] is a device or host address to memory containing index data for this geometry.
/// - [`transform_data`] is a device or host address to memory containing an optional reference to a
///   [`TransformMatrixKHR`] structure describing a transformation from the space in which the
///   vertices in this geometry are described to the space in which the acceleration structure is
///   defined.
///# Description
///## Valid Usage
/// - [`vertex_stride`] **must**  be a multiple of the size in bytes of the smallest component of
///   [`vertex_format`]
/// - [`vertex_stride`] **must**  be less than or equal to 2<sup>32</sup>-1
/// - [`vertex_format`] **must**  support the
///   `VK_FORMAT_FEATURE_ACCELERATION_STRUCTURE_VERTEX_BUFFER_BIT_KHR` in
///   [`FormatProperties::buffer_features`] as returned by [`GetPhysicalDeviceFormatProperties2`]
/// - [`index_type`] **must**  be `VK_INDEX_TYPE_UINT16`, `VK_INDEX_TYPE_UINT32`, or
///   `VK_INDEX_TYPE_NONE_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_TRIANGLES_DATA_KHR`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`AccelerationStructureGeometryMotionTrianglesDataNV`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`vertex_format`] **must**  be a valid [`Format`] value
/// - [`index_type`] **must**  be a valid [`IndexType`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`DeviceSize`]
/// - [`Format`]
/// - [`IndexType`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryTrianglesDataKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureGeometryTrianglesDataKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`vertex_format`] is the [`Format`] of each vertex element.
    pub vertex_format: Format,
    ///[`vertex_data`] is a device or host address to memory containing vertex
    ///data for this geometry.
    pub vertex_data: DeviceOrHostAddressConstKHR,
    ///[`vertex_stride`] is the stride in bytes between each vertex.
    pub vertex_stride: DeviceSize,
    ///[`max_vertex`] is the highest index of a vertex that will be addressed
    ///by a build command using this structure.
    pub max_vertex: u32,
    ///[`index_type`] is the [`IndexType`] of each index element.
    pub index_type: IndexType,
    ///[`index_data`] is a device or host address to memory containing index
    ///data for this geometry.
    pub index_data: DeviceOrHostAddressConstKHR,
    ///[`transform_data`] is a device or host address to memory containing an
    ///optional reference to a [`TransformMatrixKHR`] structure describing
    ///a transformation from the space in which the vertices in this geometry
    ///are described to the space in which the acceleration structure is
    ///defined.
    pub transform_data: DeviceOrHostAddressConstKHR,
}
impl<'lt> Default for AccelerationStructureGeometryTrianglesDataKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            vertex_format: Default::default(),
            vertex_data: Default::default(),
            vertex_stride: Default::default(),
            max_vertex: 0,
            index_type: Default::default(),
            index_data: Default::default(),
            transform_data: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureGeometryTrianglesDataKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::vertex_format`]
    pub fn vertex_format(&self) -> Format {
        self.vertex_format
    }
    ///Gets the value of [`Self::vertex_data`]
    pub fn vertex_data(&self) -> DeviceOrHostAddressConstKHR {
        self.vertex_data
    }
    ///Gets the value of [`Self::vertex_stride`]
    pub fn vertex_stride(&self) -> DeviceSize {
        self.vertex_stride
    }
    ///Gets the value of [`Self::max_vertex`]
    pub fn max_vertex(&self) -> u32 {
        self.max_vertex
    }
    ///Gets the value of [`Self::index_type`]
    pub fn index_type(&self) -> IndexType {
        self.index_type
    }
    ///Gets the value of [`Self::index_data`]
    pub fn index_data(&self) -> DeviceOrHostAddressConstKHR {
        self.index_data
    }
    ///Gets the value of [`Self::transform_data`]
    pub fn transform_data(&self) -> DeviceOrHostAddressConstKHR {
        self.transform_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::vertex_format`]
    pub fn vertex_format_mut(&mut self) -> &mut Format {
        &mut self.vertex_format
    }
    ///Gets a mutable reference to the value of [`Self::vertex_data`]
    pub fn vertex_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.vertex_data
    }
    ///Gets a mutable reference to the value of [`Self::vertex_stride`]
    pub fn vertex_stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.vertex_stride
    }
    ///Gets a mutable reference to the value of [`Self::max_vertex`]
    pub fn max_vertex_mut(&mut self) -> &mut u32 {
        &mut self.max_vertex
    }
    ///Gets a mutable reference to the value of [`Self::index_type`]
    pub fn index_type_mut(&mut self) -> &mut IndexType {
        &mut self.index_type
    }
    ///Gets a mutable reference to the value of [`Self::index_data`]
    pub fn index_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.index_data
    }
    ///Gets a mutable reference to the value of [`Self::transform_data`]
    pub fn transform_data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.transform_data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::vertex_format`]
    pub fn set_vertex_format(&mut self, value: crate::vulkan1_0::Format) -> &mut Self {
        self.vertex_format = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_data`]
    pub fn set_vertex_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.vertex_data = value;
        self
    }
    ///Sets the raw value of [`Self::vertex_stride`]
    pub fn set_vertex_stride(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.vertex_stride = value;
        self
    }
    ///Sets the raw value of [`Self::max_vertex`]
    pub fn set_max_vertex(&mut self, value: u32) -> &mut Self {
        self.max_vertex = value;
        self
    }
    ///Sets the raw value of [`Self::index_type`]
    pub fn set_index_type(&mut self, value: crate::vulkan1_0::IndexType) -> &mut Self {
        self.index_type = value;
        self
    }
    ///Sets the raw value of [`Self::index_data`]
    pub fn set_index_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.index_data = value;
        self
    }
    ///Sets the raw value of [`Self::transform_data`]
    pub fn set_transform_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.transform_data = value;
        self
    }
}
///[VkAccelerationStructureGeometryAabbsDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryAabbsDataKHR.html) - Structure specifying axis-aligned bounding box geometry in a bottom-level acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryAabbsDataKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryAabbsDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkDeviceOrHostAddressConstKHR    data;
///    VkDeviceSize                     stride;
///} VkAccelerationStructureGeometryAabbsDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`data`] is a device or host address to memory containing [`AabbPositionsKHR`] structures
///   containing position data for each axis-aligned bounding box in the geometry.
/// - [`stride`] is the stride in bytes between each entry in [`data`]. The stride  **must**  be a
///   multiple of `8`.
///# Description
///## Valid Usage
/// - [`stride`] **must**  be a multiple of `8`
/// - [`stride`] **must**  be less than or equal to 2<sup>32</sup>-1
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_AABBS_DATA_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`DeviceSize`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryAabbsDataKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureGeometryAabbsDataKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`data`] is a device or host address to memory containing
    ///[`AabbPositionsKHR`] structures containing position data for each
    ///axis-aligned bounding box in the geometry.
    pub data: DeviceOrHostAddressConstKHR,
    ///[`stride`] is the stride in bytes between each entry in [`data`].
    ///The stride  **must**  be a multiple of `8`.
    pub stride: DeviceSize,
}
impl<'lt> Default for AccelerationStructureGeometryAabbsDataKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            data: Default::default(),
            stride: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureGeometryAabbsDataKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> DeviceOrHostAddressConstKHR {
        self.data
    }
    ///Gets the value of [`Self::stride`]
    pub fn stride(&self) -> DeviceSize {
        self.stride
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.data
    }
    ///Gets a mutable reference to the value of [`Self::stride`]
    pub fn stride_mut(&mut self) -> &mut DeviceSize {
        &mut self.stride
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.data = value;
        self
    }
    ///Sets the raw value of [`Self::stride`]
    pub fn set_stride(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.stride = value;
        self
    }
}
///[VkAccelerationStructureGeometryInstancesDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryInstancesDataKHR.html) - Structure specifying a geometry consisting of instances of other acceleration structures
///# C Specifications
///The [`AccelerationStructureGeometryInstancesDataKHR`] structure is
///defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryInstancesDataKHR {
///    VkStructureType                  sType;
///    const void*                      pNext;
///    VkBool32                         arrayOfPointers;
///    VkDeviceOrHostAddressConstKHR    data;
///} VkAccelerationStructureGeometryInstancesDataKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`array_of_pointers`] specifies whether [`data`] is used as an array of addresses or just an
///   array.
/// - [`data`] is either the address of an array of device or host addresses referencing individual [`AccelerationStructureInstanceKHR`] structures or packed motion instance information as described in [motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances) if [`array_of_pointers`] is [`TRUE`], or the address of an array of [`AccelerationStructureInstanceKHR`] or [`AccelerationStructureMotionInstanceNV`] structures. Addresses and [`AccelerationStructureInstanceKHR`] structures are tightly packed. [`AccelerationStructureMotionInstanceNV`] structures have a stride of 160 bytes.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_INSTANCES_DATA_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`Bool32`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryInstancesDataKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureGeometryInstancesDataKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`array_of_pointers`] specifies whether [`data`] is used as an array
    ///of addresses or just an array.
    pub array_of_pointers: Bool32,
    ///[`data`] is either the address of an array of device or host addresses
    ///referencing individual [`AccelerationStructureInstanceKHR`]
    ///structures
    ///or packed motion instance information as described in
    ///[motion instances](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#acceleration-structure-motion-instances)
    ///if [`array_of_pointers`] is [`TRUE`], or the address of an array of
    ///[`AccelerationStructureInstanceKHR`]
    ///or [`AccelerationStructureMotionInstanceNV`]
    ///structures.
    ///Addresses and [`AccelerationStructureInstanceKHR`] structures are
    ///tightly packed.
    ///[`AccelerationStructureMotionInstanceNV`] structures have a stride
    ///of 160 bytes.
    pub data: DeviceOrHostAddressConstKHR,
}
impl<'lt> Default for AccelerationStructureGeometryInstancesDataKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            array_of_pointers: 0,
            data: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureGeometryInstancesDataKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::array_of_pointers`]
    pub fn array_of_pointers_raw(&self) -> Bool32 {
        self.array_of_pointers
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::array_of_pointers`]
    pub fn set_array_of_pointers_raw(&mut self, value: Bool32) -> &mut Self {
        self.array_of_pointers = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::array_of_pointers`]
    pub fn array_of_pointers(&self) -> bool {
        unsafe { std::mem::transmute(self.array_of_pointers as u8) }
    }
    ///Gets the value of [`Self::data`]
    pub fn data(&self) -> DeviceOrHostAddressConstKHR {
        self.data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::array_of_pointers`]
    pub fn array_of_pointers_mut(&mut self) -> &mut bool {
        unsafe {
            if cfg!(target_endian = "little") {
                &mut *(self.array_of_pointers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .cast::<bool>()
            } else {
                eprintln!("Big-endianess has not been tested!");
                &mut *(self.array_of_pointers as *mut Bool32)
                    .cast::<u32>()
                    .cast::<u8>()
                    .add(3)
                    .cast::<bool>()
            }
        }
    }
    ///Gets a mutable reference to the value of [`Self::data`]
    pub fn data_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::array_of_pointers`]
    pub fn set_array_of_pointers(&mut self, value: bool) -> &mut Self {
        self.array_of_pointers = value as u8 as u32;
        self
    }
    ///Sets the raw value of [`Self::data`]
    pub fn set_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.data = value;
        self
    }
}
///[VkAccelerationStructureGeometryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryKHR.html) - Structure specifying geometries to be built into an acceleration structure
///# C Specifications
///The [`AccelerationStructureGeometryKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureGeometryKHR {
///    VkStructureType                           sType;
///    const void*                               pNext;
///    VkGeometryTypeKHR                         geometryType;
///    VkAccelerationStructureGeometryDataKHR    geometry;
///    VkGeometryFlagsKHR                        flags;
///} VkAccelerationStructureGeometryKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`geometry_type`] describes which type of geometry this [`AccelerationStructureGeometryKHR`]
///   refers to.
/// - [`geometry`] is a [`AccelerationStructureGeometryDataKHR`] union describing the geometry data
///   for the relevant geometry type.
/// - [`flags`] is a bitmask of [`GeometryFlagBitsKHR`] values describing additional properties of
///   how the geometry should be built.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_GEOMETRY_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`geometry_type`] **must**  be a valid [`GeometryTypeKHR`] value
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the `triangles` member of
///   [`geometry`] **must**  be a valid [`AccelerationStructureGeometryTrianglesDataKHR`] structure
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the `aabbs` member of [`geometry`]
///   **must**  be a valid [`AccelerationStructureGeometryAabbsDataKHR`] structure
/// - If [`geometry_type`] is `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the `instances` member of
///   [`geometry`] **must**  be a valid [`AccelerationStructureGeometryInstancesDataKHR`] structure
/// - [`flags`] **must**  be a valid combination of [`GeometryFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`AccelerationStructureGeometryDataKHR`]
/// - [`GeometryFlagsKHR`]
/// - [`GeometryTypeKHR`]
/// - [`StructureType`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureGeometryKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`geometry_type`] describes which type of geometry this
    ///[`AccelerationStructureGeometryKHR`] refers to.
    pub geometry_type: GeometryTypeKHR,
    ///[`geometry`] is a [`AccelerationStructureGeometryDataKHR`] union
    ///describing the geometry data for the relevant geometry type.
    pub geometry: AccelerationStructureGeometryDataKHR<'lt>,
    ///[`flags`] is a bitmask of [`GeometryFlagBitsKHR`] values
    ///describing additional properties of how the geometry should be built.
    pub flags: GeometryFlagsKHR,
}
impl<'lt> Default for AccelerationStructureGeometryKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            geometry_type: Default::default(),
            geometry: Default::default(),
            flags: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureGeometryKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::geometry_type`]
    pub fn geometry_type(&self) -> GeometryTypeKHR {
        self.geometry_type
    }
    ///Gets the value of [`Self::geometry`]
    pub fn geometry(&self) -> AccelerationStructureGeometryDataKHR<'lt> {
        self.geometry
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> GeometryFlagsKHR {
        self.flags
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::geometry_type`]
    pub fn geometry_type_mut(&mut self) -> &mut GeometryTypeKHR {
        &mut self.geometry_type
    }
    ///Gets a mutable reference to the value of [`Self::geometry`]
    pub fn geometry_mut(&mut self) -> &mut AccelerationStructureGeometryDataKHR<'lt> {
        &mut self.geometry
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut GeometryFlagsKHR {
        &mut self.flags
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::geometry_type`]
    pub fn set_geometry_type(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::GeometryTypeKHR,
    ) -> &mut Self {
        self.geometry_type = value;
        self
    }
    ///Sets the raw value of [`Self::geometry`]
    pub fn set_geometry(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryDataKHR<'lt>,
    ) -> &mut Self {
        self.geometry = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(&mut self, value: crate::extensions::khr_acceleration_structure::GeometryFlagsKHR) -> &mut Self {
        self.flags = value;
        self
    }
}
///[VkAccelerationStructureBuildGeometryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildGeometryInfoKHR.html) - Structure specifying the geometry data used to build an acceleration structure
///# C Specifications
///The [`AccelerationStructureBuildGeometryInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildGeometryInfoKHR {
///    VkStructureType                                     sType;
///    const void*                                         pNext;
///    VkAccelerationStructureTypeKHR                      type;
///    VkBuildAccelerationStructureFlagsKHR                flags;
///    VkBuildAccelerationStructureModeKHR                 mode;
///    VkAccelerationStructureKHR                          srcAccelerationStructure;
///    VkAccelerationStructureKHR                          dstAccelerationStructure;
///    uint32_t                                            geometryCount;
///    const VkAccelerationStructureGeometryKHR*           pGeometries;
///    const VkAccelerationStructureGeometryKHR* const*    ppGeometries;
///    VkDeviceOrHostAddressKHR                            scratchData;
///} VkAccelerationStructureBuildGeometryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration
///   structure being built.
/// - [`flags`] is a bitmask of [`BuildAccelerationStructureFlagBitsKHR`] specifying additional
///   parameters of the acceleration structure.
/// - [`mode`] is a [`BuildAccelerationStructureModeKHR`] value specifying the type of operation to
///   perform.
/// - [`src_acceleration_structure`] is a pointer to an existing acceleration structure that is to
///   be used to update the `dst` acceleration structure when [`mode`] is
///   `VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`.
/// - [`dst_acceleration_structure`] is a pointer to the target acceleration structure for the
///   build.
/// - [`geometry_count`] specifies the number of geometries that will be built into
///   [`dst_acceleration_structure`].
/// - [`geometries`] is a pointer to an array of [`AccelerationStructureGeometryKHR`] structures.
/// - [`pp_geometries`] is a pointer to an array of pointers to [`AccelerationStructureGeometryKHR`]
///   structures.
/// - [`scratch_data`] is the device or host address to memory that will be used as scratch memory
///   for the build.
///# Description
///Only one of [`geometries`] or [`pp_geometries`] **can**  be a valid pointer,
///the other  **must**  be `NULL`.
///Each element of the non-`NULL` array describes the data used to build each
///acceleration structure geometry.The index of each element of the [`geometries`] or
/// [`pp_geometries`]
///members of [`AccelerationStructureBuildGeometryInfoKHR`] is used as the
///*geometry index* during ray traversal.
///The geometry index is available in ray shaders via the
///[`RayGeometryIndexKHR`
///built-in](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#interfaces-builtin-variables-raygeometryindex), and is [used to
///determine hit and intersection shaders executed during traversal](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shader-binding-table-hit-shader-indexing).
///The geometry index is available to ray queries via the
///`OpRayQueryGetIntersectionGeometryIndexKHR` instruction.Setting
/// `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`]
///indicates that this build is a motion top level acceleration structure.
///A motion top level uses instances of format
///[`AccelerationStructureMotionInstanceNV`] if
///[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
///is [`FALSE`].If
///[`AccelerationStructureGeometryInstancesDataKHR::array_of_pointers`]
///is [`TRUE`], the pointer for any given element of the array of instance
///pointers consists of 4 bits of
///[`AccelerationStructureMotionInstanceTypeNV`] in the low 4 bits of the
///pointer identifying the type of structure at the pointer.
///The device address accessed is the value in the array with the low 4 bits
///set to zero.
///The structure at the pointer is one of
///[`AccelerationStructureInstanceKHR`],
///[`AccelerationStructureMatrixMotionInstanceNV`] or
///[`AccelerationStructureSrtMotionInstanceNV`], depending on the type
///value encoded in the low 4 bits.A top level acceleration structure with either motion instances
/// or vertex
///motion in its instances  **must**  set
///`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` in [`flags`].Members
/// [`src_acceleration_structure`] and [`dst_acceleration_structure`] **may**  be the same or
/// different for an update operation (when [`mode`] is
///`VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`).
///If they are the same, the update happens in-place.
///Otherwise, the target acceleration structure is updated and the source is
///not modified.
///## Valid Usage
/// - [`type_`] **must**  not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
/// - Only one of [`geometries`] or [`pp_geometries`] **can**  be a valid pointer, the other
///   **must**  be `NULL`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, the `geometryType` member of
///   elements of either [`geometries`] or [`pp_geometries`] **must**  be
///   `VK_GEOMETRY_TYPE_INSTANCES_KHR`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, [`geometry_count`] **must**
///   be `1`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` the `geometryType` member of
///   elements of either [`geometries`] or [`pp_geometries`] **must**  not be
///   `VK_GEOMETRY_TYPE_INSTANCES_KHR`
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then the `geometryType`
///   member of each geometry in either [`geometries`] or [`pp_geometries`] **must**  be the same
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` then [`geometry_count`]
///   **must**  be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_geometry_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType`
///   member of either [`geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_AABBS_KHR`, the
///   total number of AABBs in all geometries  **must**  be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
/// - If [`type_`] is `VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR` and the `geometryType`
///   member of either [`geometries`] or [`pp_geometries`] is `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the
///   total number of triangles in all geometries  **must**  be less than or equal to
///   [`PhysicalDeviceAccelerationStructurePropertiesKHR::max_primitive_count`]
/// - If [`flags`] has the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR` bit set, then
///   it  **must**  not have the `VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR` bit set
/// - If [`dst_acceleration_structure`] was created with
///   `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in
///   [`AccelerationStructureCreateInfoKHR`]::[`flags`],
///   `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` **must**  be set in [`flags`]
/// - If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`],
///   [`dst_acceleration_structure`] **must**  have been created with
///   `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` set in
///   [`AccelerationStructureCreateInfoKHR`]::[`flags`]
/// - If `VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` is set in [`flags`], [`type_`] **must**
///   not be `VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_GEOMETRY_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`type_`] **must**  be a valid [`AccelerationStructureTypeKHR`] value
/// - [`flags`] **must**  be a valid combination of [`BuildAccelerationStructureFlagBitsKHR`] values
/// - If [`geometry_count`] is not `0`, and [`geometries`] is not `NULL`, [`geometries`] **must**
///   be a valid pointer to an array of [`geometry_count`] valid
///   [`AccelerationStructureGeometryKHR`] structures
/// - If [`geometry_count`] is not `0`, and [`pp_geometries`] is not `NULL`, [`pp_geometries`]
///   **must**  be a valid pointer to an array of [`geometry_count`] valid pointers to valid
///   [`AccelerationStructureGeometryKHR`] structures
/// - Both of [`dst_acceleration_structure`], and [`src_acceleration_structure`] that are valid
///   handles of non-ignored parameters  **must**  have been created, allocated, or retrieved from
///   the same [`Device`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryKHR`]
/// - [`AccelerationStructureKHR`]
/// - [`AccelerationStructureTypeKHR`]
/// - [`BuildAccelerationStructureFlagsKHR`]
/// - [`BuildAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressKHR`]
/// - [`StructureType`]
/// - [`BuildAccelerationStructuresKHR`]
/// - [`CmdBuildAccelerationStructuresIndirectKHR`]
/// - [`CmdBuildAccelerationStructuresKHR`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureBuildGeometryInfoKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureBuildGeometryInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`type_`] is a [`AccelerationStructureTypeKHR`] value specifying
    ///the type of acceleration structure being built.
    pub type_: AccelerationStructureTypeKHR,
    ///[`flags`] is a bitmask of
    ///[`BuildAccelerationStructureFlagBitsKHR`] specifying additional
    ///parameters of the acceleration structure.
    pub flags: BuildAccelerationStructureFlagsKHR,
    ///[`mode`] is a [`BuildAccelerationStructureModeKHR`] value
    ///specifying the type of operation to perform.
    pub mode: BuildAccelerationStructureModeKHR,
    ///[`src_acceleration_structure`] is a pointer to an existing acceleration
    ///structure that is to be used to update the `dst` acceleration
    ///structure when [`mode`] is
    ///`VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR`.
    pub src_acceleration_structure: AccelerationStructureKHR,
    ///[`dst_acceleration_structure`] is a pointer to the target acceleration
    ///structure for the build.
    pub dst_acceleration_structure: AccelerationStructureKHR,
    ///[`geometry_count`] specifies the number of geometries that will be
    ///built into [`dst_acceleration_structure`].
    pub geometry_count: u32,
    ///[`geometries`] is a pointer to an array of
    ///[`AccelerationStructureGeometryKHR`] structures.
    pub geometries: *const AccelerationStructureGeometryKHR<'lt>,
    ///[`pp_geometries`] is a pointer to an array of pointers to
    ///[`AccelerationStructureGeometryKHR`] structures.
    pub pp_geometries: *const *const AccelerationStructureGeometryKHR<'lt>,
    ///[`scratch_data`] is the device or host address to memory that will be
    ///used as scratch memory for the build.
    pub scratch_data: DeviceOrHostAddressKHR,
}
impl<'lt> Default for AccelerationStructureBuildGeometryInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            type_: Default::default(),
            flags: Default::default(),
            mode: Default::default(),
            src_acceleration_structure: Default::default(),
            dst_acceleration_structure: Default::default(),
            geometry_count: 0,
            geometries: std::ptr::null(),
            pp_geometries: std::ptr::null(),
            scratch_data: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureBuildGeometryInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::geometries`]
    pub fn geometries_raw(&self) -> *const AccelerationStructureGeometryKHR<'lt> {
        self.geometries
    }
    ///Gets the raw value of [`Self::pp_geometries`]
    pub fn pp_geometries_raw(&self) -> *const *const AccelerationStructureGeometryKHR<'lt> {
        self.pp_geometries
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::geometries`]
    pub fn set_geometries_raw(&mut self, value: *const AccelerationStructureGeometryKHR<'lt>) -> &mut Self {
        self.geometries = value;
        self
    }
    ///Sets the raw value of [`Self::pp_geometries`]
    pub fn set_pp_geometries_raw(&mut self, value: *const *const AccelerationStructureGeometryKHR<'lt>) -> &mut Self {
        self.pp_geometries = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> AccelerationStructureTypeKHR {
        self.type_
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> BuildAccelerationStructureFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> BuildAccelerationStructureModeKHR {
        self.mode
    }
    ///Gets the value of [`Self::src_acceleration_structure`]
    pub fn src_acceleration_structure(&self) -> AccelerationStructureKHR {
        self.src_acceleration_structure
    }
    ///Gets the value of [`Self::dst_acceleration_structure`]
    pub fn dst_acceleration_structure(&self) -> AccelerationStructureKHR {
        self.dst_acceleration_structure
    }
    ///Gets the value of [`Self::geometry_count`]
    pub fn geometry_count(&self) -> u32 {
        self.geometry_count
    }
    ///Gets the value of [`Self::geometries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn geometries(&self) -> &[AccelerationStructureGeometryKHR<'lt>] {
        std::slice::from_raw_parts(self.geometries, self.geometry_count as usize)
    }
    ///Gets the value of [`Self::pp_geometries`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn pp_geometries(&self) -> &[*const AccelerationStructureGeometryKHR<'lt>] {
        std::slice::from_raw_parts(self.pp_geometries, self.geometry_count as usize)
    }
    ///Gets the value of [`Self::scratch_data`]
    pub fn scratch_data(&self) -> &DeviceOrHostAddressKHR {
        &self.scratch_data
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut AccelerationStructureTypeKHR {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut BuildAccelerationStructureFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut BuildAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Gets a mutable reference to the value of [`Self::src_acceleration_structure`]
    pub fn src_acceleration_structure_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.src_acceleration_structure
    }
    ///Gets a mutable reference to the value of [`Self::dst_acceleration_structure`]
    pub fn dst_acceleration_structure_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.dst_acceleration_structure
    }
    ///Gets a mutable reference to the value of [`Self::geometry_count`]
    pub fn geometry_count_mut(&mut self) -> &mut u32 {
        &mut self.geometry_count
    }
    ///Gets a mutable reference to the value of [`Self::scratch_data`]
    pub fn scratch_data_mut(&mut self) -> &mut DeviceOrHostAddressKHR {
        &mut self.scratch_data
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::BuildAccelerationStructureModeKHR,
    ) -> &mut Self {
        self.mode = value;
        self
    }
    ///Sets the raw value of [`Self::src_acceleration_structure`]
    pub fn set_src_acceleration_structure(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.src_acceleration_structure = value;
        self
    }
    ///Sets the raw value of [`Self::dst_acceleration_structure`]
    pub fn set_dst_acceleration_structure(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.dst_acceleration_structure = value;
        self
    }
    ///Sets the raw value of [`Self::geometry_count`]
    pub fn set_geometry_count(&mut self, value: u32) -> &mut Self {
        self.geometry_count = value;
        self
    }
    ///Sets the raw value of [`Self::geometries`]
    pub fn set_geometries(
        &mut self,
        value: &'lt [crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.geometries = value.as_ptr();
        self.geometry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::pp_geometries`]
    pub fn set_pp_geometries(
        &mut self,
        value: &'lt [*const crate::extensions::khr_acceleration_structure::AccelerationStructureGeometryKHR<'lt>],
    ) -> &mut Self {
        let len_ = value.len() as u32;
        let len_ = len_;
        self.pp_geometries = value.as_ptr();
        self.geometry_count = len_;
        self
    }
    ///Sets the raw value of [`Self::scratch_data`]
    pub fn set_scratch_data(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
    ) -> &mut Self {
        self.scratch_data = value;
        self
    }
}
///[VkAccelerationStructureBuildRangeInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildRangeInfoKHR.html) - Structure specifying build offsets and counts for acceleration structure builds
///# C Specifications
///[`AccelerationStructureBuildRangeInfoKHR`] is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildRangeInfoKHR {
///    uint32_t    primitiveCount;
///    uint32_t    primitiveOffset;
///    uint32_t    firstVertex;
///    uint32_t    transformOffset;
///} VkAccelerationStructureBuildRangeInfoKHR;
///```
///# Members
/// - [`primitive_count`] defines the number of primitives for a corresponding acceleration
///   structure geometry.
/// - [`primitive_offset`] defines an offset in bytes into the memory where primitive data is
///   defined.
/// - [`first_vertex`] is the index of the first vertex to build from for triangle geometry.
/// - [`transform_offset`] defines an offset in bytes into the memory where a transform matrix is
///   defined.
///# Description
///The primitive count and primitive offset are interpreted differently
///depending on the [`GeometryTypeKHR`] used:
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, [`primitive_count`] is the number of
///   triangles to be built, where each triangle is treated as 3 vertices.  - If the geometry uses
///   indices, [`primitive_count`] × 3 indices are consumed from
///   [`AccelerationStructureGeometryTrianglesDataKHR::index_data`], starting at an offset of
///   [`primitive_offset`]. The value of [`first_vertex`] is added to the index values before
///   fetching vertices.  - If the geometry does not use indices, [`primitive_count`] × 3 vertices
///   are consumed from [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`], starting at
///   an offset of [`primitive_offset`] +
///   [`AccelerationStructureGeometryTrianglesDataKHR::vertex_stride`] × [`first_vertex`].  - If
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`] is not `NULL`, a single
///   [`TransformMatrixKHR`] structure is consumed from
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`], at an offset of
///   [`transform_offset`]. This matrix describes a transformation from the space in which the
///   vertices for all triangles in this geometry are described to the space in which the
///   acceleration structure is defined.
/// - For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, [`primitive_count`] is the number of
///   axis-aligned bounding boxes. [`primitive_count`][`AabbPositionsKHR`] structures are consumed
///   from [`AccelerationStructureGeometryAabbsDataKHR::data`], starting at an offset of
///   [`primitive_offset`].
/// - For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, [`primitive_count`] is the number of
///   acceleration structures. [`primitive_count`][`AccelerationStructureInstanceKHR`] or
///   [`AccelerationStructureMotionInstanceNV`] structures are consumed from
///   [`AccelerationStructureGeometryInstancesDataKHR::data`], starting at an offset of
///   [`primitive_offset`].
///
///## Valid Usage
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry uses indices, the
///   offset [`primitive_offset`] from [`AccelerationStructureGeometryTrianglesDataKHR::index_data`]
///   **must**  be a multiple of the element size of
///   [`AccelerationStructureGeometryTrianglesDataKHR::index_type`]
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, if the geometry does not use indices,
///   the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryTrianglesDataKHR::vertex_data`] **must**  be a multiple of the
///   component size of [`AccelerationStructureGeometryTrianglesDataKHR::vertex_format`]
/// - For geometries of type `VK_GEOMETRY_TYPE_TRIANGLES_KHR`, the offset [`transform_offset`] from
///   [`AccelerationStructureGeometryTrianglesDataKHR::transform_data`] **must**  be a multiple of
///   16
/// - For geometries of type `VK_GEOMETRY_TYPE_AABBS_KHR`, the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryAabbsDataKHR::data`] **must**  be a multiple of 8
/// - For geometries of type `VK_GEOMETRY_TYPE_INSTANCES_KHR`, the offset [`primitive_offset`] from
///   [`AccelerationStructureGeometryInstancesDataKHR::data`] **must**  be a multiple of 16
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`BuildAccelerationStructuresKHR`]
/// - [`CmdBuildAccelerationStructuresKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureBuildRangeInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureBuildRangeInfoKHR {
    ///[`primitive_count`] defines the number of primitives for a
    ///corresponding acceleration structure geometry.
    pub primitive_count: u32,
    ///[`primitive_offset`] defines an offset in bytes into the memory where
    ///primitive data is defined.
    pub primitive_offset: u32,
    ///[`first_vertex`] is the index of the first vertex to build from for
    ///triangle geometry.
    pub first_vertex: u32,
    ///[`transform_offset`] defines an offset in bytes into the memory where a
    ///transform matrix is defined.
    pub transform_offset: u32,
}
impl Default for AccelerationStructureBuildRangeInfoKHR {
    fn default() -> Self {
        Self {
            primitive_count: 0,
            primitive_offset: 0,
            first_vertex: 0,
            transform_offset: 0,
        }
    }
}
impl AccelerationStructureBuildRangeInfoKHR {
    ///Gets the value of [`Self::primitive_count`]
    pub fn primitive_count(&self) -> u32 {
        self.primitive_count
    }
    ///Gets the value of [`Self::primitive_offset`]
    pub fn primitive_offset(&self) -> u32 {
        self.primitive_offset
    }
    ///Gets the value of [`Self::first_vertex`]
    pub fn first_vertex(&self) -> u32 {
        self.first_vertex
    }
    ///Gets the value of [`Self::transform_offset`]
    pub fn transform_offset(&self) -> u32 {
        self.transform_offset
    }
    ///Gets a mutable reference to the value of [`Self::primitive_count`]
    pub fn primitive_count_mut(&mut self) -> &mut u32 {
        &mut self.primitive_count
    }
    ///Gets a mutable reference to the value of [`Self::primitive_offset`]
    pub fn primitive_offset_mut(&mut self) -> &mut u32 {
        &mut self.primitive_offset
    }
    ///Gets a mutable reference to the value of [`Self::first_vertex`]
    pub fn first_vertex_mut(&mut self) -> &mut u32 {
        &mut self.first_vertex
    }
    ///Gets a mutable reference to the value of [`Self::transform_offset`]
    pub fn transform_offset_mut(&mut self) -> &mut u32 {
        &mut self.transform_offset
    }
    ///Sets the raw value of [`Self::primitive_count`]
    pub fn set_primitive_count(&mut self, value: u32) -> &mut Self {
        self.primitive_count = value;
        self
    }
    ///Sets the raw value of [`Self::primitive_offset`]
    pub fn set_primitive_offset(&mut self, value: u32) -> &mut Self {
        self.primitive_offset = value;
        self
    }
    ///Sets the raw value of [`Self::first_vertex`]
    pub fn set_first_vertex(&mut self, value: u32) -> &mut Self {
        self.first_vertex = value;
        self
    }
    ///Sets the raw value of [`Self::transform_offset`]
    pub fn set_transform_offset(&mut self, value: u32) -> &mut Self {
        self.transform_offset = value;
        self
    }
}
///[VkAccelerationStructureCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureCreateInfoKHR.html) - Structure specifying the parameters of a newly created acceleration structure object
///# C Specifications
///The [`AccelerationStructureCreateInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureCreateInfoKHR {
///    VkStructureType                          sType;
///    const void*                              pNext;
///    VkAccelerationStructureCreateFlagsKHR    createFlags;
///    VkBuffer                                 buffer;
///    VkDeviceSize                             offset;
///    VkDeviceSize                             size;
///    VkAccelerationStructureTypeKHR           type;
///    VkDeviceAddress                          deviceAddress;
///} VkAccelerationStructureCreateInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`create_flags`] is a bitmask of [`AccelerationStructureCreateFlagBitsKHR`] specifying
///   additional creation parameters of the acceleration structure.
/// - [`buffer`] is the buffer on which the acceleration structure will be stored.
/// - [`offset`] is an offset in bytes from the base address of the buffer at which the acceleration
///   structure will be stored, and  **must**  be a multiple of `256`.
/// - [`size`] is the size required for the acceleration structure.
/// - [`type_`] is a [`AccelerationStructureTypeKHR`] value specifying the type of acceleration
///   structure that will be created.
/// - [`device_address`] is the device address requested for the acceleration structure if the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay)
///   feature is being used.
///# Description
///If [`device_address`] is zero, no specific address is requested.If [`device_address`] is not
/// zero, [`device_address`] **must**  be an address
///retrieved from an identically created acceleration structure on the same
///implementation.
///The acceleration structure  **must**  also be placed on an identically created
///[`buffer`] and at the same [`offset`].Applications  **should**  avoid creating acceleration
/// structures with
///application-provided addresses and implementation-provided addresses in the
///same process, to reduce the likelihood of
///`VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR` errors.Applications  **should**  create an
/// acceleration structure with a specific
///[`AccelerationStructureTypeKHR`] other than
///`VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR`.If the acceleration structure will be the target of
/// a build operation, the
///required size for an acceleration structure  **can**  be queried with
///[`GetAccelerationStructureBuildSizesKHR`].
///If the acceleration structure is going to be the target of a compacting
///copy, [`CmdWriteAccelerationStructuresPropertiesKHR`] or
///[`WriteAccelerationStructuresPropertiesKHR`] **can**  be used to obtain the
///compacted size required.If the acceleration structure will be the target of a build operation
/// with
///`VK_BUILD_ACCELERATION_STRUCTURE_MOTION_BIT_NV` it  **must**  include
///`VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` in `flags` and
///include [`AccelerationStructureMotionInfoNV`] as an extension structure
///in [`p_next`] with the number of instances as metadata for the object.
///## Valid Usage
/// - If [`device_address`] is not zero, [`create_flags`] **must**  include
///   `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`
/// - If [`create_flags`] includes
///   `VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR`,
///   [`PhysicalDeviceAccelerationStructureFeaturesKHR::acceleration_structure_capture_replay`]
///   **must**  be [`TRUE`]
/// - [`buffer`] **must**  have been created with a `usage` value containing
///   `VK_BUFFER_USAGE_ACCELERATION_STRUCTURE_STORAGE_BIT_KHR`
/// - [`buffer`] **must**  not have been created with `VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT`
/// - The sum of [`offset`] and [`size`] **must**  be less than the size of [`buffer`]
/// - [`offset`] **must**  be a multiple of `256` bytes
/// - If `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV` is set in    `flags` and [`type_`] is
///   `VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR`, one member of the [`p_next`] chain  **must**
///   be a pointer to a valid instance of [`AccelerationStructureMotionInfoNV`]
/// - If any geometry includes [`AccelerationStructureGeometryMotionTrianglesDataNV`] then `flags`
///   **must**  contain `VK_ACCELERATION_STRUCTURE_CREATE_MOTION_BIT_NV`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_CREATE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL` or a pointer to a valid instance of
///   [`AccelerationStructureMotionInfoNV`]
/// - The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
/// - [`create_flags`] **must**  be a valid combination of
///   [`AccelerationStructureCreateFlagBitsKHR`] values
/// - [`buffer`] **must**  be a valid [`Buffer`] handle
/// - [`type_`] **must**  be a valid [`AccelerationStructureTypeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureCreateFlagsKHR`]
/// - [`AccelerationStructureTypeKHR`]
/// - [`Buffer`]
/// - [`DeviceAddress`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`CreateAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureCreateInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureCreateInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`create_flags`] is a bitmask of
    ///[`AccelerationStructureCreateFlagBitsKHR`] specifying additional
    ///creation parameters of the acceleration structure.
    pub create_flags: AccelerationStructureCreateFlagsKHR,
    ///[`buffer`] is the buffer on which the acceleration structure will be
    ///stored.
    pub buffer: Buffer,
    ///[`offset`] is an offset in bytes from the base address of the buffer
    ///at which the acceleration structure will be stored, and  **must**  be a
    ///multiple of `256`.
    pub offset: DeviceSize,
    ///[`size`] is the size required for the acceleration structure.
    pub size: DeviceSize,
    ///[`type_`] is a [`AccelerationStructureTypeKHR`] value specifying
    ///the type of acceleration structure that will be created.
    pub type_: AccelerationStructureTypeKHR,
    ///[`device_address`] is the device address requested for the acceleration
    ///structure if the [`accelerationStructureCaptureReplay`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-accelerationStructureCaptureReplay) feature is being used.
    pub device_address: DeviceAddress,
}
impl<'lt> Default for AccelerationStructureCreateInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            create_flags: Default::default(),
            buffer: Default::default(),
            offset: Default::default(),
            size: Default::default(),
            type_: Default::default(),
            device_address: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureCreateInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::create_flags`]
    pub fn create_flags(&self) -> AccelerationStructureCreateFlagsKHR {
        self.create_flags
    }
    ///Gets the value of [`Self::buffer`]
    pub fn buffer(&self) -> Buffer {
        self.buffer
    }
    ///Gets the value of [`Self::offset`]
    pub fn offset(&self) -> DeviceSize {
        self.offset
    }
    ///Gets the value of [`Self::size`]
    pub fn size(&self) -> DeviceSize {
        self.size
    }
    ///Gets the value of [`Self::type_`]
    pub fn type_(&self) -> AccelerationStructureTypeKHR {
        self.type_
    }
    ///Gets the value of [`Self::device_address`]
    pub fn device_address(&self) -> DeviceAddress {
        self.device_address
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::create_flags`]
    pub fn create_flags_mut(&mut self) -> &mut AccelerationStructureCreateFlagsKHR {
        &mut self.create_flags
    }
    ///Gets a mutable reference to the value of [`Self::buffer`]
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
    ///Gets a mutable reference to the value of [`Self::offset`]
    pub fn offset_mut(&mut self) -> &mut DeviceSize {
        &mut self.offset
    }
    ///Gets a mutable reference to the value of [`Self::size`]
    pub fn size_mut(&mut self) -> &mut DeviceSize {
        &mut self.size
    }
    ///Gets a mutable reference to the value of [`Self::type_`]
    pub fn type_mut(&mut self) -> &mut AccelerationStructureTypeKHR {
        &mut self.type_
    }
    ///Gets a mutable reference to the value of [`Self::device_address`]
    pub fn device_address_mut(&mut self) -> &mut DeviceAddress {
        &mut self.device_address
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::create_flags`]
    pub fn set_create_flags(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureCreateFlagsKHR,
    ) -> &mut Self {
        self.create_flags = value;
        self
    }
    ///Sets the raw value of [`Self::buffer`]
    pub fn set_buffer(&mut self, value: crate::vulkan1_0::Buffer) -> &mut Self {
        self.buffer = value;
        self
    }
    ///Sets the raw value of [`Self::offset`]
    pub fn set_offset(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.offset = value;
        self
    }
    ///Sets the raw value of [`Self::size`]
    pub fn set_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.size = value;
        self
    }
    ///Sets the raw value of [`Self::type_`]
    pub fn set_type_(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureTypeKHR,
    ) -> &mut Self {
        self.type_ = value;
        self
    }
    ///Sets the raw value of [`Self::device_address`]
    pub fn set_device_address(&mut self, value: crate::vulkan1_0::DeviceAddress) -> &mut Self {
        self.device_address = value;
        self
    }
}
///[VkAabbPositionsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAabbPositionsKHR.html) - Structure specifying two opposing corners of an axis-aligned bounding box
///# C Specifications
///The [`AabbPositionsKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAabbPositionsKHR {
///    float    minX;
///    float    minY;
///    float    minZ;
///    float    maxX;
///    float    maxY;
///    float    maxZ;
///} VkAabbPositionsKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAabbPositionsKHR VkAabbPositionsNV;
///```
///# Members
/// - [`min_x`] is the x position of one opposing corner of a bounding box.
/// - [`min_y`] is the y position of one opposing corner of a bounding box.
/// - [`min_z`] is the z position of one opposing corner of a bounding box.
/// - [`max_x`] is the x position of the other opposing corner of a bounding box.
/// - [`max_y`] is the y position of the other opposing corner of a bounding box.
/// - [`max_z`] is the z position of the other opposing corner of a bounding box.
///# Description
///## Valid Usage
/// - [`min_x`] **must**  be less than or equal to [`max_x`]
/// - [`min_y`] **must**  be less than or equal to [`max_y`]
/// - [`min_z`] **must**  be less than or equal to [`max_z`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAabbPositionsKHR")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AabbPositionsKHR {
    ///[`min_x`] is the x position of one opposing corner of a bounding box.
    pub min_x: f32,
    ///[`min_y`] is the y position of one opposing corner of a bounding box.
    pub min_y: f32,
    ///[`min_z`] is the z position of one opposing corner of a bounding box.
    pub min_z: f32,
    ///[`max_x`] is the x position of the other opposing corner of a bounding
    ///box.
    pub max_x: f32,
    ///[`max_y`] is the y position of the other opposing corner of a bounding
    ///box.
    pub max_y: f32,
    ///[`max_z`] is the z position of the other opposing corner of a bounding
    ///box.
    pub max_z: f32,
}
impl Default for AabbPositionsKHR {
    fn default() -> Self {
        Self {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,
        }
    }
}
impl AabbPositionsKHR {
    ///Gets the value of [`Self::min_x`]
    pub fn min_x(&self) -> f32 {
        self.min_x
    }
    ///Gets the value of [`Self::min_y`]
    pub fn min_y(&self) -> f32 {
        self.min_y
    }
    ///Gets the value of [`Self::min_z`]
    pub fn min_z(&self) -> f32 {
        self.min_z
    }
    ///Gets the value of [`Self::max_x`]
    pub fn max_x(&self) -> f32 {
        self.max_x
    }
    ///Gets the value of [`Self::max_y`]
    pub fn max_y(&self) -> f32 {
        self.max_y
    }
    ///Gets the value of [`Self::max_z`]
    pub fn max_z(&self) -> f32 {
        self.max_z
    }
    ///Gets a mutable reference to the value of [`Self::min_x`]
    pub fn min_x_mut(&mut self) -> &mut f32 {
        &mut self.min_x
    }
    ///Gets a mutable reference to the value of [`Self::min_y`]
    pub fn min_y_mut(&mut self) -> &mut f32 {
        &mut self.min_y
    }
    ///Gets a mutable reference to the value of [`Self::min_z`]
    pub fn min_z_mut(&mut self) -> &mut f32 {
        &mut self.min_z
    }
    ///Gets a mutable reference to the value of [`Self::max_x`]
    pub fn max_x_mut(&mut self) -> &mut f32 {
        &mut self.max_x
    }
    ///Gets a mutable reference to the value of [`Self::max_y`]
    pub fn max_y_mut(&mut self) -> &mut f32 {
        &mut self.max_y
    }
    ///Gets a mutable reference to the value of [`Self::max_z`]
    pub fn max_z_mut(&mut self) -> &mut f32 {
        &mut self.max_z
    }
    ///Sets the raw value of [`Self::min_x`]
    pub fn set_min_x(&mut self, value: f32) -> &mut Self {
        self.min_x = value;
        self
    }
    ///Sets the raw value of [`Self::min_y`]
    pub fn set_min_y(&mut self, value: f32) -> &mut Self {
        self.min_y = value;
        self
    }
    ///Sets the raw value of [`Self::min_z`]
    pub fn set_min_z(&mut self, value: f32) -> &mut Self {
        self.min_z = value;
        self
    }
    ///Sets the raw value of [`Self::max_x`]
    pub fn set_max_x(&mut self, value: f32) -> &mut Self {
        self.max_x = value;
        self
    }
    ///Sets the raw value of [`Self::max_y`]
    pub fn set_max_y(&mut self, value: f32) -> &mut Self {
        self.max_y = value;
        self
    }
    ///Sets the raw value of [`Self::max_z`]
    pub fn set_max_z(&mut self, value: f32) -> &mut Self {
        self.max_z = value;
        self
    }
}
///[VkTransformMatrixKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTransformMatrixKHR.html) - Structure specifying a 3x4 affine transformation matrix
///# C Specifications
///The [`TransformMatrixKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkTransformMatrixKHR {
///    float    matrix[3][4];
///} VkTransformMatrixKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkTransformMatrixKHR VkTransformMatrixNV;
///```
///# Members
/// - [`matrix`] is a 3x4 row-major affine transformation matrix.
///# Description
///## Valid Usage
/// - The first three columns of [`matrix`] **must**  define an invertible 3x3 matrix
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureInstanceKHR`]
/// - [`AccelerationStructureMatrixMotionInstanceNV`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkTransformMatrixKHR")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TransformMatrixKHR {
    ///[`matrix`] is a 3x4 row-major affine transformation matrix.
    pub matrix: [f32; 3 as usize],
}
impl Default for TransformMatrixKHR {
    fn default() -> Self {
        Self {
            matrix: [0.0; 3 as usize],
        }
    }
}
impl TransformMatrixKHR {
    ///Gets the value of [`Self::matrix`]
    pub fn matrix(&self) -> &[f32; 3 as usize] {
        &self.matrix
    }
    ///Gets a mutable reference to the value of [`Self::matrix`]
    pub fn matrix_mut(&mut self) -> &mut [f32; 3 as usize] {
        &mut self.matrix
    }
    ///Sets the raw value of [`Self::matrix`]
    pub fn set_matrix(&mut self, value: [f32; 3 as usize]) -> &mut Self {
        self.matrix = value;
        self
    }
}
///[VkAccelerationStructureInstanceKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureInstanceKHR.html) - Structure specifying a single acceleration structure instance for building into an acceleration structure geometry
///# C Specifications
///*Acceleration structure instances* **can**  be built into top-level acceleration
///structures.
///Each acceleration structure instance is a separate entry in the top-level
///acceleration structure which includes all the geometry of a bottom-level
///acceleration structure at a transformed location.
///Multiple instances  **can**  point to the same bottom level acceleration
///structure.An acceleration structure instance is defined by the structure:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureInstanceKHR {
///    VkTransformMatrixKHR          transform;
///    uint32_t                      instanceCustomIndex:24;
///    uint32_t                      mask:8;
///    uint32_t                      instanceShaderBindingTableRecordOffset:24;
///    VkGeometryInstanceFlagsKHR    flags:8;
///    uint64_t                      accelerationStructureReference;
///} VkAccelerationStructureInstanceKHR;
///```
///or the equivalent
///```c
///// Provided by VK_NV_ray_tracing
///typedef VkAccelerationStructureInstanceKHR VkAccelerationStructureInstanceNV;
///```
///# Members
/// - [`transform`] is a [`TransformMatrixKHR`] structure describing a transformation to be applied
///   to the acceleration structure.
/// - [`instance_custom_index`] is a 24-bit user-specified index value accessible to ray shaders in
///   the `InstanceCustomIndexKHR` built-in.
/// - [`mask`] is an 8-bit visibility mask for the geometry. The instance  **may**  only be hit if
///   `Cull Mask & instance.mask != 0`
/// - [`instance_shader_binding_table_record_offset`] is a 24-bit offset used in calculating the hit
///   shader binding table index.
/// - [`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`] values to apply to this
///   instance.
/// - [`acceleration_structure_reference`] is either:  - a device address containing the value
///   obtained from [`GetAccelerationStructureDeviceAddressKHR`] or
///   [`GetAccelerationStructureHandleNV`]      (used by device operations which reference
///   acceleration structures) or,  - a [`AccelerationStructureKHR`] object (used by host operations
///   which reference acceleration structures).
///# Description
///The C language specification does not define the ordering of bit-fields, but
///in practice, this struct produces the correct layout with existing
///compilers.
///The intended bit pattern is for the following:
/// - [`instance_custom_index`] and [`mask`] occupy the same memory as if a single `uint32_t` was
///   specified in their place  - [`instance_custom_index`] occupies the 24 least significant bits
///   of that memory  - [`mask`] occupies the 8 most significant bits of that memory
/// - [`instance_shader_binding_table_record_offset`] and [`flags`] occupy the same memory as if a
///   single `uint32_t` was specified in their place  -
///   [`instance_shader_binding_table_record_offset`] occupies the 24 least significant bits of that
///   memory  - [`flags`] occupies the 8 most significant bits of that memory
///If a compiler produces code that diverges from that pattern, applications
/// **must**  employ another method to set values according to the correct bit
///pattern.
///## Valid Usage (Implicit)
/// - [`flags`] **must**  be a valid combination of [`GeometryInstanceFlagBitsKHR`] values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`VK_NV_ray_tracing`]
/// - [`AccelerationStructureMotionInstanceDataNV`]
/// - [`GeometryInstanceFlagsKHR`]
/// - [`TransformMatrixKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureInstanceKHR")]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct AccelerationStructureInstanceKHR {
    ///[`transform`] is a [`TransformMatrixKHR`] structure describing a
    ///transformation to be applied to the acceleration structure.
    pub transform: TransformMatrixKHR,
    ///[`instance_custom_index`] is a 24-bit user-specified index value
    ///accessible to ray shaders in the `InstanceCustomIndexKHR` built-in.
    pub instance_custom_index: u32,
    ///[`mask`] is an 8-bit visibility mask for the geometry.
    ///The instance  **may**  only be hit if `Cull Mask & instance.mask != 0`
    pub mask: u32,
    ///[`instance_shader_binding_table_record_offset`] is a 24-bit offset used in
    ///calculating the hit shader binding table index.
    pub instance_shader_binding_table_record_offset: u32,
    ///[`flags`] is an 8-bit mask of [`GeometryInstanceFlagBitsKHR`]
    ///values to apply to this instance.
    pub flags: GeometryInstanceFlagsKHR,
    ///[`acceleration_structure_reference`] is either:
    /// - a device address containing the value obtained from
    ///   [`GetAccelerationStructureDeviceAddressKHR`] or [`GetAccelerationStructureHandleNV`]
    ///   (used by device operations which reference acceleration structures) or,
    /// - a [`AccelerationStructureKHR`] object (used by host operations which reference
    ///   acceleration structures).
    pub acceleration_structure_reference: u64,
}
impl Default for AccelerationStructureInstanceKHR {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            instance_custom_index: 0,
            mask: 0,
            instance_shader_binding_table_record_offset: 0,
            flags: Default::default(),
            acceleration_structure_reference: 0,
        }
    }
}
impl AccelerationStructureInstanceKHR {
    ///Gets the value of [`Self::transform`]
    pub fn transform(&self) -> TransformMatrixKHR {
        self.transform
    }
    ///Gets the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index(&self) -> u32 {
        self.instance_custom_index
    }
    ///Gets the value of [`Self::mask`]
    pub fn mask(&self) -> u32 {
        self.mask
    }
    ///Gets the value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset(&self) -> u32 {
        self.instance_shader_binding_table_record_offset
    }
    ///Gets the value of [`Self::flags`]
    pub fn flags(&self) -> GeometryInstanceFlagsKHR {
        self.flags
    }
    ///Gets the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference(&self) -> u64 {
        self.acceleration_structure_reference
    }
    ///Gets a mutable reference to the value of [`Self::transform`]
    pub fn transform_mut(&mut self) -> &mut TransformMatrixKHR {
        &mut self.transform
    }
    ///Gets a mutable reference to the value of [`Self::instance_custom_index`]
    pub fn instance_custom_index_mut(&mut self) -> &mut u32 {
        &mut self.instance_custom_index
    }
    ///Gets a mutable reference to the value of [`Self::mask`]
    pub fn mask_mut(&mut self) -> &mut u32 {
        &mut self.mask
    }
    ///Gets a mutable reference to the value of
    /// [`Self::instance_shader_binding_table_record_offset`]
    pub fn instance_shader_binding_table_record_offset_mut(&mut self) -> &mut u32 {
        &mut self.instance_shader_binding_table_record_offset
    }
    ///Gets a mutable reference to the value of [`Self::flags`]
    pub fn flags_mut(&mut self) -> &mut GeometryInstanceFlagsKHR {
        &mut self.flags
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_reference`]
    pub fn acceleration_structure_reference_mut(&mut self) -> &mut u64 {
        &mut self.acceleration_structure_reference
    }
    ///Sets the raw value of [`Self::transform`]
    pub fn set_transform(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::TransformMatrixKHR,
    ) -> &mut Self {
        self.transform = value;
        self
    }
    ///Sets the raw value of [`Self::instance_custom_index`]
    pub fn set_instance_custom_index(&mut self, value: u32) -> &mut Self {
        self.instance_custom_index = value;
        self
    }
    ///Sets the raw value of [`Self::mask`]
    pub fn set_mask(&mut self, value: u32) -> &mut Self {
        self.mask = value;
        self
    }
    ///Sets the raw value of [`Self::instance_shader_binding_table_record_offset`]
    pub fn set_instance_shader_binding_table_record_offset(&mut self, value: u32) -> &mut Self {
        self.instance_shader_binding_table_record_offset = value;
        self
    }
    ///Sets the raw value of [`Self::flags`]
    pub fn set_flags(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::GeometryInstanceFlagsKHR,
    ) -> &mut Self {
        self.flags = value;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_reference`]
    pub fn set_acceleration_structure_reference(&mut self, value: u64) -> &mut Self {
        self.acceleration_structure_reference = value;
        self
    }
}
///[VkAccelerationStructureDeviceAddressInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureDeviceAddressInfoKHR.html) - Structure specifying the acceleration structure to query an address for
///# C Specifications
///The [`AccelerationStructureDeviceAddressInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureDeviceAddressInfoKHR {
///    VkStructureType               sType;
///    const void*                   pNext;
///    VkAccelerationStructureKHR    accelerationStructure;
///} VkAccelerationStructureDeviceAddressInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure`] specifies the acceleration structure whose address is being
///   queried.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_DEVICE_ADDRESS_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`acceleration_structure`] **must**  be a valid [`AccelerationStructureKHR`] handle
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`StructureType`]
/// - [`GetAccelerationStructureDeviceAddressKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureDeviceAddressInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureDeviceAddressInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`acceleration_structure`] specifies the acceleration structure whose
    ///address is being queried.
    pub acceleration_structure: AccelerationStructureKHR,
}
impl<'lt> Default for AccelerationStructureDeviceAddressInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            acceleration_structure: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureDeviceAddressInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure(&self) -> AccelerationStructureKHR {
        self.acceleration_structure
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure`]
    pub fn acceleration_structure_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.acceleration_structure
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure`]
    pub fn set_acceleration_structure(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.acceleration_structure = value;
        self
    }
}
///[VkAccelerationStructureVersionInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureVersionInfoKHR.html) - Acceleration structure version information
///# C Specifications
///The [`AccelerationStructureVersionInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureVersionInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    const uint8_t*     pVersionData;
///} VkAccelerationStructureVersionInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`version_data`] is a pointer to the version header of an acceleration structure as defined in
///   [`CmdCopyAccelerationStructureToMemoryKHR`]
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_VERSION_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`version_data`] **must**  be a valid pointer to an array of <span class="katex"><span
///   class="katex-html" aria-hidden="true"><span class="base"><span class="strut"
///   style="height:0.72777em;vertical-align:-0.08333em;"></span><span class="mord">2</span><span
///   style="margin-right:0.2222222222222222em;" class="mspace"></span><span
///   class="mbin">×</span><span class="mspace"
///   style="margin-right:0.2222222222222222em;"></span></span><span class="base"><span
///   class="strut" style="height:0.70625em;vertical-align:-0.09514em;"></span><span
///   class="mord"><span class="mord mathtt">V</span><span class="mord mathtt">K</span><span
///   class="mord mathtt">_</span><span class="mord mathtt">U</span><span class="mord
///   mathtt">U</span><span class="mord mathtt">I</span><span class="mord mathtt">D</span><span
///   class="mord mathtt">_</span><span class="mord mathtt">S</span><span class="mord
///   mathtt">I</span><span class="mord mathtt">Z</span><span class="mord
///   mathtt">E</span></span></span></span></span>`uint8_t` values
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`StructureType`]
/// - [`GetDeviceAccelerationStructureCompatibilityKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureVersionInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureVersionInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`version_data`] is a pointer to the version header of an acceleration
    ///structure as defined in [`CmdCopyAccelerationStructureToMemoryKHR`]
    pub version_data: *const u8,
}
impl<'lt> Default for AccelerationStructureVersionInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            version_data: std::ptr::null(),
        }
    }
}
impl<'lt> AccelerationStructureVersionInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Gets the raw value of [`Self::version_data`]
    pub fn version_data_raw(&self) -> *const u8 {
        self.version_data
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Sets the raw value of [`Self::version_data`]
    pub fn set_version_data_raw(&mut self, value: *const u8) -> &mut Self {
        self.version_data = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::version_data`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn version_data(&self) -> &[u8] {
        std::slice::from_raw_parts(self.version_data, (2 * crate::core::UUID_SIZE) as usize)
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::version_data`]
    pub fn set_version_data(&mut self, value: &'lt [u8]) -> &mut Self {
        assert_eq!(value.len(), (2 * crate::core::UUID_SIZE) as usize);
        self.version_data = value.as_ptr();
        self
    }
}
///[VkCopyAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureInfoKHR.html) - Parameters for copying an acceleration structure
///# C Specifications
///The [`CopyAccelerationStructureInfoKHR`] structure is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyAccelerationStructureInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkAccelerationStructureKHR            src;
///    VkAccelerationStructureKHR            dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyAccelerationStructureInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the source acceleration structure for the copy.
/// - [`dst`] is the target acceleration structure for the copy.
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///## Valid Usage
/// - [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR` or
///   `VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR`
/// - The source acceleration structure [`src`] **must**  have been constructed prior to the
///   execution of this command
/// - If [`mode`] is `VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR`, [`src`] **must**  have been
///   constructed with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` in the build
/// - The `buffer` used to create [`src`] **must**  be bound to device memory
/// - The `buffer` used to create [`dst`] **must**  be bound to device memory
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`src`] **must**  be a valid [`AccelerationStructureKHR`] handle
/// - [`dst`] **must**  be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
/// - Both of [`dst`], and [`src`] **must**  have been created, allocated, or retrieved from the
///   same [`Device`]
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`StructureType`]
/// - [`CmdCopyAccelerationStructureKHR`]
/// - [`CopyAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyAccelerationStructureInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CopyAccelerationStructureInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src`] is the source acceleration structure for the copy.
    pub src: AccelerationStructureKHR,
    ///[`dst`] is the target acceleration structure for the copy.
    pub dst: AccelerationStructureKHR,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    pub mode: CopyAccelerationStructureModeKHR,
}
impl<'lt> Default for CopyAccelerationStructureInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
impl<'lt> CopyAccelerationStructureInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::src`]
    pub fn src(&self) -> AccelerationStructureKHR {
        self.src
    }
    ///Gets the value of [`Self::dst`]
    pub fn dst(&self) -> AccelerationStructureKHR {
        self.dst
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src`]
    pub fn src_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.src
    }
    ///Gets a mutable reference to the value of [`Self::dst`]
    pub fn dst_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.dst
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::src`]
    pub fn set_src(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.src = value;
        self
    }
    ///Sets the raw value of [`Self::dst`]
    pub fn set_dst(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.dst = value;
        self
    }
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) -> &mut Self {
        self.mode = value;
        self
    }
}
///[VkCopyAccelerationStructureToMemoryInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyAccelerationStructureToMemoryInfoKHR.html) - Parameters for serializing an acceleration structure
///# C Specifications
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyAccelerationStructureToMemoryInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkAccelerationStructureKHR            src;
///    VkDeviceOrHostAddressKHR              dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyAccelerationStructureToMemoryInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the source acceleration structure for the copy
/// - [`dst`] is the device or host address to memory which is the target for the copy
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///## Valid Usage
/// - The source acceleration structure [`src`] **must**  have been constructed prior to the
///   execution of this command
/// - The memory pointed to by [`dst`] **must**  be at least as large as the serialization size of
///   [`src`], as reported by [`WriteAccelerationStructuresPropertiesKHR`] or
///   [`CmdWriteAccelerationStructuresPropertiesKHR`] with a query type of
///   `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
/// - [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_SERIALIZE_KHR`
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_ACCELERATION_STRUCTURE_TO_MEMORY_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`src`] **must**  be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressKHR`]
/// - [`StructureType`]
/// - [`CmdCopyAccelerationStructureToMemoryKHR`]
/// - [`CopyAccelerationStructureToMemoryKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyAccelerationStructureToMemoryInfoKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CopyAccelerationStructureToMemoryInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src`] is the source acceleration structure for the copy
    pub src: AccelerationStructureKHR,
    ///[`dst`] is the device or host address to memory which is the target
    ///for the copy
    pub dst: DeviceOrHostAddressKHR,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    pub mode: CopyAccelerationStructureModeKHR,
}
impl<'lt> Default for CopyAccelerationStructureToMemoryInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
impl<'lt> CopyAccelerationStructureToMemoryInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::src`]
    pub fn src(&self) -> AccelerationStructureKHR {
        self.src
    }
    ///Gets the value of [`Self::dst`]
    pub fn dst(&self) -> &DeviceOrHostAddressKHR {
        &self.dst
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src`]
    pub fn src_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.src
    }
    ///Gets a mutable reference to the value of [`Self::dst`]
    pub fn dst_mut(&mut self) -> &mut DeviceOrHostAddressKHR {
        &mut self.dst
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::src`]
    pub fn set_src(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.src = value;
        self
    }
    ///Sets the raw value of [`Self::dst`]
    pub fn set_dst(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressKHR,
    ) -> &mut Self {
        self.dst = value;
        self
    }
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) -> &mut Self {
        self.mode = value;
        self
    }
}
///[VkCopyMemoryToAccelerationStructureInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCopyMemoryToAccelerationStructureInfoKHR.html) - Parameters for deserializing an acceleration structure
///# C Specifications
///The [`CopyMemoryToAccelerationStructureInfoKHR`] structure is defined
///as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkCopyMemoryToAccelerationStructureInfoKHR {
///    VkStructureType                       sType;
///    const void*                           pNext;
///    VkDeviceOrHostAddressConstKHR         src;
///    VkAccelerationStructureKHR            dst;
///    VkCopyAccelerationStructureModeKHR    mode;
///} VkCopyMemoryToAccelerationStructureInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`src`] is the device or host address to memory containing the source data for the copy.
/// - [`dst`] is the target acceleration structure for the copy.
/// - [`mode`] is a [`CopyAccelerationStructureModeKHR`] value specifying additional operations to
///   perform during the copy.
///# Description
///## Valid Usage
/// - The source memory pointed to by [`src`] **must**  contain data previously serialized using
///   [`CmdCopyAccelerationStructureToMemoryKHR`], potentially modified to relocate acceleration
///   structure references as described in that command
/// - [`mode`] **must**  be `VK_COPY_ACCELERATION_STRUCTURE_MODE_DESERIALIZE_KHR`
/// - The data in [`src`] **must**  have a format compatible with the destination physical device as
///   returned by [`GetDeviceAccelerationStructureCompatibilityKHR`]
/// - [`dst`] **must**  have been created with a `size` greater than or equal to that used to
///   serialize the data in [`src`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_COPY_MEMORY_TO_ACCELERATION_STRUCTURE_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
/// - [`dst`] **must**  be a valid [`AccelerationStructureKHR`] handle
/// - [`mode`] **must**  be a valid [`CopyAccelerationStructureModeKHR`] value
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureKHR`]
/// - [`CopyAccelerationStructureModeKHR`]
/// - [`DeviceOrHostAddressConstKHR`]
/// - [`StructureType`]
/// - [`CmdCopyMemoryToAccelerationStructureKHR`]
/// - [`CopyMemoryToAccelerationStructureKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCopyMemoryToAccelerationStructureInfoKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CopyMemoryToAccelerationStructureInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`src`] is the device or host address to memory containing the source
    ///data for the copy.
    pub src: DeviceOrHostAddressConstKHR,
    ///[`dst`] is the target acceleration structure for the copy.
    pub dst: AccelerationStructureKHR,
    ///[`mode`] is a [`CopyAccelerationStructureModeKHR`] value
    ///specifying additional operations to perform during the copy.
    pub mode: CopyAccelerationStructureModeKHR,
}
impl<'lt> Default for CopyMemoryToAccelerationStructureInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            src: Default::default(),
            dst: Default::default(),
            mode: Default::default(),
        }
    }
}
impl<'lt> CopyMemoryToAccelerationStructureInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::src`]
    pub fn src(&self) -> DeviceOrHostAddressConstKHR {
        self.src
    }
    ///Gets the value of [`Self::dst`]
    pub fn dst(&self) -> AccelerationStructureKHR {
        self.dst
    }
    ///Gets the value of [`Self::mode`]
    pub fn mode(&self) -> CopyAccelerationStructureModeKHR {
        self.mode
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::src`]
    pub fn src_mut(&mut self) -> &mut DeviceOrHostAddressConstKHR {
        &mut self.src
    }
    ///Gets a mutable reference to the value of [`Self::dst`]
    pub fn dst_mut(&mut self) -> &mut AccelerationStructureKHR {
        &mut self.dst
    }
    ///Gets a mutable reference to the value of [`Self::mode`]
    pub fn mode_mut(&mut self) -> &mut CopyAccelerationStructureModeKHR {
        &mut self.mode
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::src`]
    pub fn set_src(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::DeviceOrHostAddressConstKHR,
    ) -> &mut Self {
        self.src = value;
        self
    }
    ///Sets the raw value of [`Self::dst`]
    pub fn set_dst(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::AccelerationStructureKHR,
    ) -> &mut Self {
        self.dst = value;
        self
    }
    ///Sets the raw value of [`Self::mode`]
    pub fn set_mode(
        &mut self,
        value: crate::extensions::khr_acceleration_structure::CopyAccelerationStructureModeKHR,
    ) -> &mut Self {
        self.mode = value;
        self
    }
}
///[VkAccelerationStructureBuildSizesInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureBuildSizesInfoKHR.html) - Structure specifying build sizes for an acceleration structure
///# C Specifications
///The [`AccelerationStructureBuildSizesInfoKHR`] structure describes the
///required build sizes for an acceleration structure and scratch buffers and
///is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef struct VkAccelerationStructureBuildSizesInfoKHR {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkDeviceSize       accelerationStructureSize;
///    VkDeviceSize       updateScratchSize;
///    VkDeviceSize       buildScratchSize;
///} VkAccelerationStructureBuildSizesInfoKHR;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`acceleration_structure_size`] is the size in bytes required in a
///   [`AccelerationStructureKHR`] for a build or update operation.
/// - [`update_scratch_size`] is the size in bytes required in a scratch buffer for an update
///   operation.
/// - [`build_scratch_size`] is the size in bytes required in a scratch buffer for a build
///   operation.
///# Description
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_ACCELERATION_STRUCTURE_BUILD_SIZES_INFO_KHR`
/// - [`p_next`] **must**  be `NULL`
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`DeviceSize`]
/// - [`StructureType`]
/// - [`GetAccelerationStructureBuildSizesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureBuildSizesInfoKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct AccelerationStructureBuildSizesInfoKHR<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`acceleration_structure_size`] is the size in bytes required in a
    ///[`AccelerationStructureKHR`] for a build or update operation.
    pub acceleration_structure_size: DeviceSize,
    ///[`update_scratch_size`] is the size in bytes required in a scratch
    ///buffer for an update operation.
    pub update_scratch_size: DeviceSize,
    ///[`build_scratch_size`] is the size in bytes required in a scratch buffer
    ///for a build operation.
    pub build_scratch_size: DeviceSize,
}
impl<'lt> Default for AccelerationStructureBuildSizesInfoKHR<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
            p_next: std::ptr::null(),
            acceleration_structure_size: Default::default(),
            update_scratch_size: Default::default(),
            build_scratch_size: Default::default(),
        }
    }
}
impl<'lt> AccelerationStructureBuildSizesInfoKHR<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::acceleration_structure_size`]
    pub fn acceleration_structure_size(&self) -> DeviceSize {
        self.acceleration_structure_size
    }
    ///Gets the value of [`Self::update_scratch_size`]
    pub fn update_scratch_size(&self) -> DeviceSize {
        self.update_scratch_size
    }
    ///Gets the value of [`Self::build_scratch_size`]
    pub fn build_scratch_size(&self) -> DeviceSize {
        self.build_scratch_size
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::acceleration_structure_size`]
    pub fn acceleration_structure_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.acceleration_structure_size
    }
    ///Gets a mutable reference to the value of [`Self::update_scratch_size`]
    pub fn update_scratch_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.update_scratch_size
    }
    ///Gets a mutable reference to the value of [`Self::build_scratch_size`]
    pub fn build_scratch_size_mut(&mut self) -> &mut DeviceSize {
        &mut self.build_scratch_size
    }
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::acceleration_structure_size`]
    pub fn set_acceleration_structure_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.acceleration_structure_size = value;
        self
    }
    ///Sets the raw value of [`Self::update_scratch_size`]
    pub fn set_update_scratch_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.update_scratch_size = value;
        self
    }
    ///Sets the raw value of [`Self::build_scratch_size`]
    pub fn set_build_scratch_size(&mut self, value: crate::vulkan1_0::DeviceSize) -> &mut Self {
        self.build_scratch_size = value;
        self
    }
}
///[VkDeviceOrHostAddressKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressKHR.html) - Union specifying a device or host address
///# C Specifications
///The [`DeviceOrHostAddressKHR`] union is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef union VkDeviceOrHostAddressKHR {
///    VkDeviceAddress    deviceAddress;
///    void*              hostAddress;
///} VkDeviceOrHostAddressKHR;
///```
///# Members
/// - [`device_address`] is a buffer device address as returned by the [`GetBufferDeviceAddressKHR`]
///   command.
/// - [`host_address`] is a host memory address.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`CopyAccelerationStructureToMemoryInfoKHR`]
/// - [`DeviceAddress`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceOrHostAddressKHR")]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union DeviceOrHostAddressKHR {
    ///[`device_address`] is a buffer device address as returned by the
    ///[`GetBufferDeviceAddressKHR`] command.
    pub device_address: DeviceAddress,
    ///[`host_address`] is a host memory address.
    pub host_address: *mut c_void,
}
impl Default for DeviceOrHostAddressKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
///[VkDeviceOrHostAddressConstKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDeviceOrHostAddressConstKHR.html) - Union specifying a const device or host address
///# C Specifications
///The [`DeviceOrHostAddressConstKHR`] union is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef union VkDeviceOrHostAddressConstKHR {
///    VkDeviceAddress    deviceAddress;
///    const void*        hostAddress;
///} VkDeviceOrHostAddressConstKHR;
///```
///# Members
/// - [`device_address`] is a buffer device address as returned by the [`GetBufferDeviceAddressKHR`]
///   command.
/// - [`host_address`] is a const host memory address.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryAabbsDataKHR`]
/// - [`AccelerationStructureGeometryInstancesDataKHR`]
/// - [`AccelerationStructureGeometryMotionTrianglesDataNV`]
/// - [`AccelerationStructureGeometryTrianglesDataKHR`]
/// - [`CopyMemoryToAccelerationStructureInfoKHR`]
/// - [`DeviceAddress`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkDeviceOrHostAddressConstKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union DeviceOrHostAddressConstKHR {
    ///[`device_address`] is a buffer device address as returned by the
    ///[`GetBufferDeviceAddressKHR`] command.
    pub device_address: DeviceAddress,
    ///[`host_address`] is a const host memory address.
    pub host_address: *const c_void,
}
impl Default for DeviceOrHostAddressConstKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
///[VkAccelerationStructureGeometryDataKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureGeometryDataKHR.html) - Union specifying acceleration structure geometry data
///# C Specifications
///The [`AccelerationStructureGeometryDataKHR`] union is defined as:
///```c
///// Provided by VK_KHR_acceleration_structure
///typedef union VkAccelerationStructureGeometryDataKHR {
///    VkAccelerationStructureGeometryTrianglesDataKHR    triangles;
///    VkAccelerationStructureGeometryAabbsDataKHR        aabbs;
///    VkAccelerationStructureGeometryInstancesDataKHR    instances;
///} VkAccelerationStructureGeometryDataKHR;
///```
///# Members
/// - [`triangles`] is a [`AccelerationStructureGeometryTrianglesDataKHR`] structure.
/// - [`aabbs`] is a [`AccelerationStructureGeometryAabbsDataKHR`] struture.
/// - [`instances`] is a [`AccelerationStructureGeometryInstancesDataKHR`] structure.
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureGeometryAabbsDataKHR`]
/// - [`AccelerationStructureGeometryInstancesDataKHR`]
/// - [`AccelerationStructureGeometryKHR`]
/// - [`AccelerationStructureGeometryTrianglesDataKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureGeometryDataKHR")]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub union AccelerationStructureGeometryDataKHR<'lt> {
    ///[`triangles`] is a
    ///[`AccelerationStructureGeometryTrianglesDataKHR`] structure.
    pub triangles: AccelerationStructureGeometryTrianglesDataKHR<'lt>,
    ///[`aabbs`] is a [`AccelerationStructureGeometryAabbsDataKHR`]
    ///struture.
    pub aabbs: AccelerationStructureGeometryAabbsDataKHR<'lt>,
    ///[`instances`] is a
    ///[`AccelerationStructureGeometryInstancesDataKHR`] structure.
    pub instances: AccelerationStructureGeometryInstancesDataKHR<'lt>,
}
impl Default for AccelerationStructureGeometryDataKHR {
    fn default() -> Self {
        unsafe { std::mem::zeroed() }
    }
}
///[VkAccelerationStructureKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkAccelerationStructureKHR.html) - Opaque handle to an acceleration structure object
///# C Specifications
///Acceleration structures are opaque data structures that are built by the
///implementation to more efficiently perform spatial queries on the provided
///geometric data.
///For this extension, an acceleration structure is either a top-level
///acceleration structure containing a set of bottom-level acceleration
///structures or a bottom-level acceleration structure containing either a set
///of axis-aligned bounding boxes for custom geometry or a set of triangles.Each instance in the
/// top-level acceleration structure contains a reference
///to a bottom-level acceleration structure as well as an instance transform
///plus information required to index into the shader bindings.
///The top-level acceleration structure is what is bound to the acceleration
///descriptor, for example to trace inside the shader in the ray tracing
///pipeline.Acceleration structures are represented by [`AccelerationStructureKHR`]
///handles:
///```c
///// Provided by VK_KHR_acceleration_structure
///VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkAccelerationStructureKHR)
///```
///# Related
/// - [`VK_KHR_acceleration_structure`]
/// - [`AccelerationStructureBuildGeometryInfoKHR`]
/// - [`AccelerationStructureDeviceAddressInfoKHR`]
/// - [`CopyAccelerationStructureInfoKHR`]
/// - [`CopyAccelerationStructureToMemoryInfoKHR`]
/// - [`CopyMemoryToAccelerationStructureInfoKHR`]
/// - [`WriteDescriptorSetAccelerationStructureKHR`]
/// - [`CmdWriteAccelerationStructuresPropertiesKHR`]
/// - [`CreateAccelerationStructureKHR`]
/// - [`DestroyAccelerationStructureKHR`]
/// - [`WriteAccelerationStructuresPropertiesKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkAccelerationStructureKHR")]
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(transparent)]
pub struct AccelerationStructureKHR(pub u64);
impl AccelerationStructureKHR {
    ///Creates a new null handle
    #[inline]
    pub const fn null() -> Self {
        Self(0)
    }
    ///Checks if this is a null handle
    #[inline]
    pub fn is_null(&self) -> bool {
        self == &Self::null()
    }
    ///Gets the raw value
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
unsafe impl Send for AccelerationStructureKHR {}
impl Default for AccelerationStructureKHR {
    fn default() -> Self {
        Self::null()
    }
}
