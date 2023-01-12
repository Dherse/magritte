[VkDebugUtilsObjectNameInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectNameInfoEXT.html) - Specify parameters of a name to give to an object

# C Specifications
The [`DebugUtilsObjectNameInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_debug_utils
typedef struct VkDebugUtilsObjectNameInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkObjectType       objectType;
    uint64_t           objectHandle;
    const char*        pObjectName;
} VkDebugUtilsObjectNameInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
- [`object_handle`] is the object to be named.
- [`object_name`] is either `NULL` or a null-terminated UTF-8 string specifying the name to apply to [`object_handle`].

# Description
Applications **may**  change the name associated with an object simply by
calling [`set_debug_utils_object_name_ext`] again with a new string.
If [`object_name`] is either `NULL` or an empty string, then any
previously set name is removed.
## Valid Usage
-    If [`object_type`] is `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`] **must**  not be [`crate::Handle::null`]
-    If [`object_type`] is not `VK_OBJECT_TYPE_UNKNOWN`, [`object_handle`] **must**  be [`crate::Handle::null`] or a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_NAME_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`object_type`] **must**  be a valid [`ObjectType`] value
-    If [`object_name`] is not `NULL`, [`object_name`] **must**  be a null-terminated UTF-8 string

# Related
- [`ext_debug_utils`]
- [`DebugUtilsMessengerCallbackDataEXT`]
- [`ObjectType`]
- [`StructureType`]
- [`set_debug_utils_object_name_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        