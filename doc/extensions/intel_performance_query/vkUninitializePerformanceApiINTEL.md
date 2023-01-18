[vkUninitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html) - Uninitialize a device for performance queries

# C Specifications
Once performance query operations have completed, uninitalize the device for
performance queries with the call:
```c
// Provided by VK_INTEL_performance_query
void vkUninitializePerformanceApiINTEL(
    VkDevice                                    device);
```

# Parameters
- [`device`] is the logical device used for the queries.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle

# Related
- [`VK_INTEL_performance_query`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        