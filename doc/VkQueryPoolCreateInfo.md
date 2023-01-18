[VkQueryPoolCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolCreateInfo.html) - Structure specifying parameters of a newly created query pool

# C Specifications
The [`QueryPoolCreateInfo`] structure is defined as:
```c
// Provided by VK_VERSION_1_0
typedef struct VkQueryPoolCreateInfo {
    VkStructureType                  sType;
    const void*                      pNext;
    VkQueryPoolCreateFlags           flags;
    VkQueryType                      queryType;
    uint32_t                         queryCount;
    VkQueryPipelineStatisticFlags    pipelineStatistics;
} VkQueryPoolCreateInfo;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`query_type`] is a [`QueryType`] value specifying the type of queries managed by the pool.
- [`query_count`] is the number of queries managed by the pool.
- [`pipeline_statistics`] is a bitmask of [`QueryPipelineStatisticFlagBits`] specifying which counters will be returned in queries on the new pool, as described below in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-pipestats](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#queries-pipestats).

# Description
[`pipeline_statistics`] is ignored if [`query_type`] is not
`VK_QUERY_TYPE_PIPELINE_STATISTICS`.
## Valid Usage
-    If the [pipeline statistics queries](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-pipelineStatisticsQuery) feature is not enabled, [`query_type`] **must**  not be `VK_QUERY_TYPE_PIPELINE_STATISTICS`
-    If [`query_type`] is `VK_QUERY_TYPE_PIPELINE_STATISTICS`, [`pipeline_statistics`] **must**  be a valid combination of [`QueryPipelineStatisticFlagBits`] values
-    If [`query_type`] is `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`, the [`p_next`] chain  **must**  include a [`QueryPoolPerformanceCreateInfoKHR`] structure
-  [`query_count`] **must**  be greater than 0

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO`
-    Each [`p_next`] member of any structure (including this one) in the [`p_next`] chain  **must**  be either `NULL` or a pointer to a valid instance of [`QueryPoolPerformanceCreateInfoKHR`], [`QueryPoolPerformanceQueryCreateInfoINTEL`], [`VideoDecodeH264ProfileEXT`], [`VideoDecodeH265ProfileEXT`], [`VideoEncodeH264ProfileEXT`], [`VideoEncodeH265ProfileEXT`], or [`VideoProfileKHR`]
-    The [`s_type`] value of each struct in the [`p_next`] chain  **must**  be unique
-  [`flags`] **must**  be `0`
-  [`query_type`] **must**  be a valid [`QueryType`] value

# Related
- [`crate::vulkan1_0`]
- [`QueryPipelineStatisticFlags`]
- [`QueryPoolCreateFlags`]
- [`QueryType`]
- [`StructureType`]
- [`create_query_pool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        