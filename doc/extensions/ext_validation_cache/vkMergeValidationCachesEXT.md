[vkMergeValidationCachesEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkMergeValidationCachesEXT.html) - Combine the data stores of validation caches

# C Specifications
Validation cache objects  **can**  be merged using the command:
```c
// Provided by VK_EXT_validation_cache
VkResult vkMergeValidationCachesEXT(
    VkDevice                                    device,
    VkValidationCacheEXT                        dstCache,
    uint32_t                                    srcCacheCount,
    const VkValidationCacheEXT*                 pSrcCaches);
```

# Parameters
- [`device`] is the logical device that owns the validation cache objects.
- [`dst_cache`] is the handle of the validation cache to merge results into.
- [`src_cache_count`] is the length of the [`p_src_caches`] array.
- [`p_src_caches`] is a pointer to an array of validation cache handles, which will be merged into [`dst_cache`]. The previous contents of [`dst_cache`] are included after the merge.

# Description
## Valid Usage
-  [`dst_cache`] **must**  not appear in the list of source caches

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`dst_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
-  [`p_src_caches`] **must**  be a valid pointer to an array of [`src_cache_count`] valid [`ValidationCacheEXT`] handles
-  [`src_cache_count`] **must**  be greater than `0`
-  [`dst_cache`] **must**  have been created, allocated, or retrieved from [`device`]
-    Each element of [`p_src_caches`] **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`dst_cache`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`

# Related
- [`ext_validation_cache`]
- [`Device`]
- [`ValidationCacheEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        