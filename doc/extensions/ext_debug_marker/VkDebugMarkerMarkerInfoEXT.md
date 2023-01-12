[VkDebugMarkerMarkerInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerMarkerInfoEXT.html) - Specify parameters of a command buffer marker region

# C Specifications
The [`DebugMarkerMarkerInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_debug_marker
typedef struct VkDebugMarkerMarkerInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    const char*        pMarkerName;
    float              color[4];
} VkDebugMarkerMarkerInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`marker_name`] is a pointer to a null-terminated UTF-8 string containing the name of the marker.
- [`color`] is an  **optional**  RGBA color value that can be associated with the marker. A particular implementation  **may**  choose to ignore this color value. The values contain RGBA values in order, in the range 0.0 to 1.0. If all elements in [`color`] are set to 0.0 then it is ignored.

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_MARKER_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`marker_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`ext_debug_marker`]
- [`StructureType`]
- [`cmd_debug_marker_begin_ext`]
- [`cmd_debug_marker_insert_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        