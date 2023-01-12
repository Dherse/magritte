[vkGetQueryPoolResults](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetQueryPoolResults.html) - Copy results of queries in a query pool to a host memory region

# C Specifications
To retrieve status and results for a set of queries, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkGetQueryPoolResults(
    VkDevice                                    device,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery,
    uint32_t                                    queryCount,
    size_t                                      dataSize,
    void*                                       pData,
    VkDeviceSize                                stride,
    VkQueryResultFlags                          flags);
```

# Parameters
- [`device`] is the logical device that owns the query pool.
- [`query_pool`] is the query pool managing the queries containing the desired results.
- [`first_query`] is the initial query index.
- [`query_count`] is the number of queries to read.
- [`data_size`] is the size in bytes of the buffer pointed to by [`p_data`].
- [`p_data`] is a pointer to a user-allocated buffer where the results will be written
- [`stride`] is the stride in bytes between results for individual queries within [`p_data`].
- [`flags`] is a bitmask of [`QueryResultFlagBits`] specifying how and when results are returned.

# Description
The range of queries read is defined by [[`first_query`],
[`first_query`] +  [`query_count`] - 1].
For pipeline statistics queries, each query index in the pool contains one
integer value for each bit that is enabled in
[`QueryPoolCreateInfo::pipeline_statistics`] when the pool is
created.If no bits are set in [`flags`], and all requested queries are in the
available state, results are written as an array of 32-bit unsigned integer
values.
The behavior when not all queries are available, is described
[below](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-wait-bit-not-set).If `VK_QUERY_RESULT_64_BIT` is not set and the result overflows a 32-bit
value, the value  **may**  either wrap or saturate.
Similarly, if `VK_QUERY_RESULT_64_BIT` is set and the result overflows a
64-bit value, the value  **may**  either wrap or saturate.If `VK_QUERY_RESULT_WAIT_BIT` is set, Vulkan will wait for each query to
be in the available state before retrieving the numerical results for that
query.
In this case, [`get_query_pool_results`] is guaranteed to succeed and
return `VK_SUCCESS` if the queries become available in a finite time
(i.e. if they have been issued and not reset).
If queries will never finish (e.g. due to being reset but not issued), then
[`get_query_pool_results`] **may**  not return in finite time.If `VK_QUERY_RESULT_WAIT_BIT` and `VK_QUERY_RESULT_PARTIAL_BIT` are
both not set then no result values are written to [`p_data`] for queries
that are in the unavailable state at the time of the call, and
[`get_query_pool_results`] returns `VK_NOT_READY`.
However, availability state is still written to [`p_data`] for those
queries if `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set.
Similarly, the status is still written to [`p_data`] for those queries if
`VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` is set.If `VK_QUERY_RESULT_WAIT_BIT` is not set, [`get_query_pool_results`] **may**  return `VK_NOT_READY` if there are queries in the unavailable
state.If `VK_QUERY_RESULT_PARTIAL_BIT` is set, `VK_QUERY_RESULT_WAIT_BIT`
is not set, and the query’s status is unavailable, an intermediate result
value between zero and the final result value is written to [`p_data`] for
that query.If `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is set, the final integer
value written for each query is non-zero if the query’s status was available
or zero if the status was unavailable.
When `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT` is used, implementations
 **must**  guarantee that if they return a non-zero availability value then the
numerical results  **must**  be valid, assuming the results are not reset by a
subsequent command.If `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR` is set, the final integer value
written for each query indicates whether the result is available or not, and
whether an error occurred.
A value of zero indicates that the results are not yet available.
Positive values indicate that the operations within the query completed
successfully, and the query results are valid.
Negative values indicate that the operations within the query completed
unsuccessfully.Specific result codes are defined by the [`QueryResultStatusKHR`]
enumeration.
## Valid Usage
-  [`first_query`] **must**  be less than the number of queries in [`query_pool`]
-    If `VK_QUERY_RESULT_64_BIT` is not set in [`flags`] and the `queryType` used to create [`query_pool`] was not `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, then [`p_data`] and [`stride`] **must**  be multiples of `4`
-    If `VK_QUERY_RESULT_64_BIT` is set in [`flags`] then [`p_data`] and [`stride`] **must**  be multiples of `8`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, then [`p_data`] and [`stride`] **must**  be multiples of the size of [`PerformanceCounterResultKHR`]
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, then [`stride`] **must**  be large enough to contain [`QueryPoolPerformanceCreateInfoKHR::counter_index_count`] used to create [`query_pool`] times the size of [`PerformanceCounterResultKHR`]
-    The sum of [`first_query`] and [`query_count`] **must**  be less than or equal to the number of queries in [`query_pool`]
-  [`data_size`] **must**  be large enough to contain the result of each query, as described [here](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-operation-memorylayout)
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_TIMESTAMP`, [`flags`] **must**  not contain `VK_QUERY_RESULT_PARTIAL_BIT`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, [`flags`] **must**  not contain `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT`, `VK_QUERY_RESULT_PARTIAL_BIT` or `VK_QUERY_RESULT_64_BIT`
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [`query_pool`] **must**  have been recorded once for each pass as retrieved via a call to [`get_physical_device_queue_family_performance_query_passes_khr`]
-    If the `queryType` used to create [`query_pool`] was `VK_QUERY_TYPE_RESULT_STATUS_ONLY_KHR`, [`flags`] **must**  include `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR`
-    If [`flags`] includes `VK_QUERY_RESULT_WITH_STATUS_BIT_KHR`, it  **must**  not include `VK_QUERY_RESULT_WITH_AVAILABILITY_BIT`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`p_data`] **must**  be a valid pointer to an array of [`data_size`] bytes
-  [`flags`] **must**  be a valid combination of [`QueryResultFlagBits`] values
-  [`data_size`] **must**  be greater than `0`
-  [`query_pool`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_NOT_READY` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_DEVICE_LOST`

# Related
- [`crate::vulkan1_0`]
- [`Device`]
- [`DeviceSize`]
- [`QueryPool`]
- [VkQueryResultFlags]()

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        