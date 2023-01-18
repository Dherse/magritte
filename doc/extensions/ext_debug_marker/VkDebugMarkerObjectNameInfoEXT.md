[VkDebugMarkerObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugMarkerObjectNameInfoEXT.html) - Specify parameters of a name to give to an object

# C Specifications
The [`DebugMarkerObjectNameInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_debug_marker
typedef struct VkDebugMarkerObjectNameInfoEXT {
    VkStructureType               sType;
    const void*                   pNext;
    VkDebugReportObjectTypeEXT    objectType;
    uint64_t                      object;
    const char*                   pObjectName;
} VkDebugMarkerObjectNameInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`object_type`] is a [`DebugReportObjectTypeEXT`] specifying the type of the object to be named.
- [`object`] is the object to be named.
- [`object_name`] is a null-terminated UTF-8 string specifying the name to apply to [`object`].

# Description
Applications **may**  change the name associated with an object simply by
calling [`debug_marker_set_object_name_ext`] again with a new string.
To remove a previously set name, [`object_name`] **should**  be set to an
empty string.
## Valid Usage
-  [`object_type`] **must**  not be `VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT`
-  [`object`] **must**  not be [`crate::Handle::null`]
-  [`object`] **must**  be a Vulkan object of the type associated with [`object_type`] as defined in [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debug-report-object-types)

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_MARKER_OBJECT_NAME_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`object_type`] **must**  be a valid [`DebugReportObjectTypeEXT`] value
-  [`object_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`VK_EXT_debug_marker`]
- [`DebugReportObjectTypeEXT`]
- [`StructureType`]
- [`debug_marker_set_object_name_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        