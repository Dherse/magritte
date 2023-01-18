[VkDebugUtilsLabelEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsLabelEXT.html) - Specify parameters of a label region

# C Specifications
The [`DebugUtilsLabelEXT`] structure is defined as:
```c
// Provided by VK_EXT_debug_utils
typedef struct VkDebugUtilsLabelEXT {
    VkStructureType    sType;
    const void*        pNext;
    const char*        pLabelName;
    float              color[4];
} VkDebugUtilsLabelEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`label_name`] is a pointer to a null-terminated UTF-8 string containing the name of the label.
- [`color`] is an optional RGBA color value that can be associated with the label. A particular implementation  **may**  choose to ignore this color value. The values contain RGBA values in order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it is ignored.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_LABEL_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`label_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`VK_EXT_debug_utils`]
- [`DebugUtilsMessengerCallbackDataEXT`]
- [`StructureType`]
- [`cmd_begin_debug_utils_label_ext`]
- [`cmd_insert_debug_utils_label_ext`]
- [`queue_begin_debug_utils_label_ext`]
- [`queue_insert_debug_utils_label_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        