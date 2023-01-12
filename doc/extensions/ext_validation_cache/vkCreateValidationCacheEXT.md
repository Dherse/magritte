[vkCreateValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateValidationCacheEXT.html) - Creates a new validation cache

# C Specifications
To create validation cache objects, call:
```c
// Provided by VK_EXT_validation_cache
VkResult vkCreateValidationCacheEXT(
    VkDevice                                    device,
    const VkValidationCacheCreateInfoEXT*       pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkValidationCacheEXT*                       pValidationCache);
```

# Parameters
- [`device`] is the logical device that creates the validation cache object.
- [`p_create_info`] is a pointer to a [`ValidationCacheCreateInfoEXT`] structure containing the initial parameters for the validation cache object.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_validation_cache`] is a pointer to a [`ValidationCacheEXT`] handle in which the resulting validation cache object is returned.

# Description
Once created, a validation cache  **can**  be passed to the
[`create_shader_module`] command by adding this object to the
[`ShaderModuleCreateInfo`] structure’s `pNext` chain.
If a [`ShaderModuleValidationCacheCreateInfoEXT`] object is included in
the [`ShaderModuleCreateInfo::p_next`] chain, and its
`validationCache` field is not [`crate::Handle::null`], the implementation
will query it for possible reuse opportunities and update it with new
content.
The use of the validation cache object in these commands is internally
synchronized, and the same validation cache object  **can**  be used in multiple
threads simultaneously.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`ValidationCacheCreateInfoEXT`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_validation_cache`] **must**  be a valid pointer to a [`ValidationCacheEXT`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_validation_cache`]
- [`AllocationCallbacks`]
- [`Device`]
- [`ValidationCacheCreateInfoEXT`]
- [`ValidationCacheEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        