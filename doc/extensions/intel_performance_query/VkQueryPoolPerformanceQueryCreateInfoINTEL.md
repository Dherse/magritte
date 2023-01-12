[VkQueryPoolPerformanceQueryCreateInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html) - Structure specifying parameters to create a pool of performance queries

# C Specifications
The [`QueryPoolPerformanceQueryCreateInfoINTEL`] structure is defined
as:
```c
// Provided by VK_INTEL_performance_query
typedef struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
    VkStructureType                 sType;
    const void*                     pNext;
    VkQueryPoolSamplingModeINTEL    performanceCountersSampling;
} VkQueryPoolPerformanceQueryCreateInfoINTEL;
```

```c
// Provided by VK_INTEL_performance_query
typedef VkQueryPoolPerformanceQueryCreateInfoINTEL VkQueryPoolCreateInfoINTEL;
```

# Members
To create a pool for Intel performance queries, set
[`QueryPoolCreateInfo::query_type`] to
`VK_QUERY_TYPE_PERFORMANCE_QUERY_INTEL` and add a
[`QueryPoolPerformanceQueryCreateInfoINTEL`] structure to the
[`p_next`] chain of the [`QueryPoolCreateInfo`] structure.

# Description
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`performance_counters_sampling`] describe how performance queries should be captured.

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_QUERY_CREATE_INFO_INTEL`
-  [`performance_counters_sampling`] **must**  be a valid [`QueryPoolSamplingModeINTEL`] value

# Related
- [`intel_performance_query`]
- [`QueryPoolSamplingModeINTEL`]
- [`StructureType`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        