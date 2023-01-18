[VkValidationCacheHeaderVersionEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkValidationCacheHeaderVersionEXT.html) - Encode validation cache version

# C Specifications
Possible values of the second group of four bytes in the header returned by
[`get_validation_cache_data_ext`], encoding the validation cache version,
are:
```c
// Provided by VK_EXT_validation_cache
typedef enum VkValidationCacheHeaderVersionEXT {
    VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1,
} VkValidationCacheHeaderVersionEXT;
```

# Description
- [`ONE`] specifies version one of the validation cache.

# Related
- [`VK_EXT_validation_cache`]
- [`create_validation_cache_ext`]
- [`get_validation_cache_data_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        