[vkGetPerformanceParameterINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html) - Query performance capabilities of the device

# C Specifications
Some performance query features of a device can be discovered with the call:
```c
// Provided by VK_INTEL_performance_query
VkResult vkGetPerformanceParameterINTEL(
    VkDevice                                    device,
    VkPerformanceParameterTypeINTEL             parameter,
    VkPerformanceValueINTEL*                    pValue);
```

# Parameters
- [`device`] is the logical device to query.
- [`parameter`] is the parameter to query.
- [`p_value`] is a pointer to a [`PerformanceValueINTEL`] structure in which the type and value of the parameter are returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`parameter`] **must**  be a valid [`PerformanceParameterTypeINTEL`] value
-  [`p_value`] **must**  be a valid pointer to a [`PerformanceValueINTEL`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`intel_performance_query`]
- [`Device`]
- [`PerformanceParameterTypeINTEL`]
- [`PerformanceValueINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        