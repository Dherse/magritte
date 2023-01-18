[VkPerformanceCounterStorageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html) - Supported counter storage types

# C Specifications
Performance counters have an associated storage.
This storage describes the payload of a counter result.The performance counter storage types which  **may**  be returned in
[`PerformanceCounterKHR::storage`] are:
```c
// Provided by VK_KHR_performance_query
typedef enum VkPerformanceCounterStorageKHR {
    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0,
    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2,
    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4,
    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5,
} VkPerformanceCounterStorageKHR;
```

# Description
- [`INT32`] - the performance counter storage is a 32-bit signed integer.
- [`INT64`] - the performance counter storage is a 64-bit signed integer.
- [`UINT32`] - the performance counter storage is a 32-bit unsigned integer.
- [`UINT64`] - the performance counter storage is a 64-bit unsigned integer.
- [`FLOAT32`] - the performance counter storage is a 32-bit floating-point.
- [`FLOAT64`] - the performance counter storage is a 64-bit floating-point.

# Related
- [`VK_KHR_performance_query`]
- [`PerformanceCounterKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        