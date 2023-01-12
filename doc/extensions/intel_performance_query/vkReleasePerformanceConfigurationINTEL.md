[vkReleasePerformanceConfigurationINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html) - Release a configuration to capture performance data

# C Specifications
To release a device performance configuration, call:
```c
// Provided by VK_INTEL_performance_query
VkResult vkReleasePerformanceConfigurationINTEL(
    VkDevice                                    device,
    VkPerformanceConfigurationINTEL             configuration);
```

# Parameters
- [`device`] is the device associated to the configuration object to release.
- [`configuration`] is the configuration object to release.

# Description
## Valid Usage
-  [`configuration`] **must**  not be released before all command buffers submitted while the configuration was set are in [pending state](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#commandbuffers-lifecycle)

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`configuration`] is not [`crate::Handle::null`], [`configuration`] **must**  be a valid [`PerformanceConfigurationINTEL`] handle
-    If [`configuration`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`configuration`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`intel_performance_query`]
- [`Device`]
- [`PerformanceConfigurationINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        