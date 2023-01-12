[VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) - Performance override type

# C Specifications
Possible values of [`PerformanceOverrideInfoINTEL::type_`],
specifying performance override types, are:
```c
// Provided by VK_INTEL_performance_query
typedef enum VkPerformanceOverrideTypeINTEL {
    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0,
    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1,
} VkPerformanceOverrideTypeINTEL;
```

# Description
- [`VK_PERFORMANCE_OVERRIDE_TYPE_INTEL`] turns all rendering operations into noop.
- [`VK_PERFORMANCE_OVERRIDE_TYPE_INTEL`] stalls the stream of commands until all previously emitted commands have completed and all caches been flushed and invalidated.

# Related
- [`intel_performance_query`]
- [`PerformanceOverrideInfoINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        