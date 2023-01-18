[vkCmdEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCmdEndDebugUtilsLabelEXT.html) - Close a command buffer label region

# C Specifications
A command buffer label region can be closed by calling:
```c
// Provided by VK_EXT_debug_utils
void vkCmdEndDebugUtilsLabelEXT(
    VkCommandBuffer                             commandBuffer);
```

# Parameters
- [`command_buffer`] is the command buffer into which the command is recorded.

# Description
An application  **may**  open a debug label region in one command buffer and
close it in another, or otherwise split debug label regions across multiple
command buffers or multiple queue submissions.
When viewed from the linear series of submissions to a single queue, the
calls to [`cmd_begin_debug_utils_label_ext`] and
[`cmd_end_debug_utils_label_ext`] **must**  be matched and balanced.There  **can**  be problems reporting command buffer debug labels during the
recording process because command buffers  **may**  be recorded out of sequence
with the resulting execution order.
Since the recording order  **may**  be different, a solitary command buffer  **may** 
have an inconsistent view of the debug label regions by itself.
Therefore, if an issue occurs during the recording of a command buffer, and
the environment requires returning debug labels, the implementation  **may** 
return only those labels it is aware of.
This is true even if the implementation is aware of only the debug labels
within the command buffer being actively recorded.
## Valid Usage
-    There  **must**  be an outstanding [`cmd_begin_debug_utils_label_ext`] command prior to the [`cmd_end_debug_utils_label_ext`] on the queue that [`command_buffer`] is submitted to
-    If [`command_buffer`] is a secondary command buffer, there  **must**  be an outstanding [`cmd_begin_debug_utils_label_ext`] command recorded to [`command_buffer`] that has not previously been ended by a call to [`cmd_end_debug_utils_label_ext`]

## Valid Usage (Implicit)
-  [`command_buffer`] **must**  be a valid [`CommandBuffer`] handle
-  [`command_buffer`] **must**  be in the [recording state]()
-    The [`CommandPool`] that [`command_buffer`] was allocated from  **must**  support graphics, or compute operations

## Host Synchronization
- Host access to [`command_buffer`] **must**  be externally synchronized
- Host access to the [`CommandPool`] that [`command_buffer`] was allocated from  **must**  be externally synchronized

## Command Properties

# Related
- [`VK_EXT_debug_utils`]
- [`CommandBuffer`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        