[VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html) - Type of the parameters that can be queried

# C Specifications
Possible values of [`PerformanceValueINTEL::type_`], specifying the
type of the data returned in [`PerformanceValueINTEL::data`], are:
- [`VK_PERFORMANCE_VALUE_TYPE_INTEL`] specifies that unsigned 32-bit integer data is returned in `data.value32`.
- [`VK_PERFORMANCE_VALUE_TYPE_INTEL`] specifies that unsigned 64-bit integer data is returned in `data.value64`.
- [`VK_PERFORMANCE_VALUE_TYPE_INTEL`] specifies that floating-point data is returned in `data.valueFloat`.
- [`VK_PERFORMANCE_VALUE_TYPE_INTEL`] specifies that [`Bool32`] data is returned in `data.valueBool`.
- [`VK_PERFORMANCE_VALUE_TYPE_INTEL`] specifies that a pointer to a null-terminated UTF-8 string is returned in `data.valueString`. The pointer is valid for the lifetime of the `device` parameter passed to [`get_performance_parameter_intel`].

```c
// Provided by VK_INTEL_performance_query
typedef enum VkPerformanceValueTypeINTEL {
    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0,
    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1,
    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2,
    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3,
    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4,
} VkPerformanceValueTypeINTEL;
```

# Related
- [`intel_performance_query`]
- [`PerformanceValueINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        