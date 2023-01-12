[vkCmdCopyQueryPoolResults](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdCopyQueryPoolResults.html) - Copy the results of queries in a query pool to a buffer object

# C Specifications
To copy query statuses and numerical results directly to buffer memory,
call:
```c
// Provided by VK_VERSION_1_0
void vkCmdCopyQueryPoolResults(
    VkCommandBuffer                             commandBuffer,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery,
    uint32_t                                    queryCount,
    VkBuffer                                    dstBuffer,
    VkDeviceSize                                dstOffset,
    VkDeviceSize                                stride,
    VkQueryResultFlags                          flags);
```

# Parameters
- [`command_buffer`] is the command buffer into which this command will be recorded.
- [`query_pool`] is the query pool managing the queries containing the desired results.
- [`first_query`] is the initial query index.
- [`query_count`] is the number of queries. [`first_query`] and [`query_count`] together define a range of queries.
- [`dst_buffer`] is a [`Buffer`] object that will receive the results of the copy command.
- [`dst_offset`] is an offset into [`dst_buffer`].
- [`stride`] is the stride in bytes between results for individual queries within [`dst_buffer`]. The required size of the backing memory for [`dst_buffer`] is determined as described above for [`get_query_pool_results`].
- [`flags`] is a bitmask of [`QueryResultFlagBits`] specifying how and when results are returned.

# Description
[`cmd_copy_query_pool_results`] is guaranteed to see the effect of previous
uses of [`cmd_reset_query_pool`] in the same queue, without any additional
synchronization.
Thus, the results will always reflect the most recent use of the query.[`flags`] has the same possible values described above for the [`flags`]
parameter of [`get_query_pool_results`], but the different style of
execution causes some subtle behavioral differences.
Because [`cmd_copy_query_pool_results`] executes in order with respect to
other query commands, there is less ambiguity about which use of a query is
being requested.Results for all requested occlusion queries, pipeline statistics queries,
transform feedback queries,
and timestamp queries are written as 64-bit unsigned integer values if
`VK_QUERY_RESULT_64_BIT` is set or 32-bit unsigned integer values
otherwise.
Performance queries store results in a tightly packed array whose type is
determined by the `unit` member of the corresponding
[`PerformanceCounterKHR`].If neither of `VK_QUERY_RESULT_WAIT_BIT` and
`VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` are set, results are only
written out for queries in the available state.If `VK_QUERY_RESULT_WAIT_BIT` is set, the implementation will wait for
each query’s status to be in the available state before retrieving the
numerical results for that query.
This is guaranteed to reflect the most recent use of the query on the same
queue, assuming that the query is not being simultaneously used by other
queues.
If the query does not become available in a finite amount of time (e.g. due
to not issuing a query since the last reset), a `VK_ERROR_DEVICE_LOST`
error  **may**  occur.Similarly, if `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set and
`VK_QUERY_RESULT_WAIT_BIT` is not set, the availability is guaranteed to
reflect the most recent use of the query on the same queue, assuming that
the query is not being simultaneously used by other queues.
As with [`get_query_pool_results`], implementations  **must**  guarantee that if
they return a non-zero availability value, then the numerical results are
valid.If `VK_QUERY_RESULT_PARTIAL_BIT` is set, `VK_QUERY_RESULT_WAIT_BIT`
is not set, and the query’s status is unavailable, an intermediate result
value between zero and the final result value is written for that query.`VK_QUERY_RESULT_PARTIAL_BIT` **must**  not be used if the pool’s
`queryType` is `VK_QUERY_TYPE_TIMESTAMP`.[`cmd_copy_query_pool_results`] is considered to be a transfer operation,
and its writes to buffer memory  **must**  be synchronized using
`VK_PIPELINE_STAGE_TRANSFER_BIT` and `VK_ACCESS_TRANSFER_WRITE_BIT`
before using the results.
## Valid Usage
-  [`dst_offset`] **must**  be less than the size of [`dst_buffer`]
-  [`first_query`] **must**  be less than the number of queries in [`query_pool`]
-    The sum of [`first_query`] and [`query_count`] **must**  be less than or equal to the number of queries in [`query_pool`]
-    If `VK_QUERY_RESULT_64_BIT` is not set in [`flags`] then [`dst_offset`] and [`stride`] **must**  be multiples of `4`
-    If `VK_QUERY_RESULT_64_BIT` is set in [`flags`] then [`dst_offset`] and [`stride`] **must**  be multiples of `8`
-  [`dst_buffer`] **must**  have enough storage, from [`dst_offset`], to contain the result of each query, as described [here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-memorylayout)
-  [`dst_buffer`] **must**  have been created with `VK_BUFFER_USAGE_TRANSFER_DST_BIT` usage flag
-    If [`dst_buffer`] is non-sparse then it  **must**  be bound completely and contiguously to a single [`DeviceMemory`] object
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TIMESTAMP`, [`flags`] **must**  not contain `VK_QUERY_RESULT_PARTIAL_BIT`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, [`PhysicalDevicePerformanceQueryPropertiesKHR::allow_command_buffer_query_copies`] **must**  be `VK_TRUE`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, [`flags`] **must**  not contain `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT`, `VK_QUERY_RESULT_PARTIAL_BIT` or `VK_QUERY_RESULT_64_BIT`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [`query_pool`] **must**  have been submitted once for each pass as retrieved via a call to [`get_physical_device_queue_family_performance_query_passes_khr`]
-  [`cmd_copy_query_pool_results`] **must**  not be called if the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL`

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`dst_buffer`] **must**  be a valid [`Buffer`] handle
-  [`flags`] **must**  be a valid combination of [`QueryResultFlagBits`] values
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations
-    This command  **must**  only be called outside of a render pass instance
-    Each of [`command_buffer`], [`dst_buffer`], and [`query_pool`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`crate::vulkan1_0`]
- [`Buffer`]
- [`CommandBuffer`]
- [`DeviceSize`]
- [`QueryPool`]
- [VkQueryResultFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        