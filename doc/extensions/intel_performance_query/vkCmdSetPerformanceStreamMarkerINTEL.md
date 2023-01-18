[vkCmdSetPerformanceStreamMarkerINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html) - Markers

# C Specifications
When monitoring the behavior of an application wihtin the dataset generated
by the entire set of applications running on the system, it is useful to
identify draw calls within a potentially huge amount of performance data.
To do so, application can generate stream markers that will be used to trace
back a particular draw call with a particular performance data item.
```c
// Provided by VK_INTEL_performance_query
VkResult vkCmdSetPerformanceStreamMarkerINTEL(
    VkCommandBuffer                             commandBuffer,
    const VkPerformanceStreamMarkerInfoINTEL*   pMarkerInfo);
```

# Description
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_marker_info`] **must**  be a valid pointer to a valid [`PerformanceStreamMarkerInfoINTEL`] structure
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
- [`PerformanceStreamMarkerInfoINTEL`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        