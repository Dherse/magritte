[vkAcquirePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html) - Acquire the performance query capability

# C Specifications
To acquire a device performance configuration, call:
```c
// Provided by VK_INTEL_performance_query
VkResult vkAcquirePerformanceConfigurationINTEL(
    VkDevice                                    device,
    const VkPerformanceConfigurationAcquireInfoINTEL* pAcquireInfo,
    VkPerformanceConfigurationINTEL*            pConfiguration);
```

# Parameters
- [`device`] is the logical device that the performance query commands will be submitted to.
- [`p_acquire_info`] is a pointer to a [`PerformanceConfigurationAcquireInfoINTEL`] structure, specifying the performance configuration to acquire.
- [`p_configuration`] is a pointer to a [`PerformanceConfigurationINTEL`] handle in which the resulting configuration object is returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_acquire_info`] **must**  be a valid pointer to a valid [`PerformanceConfigurationAcquireInfoINTEL`] structure
-  [`p_configuration`] **must**  be a valid pointer to a [`PerformanceConfigurationINTEL`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`intel_performance_query`]
- [`Device`]
- [`PerformanceConfigurationAcquireInfoINTEL`]
- [`PerformanceConfigurationINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        