[VkPerformanceConfigurationAcquireInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html) - Acquire a configuration to capture performance data

# C Specifications
The [`PerformanceConfigurationAcquireInfoINTEL`] structure is defined
as:
```c
// Provided by VK_INTEL_performance_query
typedef struct VkPerformanceConfigurationAcquireInfoINTEL {
    VkStructureType                        sType;
    const void*                            pNext;
    VkPerformanceConfigurationTypeINTEL    type;
} VkPerformanceConfigurationAcquireInfoINTEL;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`type_`] is one of the [`PerformanceConfigurationTypeINTEL`] type of performance configuration that will be acquired.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_CONFIGURATION_ACQUIRE_INFO_INTEL`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`PerformanceConfigurationTypeINTEL`] value

# Related
- [`intel_performance_query`]
- [`PerformanceConfigurationTypeINTEL`]
- [`StructureType`]
- [`acquire_performance_configuration_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        