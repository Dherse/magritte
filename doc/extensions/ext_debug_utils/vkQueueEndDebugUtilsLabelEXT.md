[vkQueueEndDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueEndDebugUtilsLabelEXT.html) - Close a queue debug label region

# C Specifications
A queue debug label region is closed by calling:
```c
// Provided by VK_EXT_debug_utils
void vkQueueEndDebugUtilsLabelEXT(
    VkQueue                                     queue);
```

# Parameters
- [`queue`] is the queue in which a debug label region should be closed.

# Description
The calls to [`queue_begin_debug_utils_label_ext`] and
[`queue_end_debug_utils_label_ext`] **must**  be matched and balanced.
## Valid Usage
-    There  **must**  be an outstanding [`queue_begin_debug_utils_label_ext`] command prior to the [`queue_end_debug_utils_label_ext`] on the queue

## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle

## Command Properties

# Related
- [`VK_EXT_debug_utils`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        