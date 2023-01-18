[vkCmdEndQueryIndexedEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQueryIndexedEXT.html) - Ends a query

# C Specifications
To end an indexed query after the set of desired drawing or dispatching
commands is recorded, call:
```c
// Provided by VK_EXT_transform_feedback
void vkCmdEndQueryIndexedEXT(
    VkCommandBuffer                             commandBuffer,
    VkQueryPool                                 queryPool,
    uint32_t                                    query,
    uint32_t                                    index);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`query_pool`] is the query pool that is managing the results of the query.
- [`query`] is the query index within the query pool where the result is stored.
- [`index`] is the query type specific index.

# Description
The [`cmd_end_query_indexed_ext`] command operates the same as the
[`cmd_end_query`] command, except that it also accepts a query type
specific [`index`] parameter.
## Valid Usage
-    All queries used by the command  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
-  [`query`] **must**  be less than the number of queries in [`query_pool`]
-  [`command_buffer`] **must**  not be a protected command buffer
-    If [`cmd_end_query_indexed_ext`] is called within a render pass instance, the sum of [`query`] and the number of bits set in the current subpass’s view mask  **must**  be less than or equal to the number of queries in [`query_pool`]
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] parameter  **must**  be less than [`PhysicalDeviceTransformFeedbackPropertiesEXT::max_transform_feedback_streams`]
-    If the `queryType` used to create [`query_pool`] was not `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT` the [`index`] **must**  be zero
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TRANSFORM_FEEDBACK_STREAM_EXT`[`index`] **must**  equal the [`index`] used to begin the query

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
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
- [`QueryPool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        