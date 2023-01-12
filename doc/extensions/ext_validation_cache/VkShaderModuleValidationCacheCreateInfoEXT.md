[VkShaderModuleValidationCacheCreateInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkShaderModuleValidationCacheCreateInfoEXT.html) - Specify validation cache to use during shader module creation

# C Specifications
To use a [`ValidationCacheEXT`] to cache shader validation results, add
a [`ShaderModuleValidationCacheCreateInfoEXT`] structure to the
[`p_next`] chain of the [`ShaderModuleCreateInfo`] structure,
specifying the cache object to use.The [`ShaderModuleValidationCacheCreateInfoEXT`] struct is defined as:
```c
// Provided by VK_EXT_validation_cache
typedef struct VkShaderModuleValidationCacheCreateInfoEXT {
    VkStructureType         sType;
    const void*             pNext;
    VkValidationCacheEXT    validationCache;
} VkShaderModuleValidationCacheCreateInfoEXT;
```

# Members
- [`s_type`] is the type of this structure.
- [`p_next`] is `NULL` or a pointer to a structure extending this structure.
- [`validation_cache`] is the validation cache object from which the results of prior validation attempts will be written, and to which new validation results for this [`ShaderModule`] will be written (if not already present).

# Description
## Valid Usage (Implicit)
-  [`s_type`] **must**  be `VK_STRUCTURE_TYPE_SHADER_MODULE_VALIDATION_CACHE_CREATE_INFO_EXT`
-  [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle

# Related
- [`ext_validation_cache`]
- [`StructureType`]
- [`ValidationCacheEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        