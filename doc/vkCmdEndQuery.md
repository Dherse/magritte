[vkCmdEndQuery](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndQuery.html) - Ends a query

# C Specifications
To end a query after the set of desired drawing or dispatching commands is
executed, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdEndQuery(
    VkCommandBuffer                             commandBuffer,
    VkQueryPool                                 queryPool,
    uint32_t                                    query);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`query_pool`] is the query pool that is managing the results of the query.
- [`query`] is the query index within the query pool where the result is stored.

# Description
Calling [`cmd_end_query`] is equivalent to calling
[`cmd_end_query_indexed_ext`] with the `index` parameter set to zero.As queries operate asynchronously, ending a query does not immediately set
the query’s status to available.
A query is considered *finished* when the final results of the query are
ready to be retrieved by [`get_query_pool_results`] and
[`cmd_copy_query_pool_results`], and this is when the query’s status is set
to available.Once a query is ended the query  **must**  finish in finite time, unless the
state of the query is changed using other commands, e.g. by issuing a reset
of the query.
## Valid Usage
-    All queries used by the command  **must**  be [active](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-active)
-  [`query`] **must**  be less than the number of queries in [`query_pool`]
-  [`command_buffer`] **must**  not be a protected command buffer
-    If [`cmd_end_query`] is called within a render pass instance, the sum of [`query`] and the number of bits set in the current subpass’s view mask  **must**  be less than or equal to the number of queries in [`query_pool`]
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and one or more of the counters used to create [`query_pool`] was `VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR`, the [`cmd_end_query`] **must**  be the last recorded command in [`command_buffer`]
-    If [`query_pool`] was created with a `queryType` of `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` and one or more of the counters used to create [`query_pool`] was `VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR`, the [`cmd_end_query`] **must**  not be recorded within a render pass instance

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, compute, decode, or encode operations
-    Both of [`command_buffer`], and [`query_pool`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`CommandBuffer`]
- [`QueryPool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        