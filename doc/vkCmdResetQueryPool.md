[vkCmdResetQueryPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdResetQueryPool.html) - Reset queries in a query pool

# C Specifications
To reset a range of queries in a query pool on a queue, call:
```c
// Provided by VK_VERSION_1_0
void vkCmdResetQueryPool(
    VkCommandBuffer                             commandBuffer,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery,
    uint32_t                                    queryCount);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`query_pool`] is the handle of the query pool managing the queries being reset.
- [`first_query`] is the initial query index to reset.
- [`query_count`] is the number of queries to reset.

# Description
When executed on a queue, this command sets the status of query indices
[[`first_query`], [`first_query`] +  [`query_count`] - 1] to
unavailable.If the `queryType` used to create [`query_pool`] was
`VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, this command sets the status of
query indices [[`first_query`], [`first_query`] + 
[`query_count`] - 1] to unavailable for each pass of [`query_pool`], as
indicated by a call to
[`get_physical_device_queue_family_performance_query_passes_khr`].
## Valid Usage
-  [`first_query`] **must**  be less than the number of queries in [`query_pool`]
-    The sum of [`first_query`] and [`query_count`] **must**  be less than or equal to the number of queries in [`query_pool`]
-    All queries used by the command  **must**  not be active
-    If [`query_pool`] was created with `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, this command  **must**  not be recorded in a command buffer that, either directly or through secondary command buffers, also contains begin commands for a query from the set of queries [[`first_query`], [`first_query`] +  [`query_count`] - 1]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, compute, decode, or encode operations
-    This command  **must**  only be called outside of a render pass instance
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
        