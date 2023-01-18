[VkQueryPoolPerformanceCreateInfoKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceCreateInfoKHR.html) - Structure specifying parameters of a newly created performance query pool

# C Specifications
The [`QueryPoolPerformanceCreateInfoKHR`] structure is defined as:
```c
// Provided by VK_KHR_performance_query
typedef struct VkQueryPoolPerformanceCreateInfoKHR {
    VkStructureType    sType;
    const void*        pNext;
    uint32_t           queueFamilyIndex;
    uint32_t           counterIndexCount;
    const uint32_t*    pCounterIndices;
} VkQueryPoolPerformanceCreateInfoKHR;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`queue_family_index`] is the queue family index to create this performance query pool for.
- [`counter_index_count`] is the length of the [`counter_indices`] array.
- [`counter_indices`] is a pointer to an array of indices into the [`enumerate_physical_device_queue_family_performance_query_counters_khr`]`::pCounters` to enable in this performance query pool.

# Description
## Valid Usage
-  [`queue_family_index`] **must**  be a valid queue family index of the device
-    The [`performanceCounterQueryPools`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-performanceCounterQueryPools) feature  **must**  be enabled
-    Each element of [`counter_indices`] **must**  be in the range of counters reported by [`enumerate_physical_device_queue_family_performance_query_counters_khr`] for the queue family specified in [`queue_family_index`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
-  [`counter_indices`] **must**  be a valid pointer to an array of [`counter_index_count`]`uint32_t` values
-  [`counter_index_count`] **must**  be greater than `0`

# Related
- [`VK_KHR_performance_query`]
- [`StructureType`]
- [`get_physical_device_queue_family_performance_query_passes_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        