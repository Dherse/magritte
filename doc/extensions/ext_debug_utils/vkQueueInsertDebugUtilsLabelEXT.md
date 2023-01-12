[vkQueueInsertDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkQueueInsertDebugUtilsLabelEXT.html) - Insert a label into a queue

# C Specifications
A single label can be inserted into a queue by calling:
```c
// Provided by VK_EXT_debug_utils
void vkQueueInsertDebugUtilsLabelEXT(
    VkQueue                                     queue,
    const VkDebugUtilsLabelEXT*                 pLabelInfo);
```

# Parameters
- [`queue`] is the queue into which a debug label will be inserted.
- [`p_label_info`] is a pointer to a [`DebugUtilsLabelEXT`] structure specifying parameters of the label to insert.

# Description
## Valid Usage (Implicit)
-  [`queue`] **must**  be a valid [`Queue`] handle
-  [`p_label_info`] **must**  be a valid pointer to a valid [`DebugUtilsLabelEXT`] structure

## Command Properties

# Related
- [`ext_debug_utils`]
- [`DebugUtilsLabelEXT`]
- [`Queue`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        