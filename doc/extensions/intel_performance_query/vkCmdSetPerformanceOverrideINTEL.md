[vkCmdSetPerformanceOverrideINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html) - Performance override settings

# C Specifications
Some applications might want measure the effect of a set of commands with a
different settings.
It is possible to override a particular settings using :
```c
// Provided by VK_INTEL_performance_query
VkResult vkCmdSetPerformanceOverrideINTEL(
    VkCommandBuffer                             commandBuffer,
    const VkPerformanceOverrideInfoINTEL*       pOverrideInfo);
```

# Parameters
- [`command_buffer`] is the command buffer where the override takes place.
- [`p_override_info`] is a pointer to a [`PerformanceOverrideInfoINTEL`] structure selecting the parameter to override.

# Description
## Valid Usage
-  [`p_override_info`] **must**  not be used with a [`PerformanceOverrideTypeINTEL`] that is not reported available by [`get_performance_parameter_intel`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_override_info`] **must**  be a valid pointer to a valid [`PerformanceOverrideInfoINTEL`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, compute, or transfer operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties
## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_TOO_MANY_OBJECTS`  - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_INTEL_performance_query`]
- [`CommandBuffer`]
- [`PerformanceOverrideInfoINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        