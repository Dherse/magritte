[VkPerformanceValueINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html) - Container for value and types of parameters that can be queried

# C Specifications
The [`PerformanceValueINTEL`] structure is defined as:
```c
// Provided by VK_INTEL_performance_query
typedef struct VkPerformanceValueINTEL {
    VkPerformanceValueTypeINTEL    type;
    VkPerformanceValueDataINTEL    data;
} VkPerformanceValueINTEL;
```

# Members
- [`type_`] is a [`PerformanceValueTypeINTEL`] value specifying the type of the returned data.
- [`data`] is a [`PerformanceValueDataINTEL`] union specifying the value of the returned data.

# Description
## Valid Usage (Implicit)
-  [`type_`] **must**  be a valid [`PerformanceValueTypeINTEL`] value
-    If [`type_`] is `VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL`, the `valueString` member of [`data`] **must**  be a null-terminated UTF-8 string

# Related
- [`VK_INTEL_performance_query`]
- [`PerformanceValueDataINTEL`]
- [`PerformanceValueTypeINTEL`]
- [`get_performance_parameter_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        