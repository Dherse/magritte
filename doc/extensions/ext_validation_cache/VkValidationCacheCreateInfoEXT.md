[VkValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheCreateInfoEXT.html) - Structure specifying parameters of a newly created validation cache

# C Specifications
The [`ValidationCacheCreateInfoEXT`] structure is defined as:
```c
// Provided by VK_EXT_validation_cache
typedef struct VkValidationCacheCreateInfoEXT {
    VkStructureType                    sType;
    const void*                        pNext;
    VkValidationCacheCreateFlagsEXT    flags;
    size_t                             initialDataSize;
    const void*                        pInitialData;
} VkValidationCacheCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`flags`] is reserved for future use.
- [`initial_data_size`] is the number of bytes in [`initial_data`]. If [`initial_data_size`] is zero, the validation cache will initially be empty.
- [`initial_data`] is a pointer to previously retrieved validation cache data. If the validation cache data is incompatible (as defined below) with the device, the validation cache will be initially empty. If [`initial_data_size`] is zero, [`initial_data`] is ignored.

# Description
## Valid Usage
-    If [`initial_data_size`] is not `0`, it  **must**  be equal to the size of [`initial_data`], as returned by [`get_validation_cache_data_ext`] when [`initial_data`] was originally retrieved
-    If [`initial_data_size`] is not `0`, [`initial_data`] **must**  have been retrieved from a previous call to [`get_validation_cache_data_ext`]

## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_VALIDATION_CACHE_CREATE_INFO_EXT`
-  [`p_next`] **must**  be `NULL`
-  [`flags`] **must**  be `0`
-    If [`initial_data_size`] is not `0`, [`initial_data`] **must**  be a valid pointer to an array of [`initial_data_size`] bytes

# Related
- [`VK_EXT_validation_cache`]
- [`StructureType`]
- [`ValidationCacheCreateFlagsEXT`]
- [`create_validation_cache_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        