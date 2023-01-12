[vkCmdWriteAccelerationStructuresPropertiesNV](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdWriteAccelerationStructuresPropertiesNV.html) - Write acceleration structure result parameters to query results.

# C Specifications
To query acceleration structure size parameters call:
```c
// Provided by VK_NV_ray_tracing
void vkCmdWriteAccelerationStructuresPropertiesNV(
    VkCommandBuffer                             commandBuffer,
    uint32_t                                    accelerationStructureCount,
    const VkAccelerationStructureNV*            pAccelerationStructures,
    VkQueryType                                 queryType,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command will be recorded.
- [`acceleration_structure_count`] is the count of acceleration structures for which to query the property.
- [`p_acceleration_structures`] is a pointer to an array of existing previously built acceleration structures.
- [`query_type`] is a [`QueryType`] value specifying the type of queries managed by the pool.
- [`query_pool`] is the query pool that will manage the results of the query.
- [`first_query`] is the first query index within the query pool that will contain the [`acceleration_structure_count`] number of results.

# Description
Accesses to any of the acceleration structures listed in
[`p_acceleration_structures`] **must**  be [synchronized](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-dependencies) with the
`VK_PIPELINE_STAGE_ACCELERATION_STRUCTURE_BUILD_BIT_KHR`[pipeline stage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-pipeline-stages) and an
[access type](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#synchronization-access-types) of
`VK_ACCESS_ACCELERATION_STRUCTURE_READ_BIT_KHR`.
## Valid Usage
-  [`query_pool`] **must**  have been created with a [`query_type`] matching [`query_type`]
-    The queries identified by [`query_pool`] and [`first_query`] **must**  be *unavailable*
-  `accelerationStructure` **must**  be bound completely and contiguously to a single [`DeviceMemory`] object via [`bind_acceleration_structure_memory_nv`]
-    All acceleration structures in [`p_acceleration_structures`] **must**  have been built prior to the execution of this command
-    All acceleration structures in [`p_acceleration_structures`] **must**  have been built with `VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR` if [`query_type`] is `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
-  [`query_type`] **must**  be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_acceleration_structures`] **must**  be a valid pointer to an array of [`acceleration_structure_count`] valid [`AccelerationStructureNV`] handles
-  [`query_type`] **must**  be a valid [`QueryType`] value
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-    This command  **must**  only be called outside of a render pass instance
-  [`acceleration_structure_count`] **must**  be greater than `0`
-    Each of [`command_buffer`], [`query_pool`], and the elements of [`p_acceleration_structures`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`nv_ray_tracing`]
- [`AccelerationStructureNV`]
- [`CommandBuffer`]
- [`QueryPool`]
- [`QueryType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        