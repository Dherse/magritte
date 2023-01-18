[vkCmdBeginQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdBeginQueryIndexedEXT.html) - Begin an indexed query

# C Specifications
To begin an indexed query, call:
```c
// Provided by VK_EXT_transform_feedback
void vkCmdBeginQueryIndexedEXT(
    VkCommandBuffer                             commandBuffer,
    VkQueryPool                                 queryPool,
    uint32_t                                    query,
    VkQueryControlFlags                         flags,
    uint32_t                                    index);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`query_pool`] is the query pool that will manage the results of the query.
- [`query`] is the query index within the query pool that will contain the results.
- [`flags`] is a bitmask of [`QueryControlFlagBits`] specifying constraints on the types of queries that  **can**  be performed.
- [`index`] is the query type specific index. When the query type is `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the index represents the vertex stream.

# Description
The [`cmd_begin_query_indexed_ext`] command operates the same as the
[`cmd_begin_query`] command, except that it also accepts a query type
specific [`index`] parameter.
## Valid Usage
-    All queries used by the command  **must**  be unavailable
-    The `queryType` used to create [`query_pool`] **must**  not be `VK_QUERY_TYPE_TIMESTAMP`
-    The `queryType` used to create [`query_pool`] **must**  not be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_KHR` or `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_SERIALIZATION_SIZE_KHR`
-    The `queryType` used to create [`query_pool`] **must**  not be `VK_QUERY_TYPE_ACCELERATION_STRUCTURE_COMPACTED_SIZE_NV`
-    If the [precise occlusion queries]() feature is not enabled, or the `queryType` used to create [`query_pool`] was not `VK_QUERY_TYPE_OCCLUSION`, [`flags`] **must**  not contain `VK_QUERY_CONTROL_PRECISE_BIT`
-  [`query`] **must**  be less than the number of queries in [`query_pool`]
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_OCCLUSION`, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS` and any of the `pipelineStatistics` indicate graphics operations, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PIPELINE_STATISTICS` and any of the `pipelineStatistics` indicate compute operations, the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support compute operations
-  [`command_buffer`] **must**  not be a protected command buffer
-    If called within a render pass instance, the sum of [`query`] and the number of bits set in the current subpass’s view mask  **must**  be less than or equal to the number of queries in [`query_pool`]
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_VIDEO_ENCODE_BITSTREAM_BUFFER_RANGE_KHR` the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support [video encode operations]()
-    If the [`query_pool`] was created with the same `queryType` as that of another [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active) query within [`command_buffer`], then [`index`] **must**  not match the index used for the active query
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics operations
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
-    If the `queryType` used to create [`query_pool`] was not `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` then [`PhysicalDeviceTransformFeedbackPropertiesEXT::transform_feedback_queries`] **must**  be supported

-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [profiling lock]() **must**  have been held before [`begin_command_buffer`] was called on [`command_buffer`]
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and one of the counters used to create [`query_pool`] was `VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR`, the query begin  **must**  be the first recorded command in [`command_buffer`]
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and one of the counters used to create [`query_pool`] was `VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR`, the begin command  **must**  not be recorded within a render pass instance
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and another query pool with a `queryType``VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` has been used within [`command_buffer`], its parent primary command buffer or secondary command buffer recorded within the same parent primary command buffer as [`command_buffer`], the [`performanceCounterMultipleQueryPools`]() feature  **must**  be enabled
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, this command  **must**  not be recorded in a command buffer that, either directly or through secondary command buffers, also contains a [`cmd_reset_query_pool`] command affecting the same query

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`flags`] **must**  be a valid combination of [`QueryControlFlagBits`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_transform_feedback`]
- [`CommandBuffer`]
- [`QueryControlFlags`]
- [`QueryPool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        