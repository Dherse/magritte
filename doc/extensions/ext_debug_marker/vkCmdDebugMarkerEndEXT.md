[vkCmdDebugMarkerEndEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdDebugMarkerEndEXT.html) - Close a command buffer marker region

# C Specifications
A marker region can be closed by calling:
```c
// Provided by VK_EXT_debug_marker
void vkCmdDebugMarkerEndEXT(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.

# Description
An application  **may**  open a marker region in one command buffer and close it
in another, or otherwise split marker regions across multiple command
buffers or multiple queue submissions.
When viewed from the linear series of submissions to a single queue, the
calls to [`cmd_debug_marker_begin_ext`] and [`cmd_debug_marker_end_ext`] **must**  be matched and balanced.
## Valid Usage
-    There  **must**  be an outstanding [`cmd_debug_marker_begin_ext`] command prior to the [`cmd_debug_marker_end_ext`] on the queue that [`command_buffer`] is submitted to
-    If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding [`cmd_debug_marker_begin_ext`] command recorded to [`command_buffer`] that has not previously been ended by a call to [`cmd_debug_marker_end_ext`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`ext_debug_marker`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        