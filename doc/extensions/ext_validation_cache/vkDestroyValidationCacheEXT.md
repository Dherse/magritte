[vkDestroyValidationCacheEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyValidationCacheEXT.html) - Destroy a validation cache object

# C Specifications
To destroy a validation cache, call:
```c
// Provided by VK_EXT_validation_cache
void vkDestroyValidationCacheEXT(
    VkDevice                                    device,
    VkValidationCacheEXT                        validationCache,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the validation cache object.
- [`validation_cache`] is the handle of the validation cache to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`validation_cache`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`validation_cache`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`validation_cache`] is not [`crate::Handle::null`], [`validation_cache`] **must**  be a valid [`ValidationCacheEXT`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`validation_cache`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`validation_cache`] **must**  be externally synchronized

# Related
- [`VK_EXT_validation_cache`]
- [`AllocationCallbacks`]
- [`Device`]
- [`ValidationCacheEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        