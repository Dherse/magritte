[vkQueueSetPerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html) - Set a performance query

# C Specifications
To set a performance configuration, call:
```c
// Provided by VK_INTEL_performance_query
VkResult vkQueueSetPerformanceConfigurationINTEL(
    VkQueue                                     queue,
    VkPerformanceConfigurationINTEL             configuration);
```

# Parameters
- [`queue`] is the queue on which the configuration will be used.
- [`configuration`] is the configuration to use.

# Description
## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-  [`configuration`] **must**  be a valid [`PerformanceConfigurationINTEL`] handle
-    Both of [`configuration`], and [`queue`] **must**  have been created, allocated, or retrieved from the same [`Device`]

## Command Properties
## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`intel_performance_query`]
- [`PerformanceConfigurationINTEL`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        