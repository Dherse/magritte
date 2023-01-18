[VkPerformanceCounterResultKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterResultKHR.html) - Union containing a performance counter result

# C Specifications
The [`PerformanceCounterResultKHR`] union is defined as:
- [`int32`] is a 32-bit signed integer value.
- [`int64`] is a 64-bit signed integer value.
- [`uint32`] is a 32-bit unsigned integer value.
- [`uint64`] is a 64-bit unsigned integer value.
- [`float32`] is a 32-bit floating-point value.
- [`float64`] is a 64-bit floating-point value.
Performance query results are returned in an array of
[`PerformanceCounterResultKHR`] unions containing the data associated
with each counter in the query, stored in the same order as the counters
supplied in `pCounterIndices` when creating the performance query.
The [`PerformanceCounterKHR::unit`] enumeration specifies how to
parse the counter data.
```c
// Provided by VK_KHR_performance_query
typedef union VkPerformanceCounterResultKHR {
    int32_t     int32;
    int64_t     int64;
    uint32_t    uint32;
    uint64_t    uint64;
    float       float32;
    double      float64;
} VkPerformanceCounterResultKHR;
```

# Related
- [`VK_KHR_performance_query`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        