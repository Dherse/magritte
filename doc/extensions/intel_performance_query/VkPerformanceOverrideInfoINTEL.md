[VkPerformanceOverrideInfoINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html) - Performance override information

# C Specifications
The [`PerformanceOverrideInfoINTEL`] structure is defined as:
```c
// Provided by VK_INTEL_performance_query
typedef struct VkPerformanceOverrideInfoINTEL {
    VkStructureType                   sType;
    const void*                       pNext;
    VkPerformanceOverrideTypeINTEL    type;
    VkBool32                          enable;
    uint64_t                          parameter;
} VkPerformanceOverrideInfoINTEL;
```

# Members
- [`type_`] is the particular [`PerformanceOverrideTypeINTEL`] to set.
- [`enable`] defines whether the override is enabled.
- [`parameter`] is a potential required parameter for the override.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_PERFORMANCE_OVERRIDE_INFO_INTEL`
-  [`p_next`] **must**  be `NULL`
-  [`type_`] **must**  be a valid [`PerformanceOverrideTypeINTEL`] value

# Related
- [`intel_performance_query`]
- [`Bool32`]
- [`PerformanceOverrideTypeINTEL`]
- [`StructureType`]
- [`cmd_set_performance_override_intel`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        