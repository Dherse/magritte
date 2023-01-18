[vkCmdDebugMarkerInsertEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerInsertEXT.html) - Insert a marker label into a command buffer

# C Specifications
A single marker label can be inserted into a command buffer by calling:
```c
// Provided by VK_EXT_debug_marker
void vkCmdDebugMarkerInsertEXT(
    VkCommandBuffer                             commandBuffer,
    const VkDebugMarkerMarkerInfoEXT*           pMarkerInfo);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.
- [`p_marker_info`] is a pointer to a [`DebugMarkerMarkerInfoEXT`] structure specifying the parameters of the marker to insert.

# Description
## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`p_marker_info`] **must**  be a valid pointer to a valid [`DebugMarkerMarkerInfoEXT`] structure
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_debug_marker`]
- [`CommandBuffer`]
- [`DebugMarkerMarkerInfoEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        