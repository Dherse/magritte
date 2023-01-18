[vkResetQueryPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkResetQueryPool.html) - Reset queries in a query pool

# C Specifications
To reset a range of queries in a query pool on the host, call:
```c
// Provided by VK_VERSION_1_2
void vkResetQueryPool(
    VkDevice                                    device,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery,
    uint32_t                                    queryCount);
```
or the equivalent command
```c
// Provided by VK_EXT_host_query_reset
void vkResetQueryPoolEXT(
    VkDevice                                    device,
    VkQueryPool                                 queryPool,
    uint32_t                                    firstQuery,
    uint32_t                                    queryCount);
```

# Parameters
- [`device`] is the logical device that owns the query pool.
- [`query_pool`] is the handle of the query pool managing the queries being reset.
- [`first_query`] is the initial query index to reset.
- [`query_count`] is the number of queries to reset.

# Description
This command sets the status of query indices [[`first_query`],
[`first_query`] +  [`query_count`] - 1] to unavailable.If [`query_pool`] is `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR` this command
sets the status of query indices [[`first_query`], [`first_query`]
+  [`query_count`] - 1] to unavailable for each pass.
## Valid Usage
-    The [hostQueryReset](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-hostQueryReset) feature  **must**  be enabled
-  [`first_query`] **must**  be less than the number of queries in [`query_pool`]
-    The sum of [`first_query`] and [`query_count`] **must**  be less than or equal to the number of queries in [`query_pool`]
-    Submitted commands that refer to the range specified by [`first_query`] and [`query_count`] in [`query_pool`] **must**  have completed execution
-    The range of queries specified by [`first_query`] and [`query_count`] in [`query_pool`] **must**  not be in use by calls to [`get_query_pool_results`] or [`reset_query_pool`] in other threads

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`query_pool`] **must**  be a valid [`QueryPool`] handle
-  [`query_pool`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_EXT_host_query_reset`]
- [`crate::vulkan1_2`]
- [`Device`]
- [`QueryPool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        