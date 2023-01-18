[VkDebugUtilsObjectTagInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDebugUtilsObjectTagInfoEXT.html) - Specify parameters of a tag to attach to an object

# C Specifications
The [`DebugUtilsObjectTagInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_debug_utils
typedef struct VkDebugUtilsObjectTagInfoEXT {
    VkStructureType    sType;
    const void*        pNext;
    VkObjectType       objectType;
    uint64_t           objectHandle;
    uint64_t           tagName;
    size_t             tagSize;
    const void*        pTag;
} VkDebugUtilsObjectTagInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`object_type`] is a [`ObjectType`] specifying the type of the object to be named.
- [`object_handle`] is the object to be tagged.
- [`tag_name`] is a numerical identifier of the tag.
- [`tag_size`] is the number of bytes of data to attach to the object.
- [`tag`] is a pointer to an array of [`tag_size`] bytes containing the data to be associated with the object.

# Description
The [`tag_name`] parameter gives a name or identifier to the type of data
being tagged.
This can be used by debugging layers to easily filter for only data that can
be used by that implementation.
## Valid Usage
-  [`object_type`] **must**  not be `VK_OBJECT_TYPE_UNKNOWN`
-  [`object_handle`] **must**  be a valid Vulkan handle of the type associated with [`object_type`] as defined in the [[`ObjectType`] and Vulkan Handle Relationship](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#debugging-object-types) table

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_DEBUG_UTILS_OBJECT_TAG_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`object_type`] **must**  be a valid [`ObjectType`] value
-  [`tag`] **must**  be a valid pointer to an array of [`tag_size`] bytes
-  [`tag_size`] **must**  be greater than `0`

# Related
- [`VK_EXT_debug_utils`]
- [`ObjectType`]
- [`StructureType`]
- [`set_debug_utils_object_tag_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        