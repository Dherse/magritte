[VkPerformanceParameterTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html) - Parameters that can be queried

# C Specifications
Possible values of [`get_performance_parameter_intel`]`::parameter`,
specifying a performance query feature, are:
```c
// Provided by VK_INTEL_performance_query
typedef enum VkPerformanceParameterTypeINTEL {
    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0,
    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1,
} VkPerformanceParameterTypeINTEL;
```

# Description
- [`VK_PERFORMANCE_PARAMETER_TYPE_INTEL`] has a boolean result which tells whether hardware counters can be captured.
- [`VK_PERFORMANCE_PARAMETER_TYPE_INTEL`] has a 32 bits integer result which tells how many bits can be written into the [`PerformanceValueINTEL`] value.

# Related
- [`intel_performance_query`]
- [`get_performance_parameter_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        