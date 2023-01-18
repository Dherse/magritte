[vkInitializePerformanceApiINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html) - Initialize a device for performance queries

# C Specifications
Prior to creating a performance query pool, initialize the device for
performance queries with the call:
```c
// Provided by VK_INTEL_performance_query
VkResult vkInitializePerformanceApiINTEL(
    VkDevice                                    device,
    const VkInitializePerformanceApiInfoINTEL*  pInitializeInfo);
```

# Parameters
- [`device`] is the logical device used for the queries.
- [`p_initialize_info`] is a pointer to a [`InitializePerformanceApiInfoINTEL`] structure specifying initialization parameters.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_initialize_info`] **must**  be a valid pointer to a valid [`InitializePerformanceApiInfoINTEL`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_INTEL_performance_query`]
- [`Device`]
- [`InitializePerformanceApiInfoINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        