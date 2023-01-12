[VkPerformanceValueDataINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html) - Values returned for the parameters

# C Specifications
The [`PerformanceValueDataINTEL`] union is defined as:
```c
// Provided by VK_INTEL_performance_query
typedef union VkPerformanceValueDataINTEL {
    uint32_t       value32;
    uint64_t       value64;
    float          valueFloat;
    VkBool32       valueBool;
    const char*    valueString;
} VkPerformanceValueDataINTEL;
```

# Members
- `data.value32` represents 32-bit integer data.
- `data.value64` represents 64-bit integer data.
- `data.valueFloat` represents floating-point data.
- `data.valueBool` represents [`Bool32`] data.
- `data.valueString` represents a pointer to a null-terminated UTF-8 string.

# Description
The correct member of the union is determined by the associated
[`PerformanceValueTypeINTEL`] value.

# Related
- [`intel_performance_query`]
- [`Bool32`]
- [`PerformanceValueINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        