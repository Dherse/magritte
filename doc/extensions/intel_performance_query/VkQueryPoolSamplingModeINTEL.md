[VkQueryPoolSamplingModeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) - Enum specifying how performance queries should be captured

# C Specifications
Possible values of
[`QueryPoolPerformanceQueryCreateInfoINTEL::performance_counters_sampling`]
are:
```c
// Provided by VK_INTEL_performance_query
typedef enum VkQueryPoolSamplingModeINTEL {
    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0,
} VkQueryPoolSamplingModeINTEL;
```

# Description
- [`VK_QUERY_POOL_SAMPLING_MODE_INTEL`] is the default mode in which the application calls [`cmd_begin_query`] and [`cmd_end_query`] to record performance data.

# Related
- [`intel_performance_query`]
- [`QueryPoolPerformanceQueryCreateInfoINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        