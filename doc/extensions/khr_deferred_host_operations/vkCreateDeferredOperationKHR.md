[vkCreateDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDeferredOperationKHR.html) - Create a deferred operation handle

# C Specifications
To construct the tracking object for a deferred command, call:
```c
// Provided by VK_KHR_deferred_host_operations
VkResult vkCreateDeferredOperationKHR(
    VkDevice                                    device,
    const VkAllocationCallbacks*                pAllocator,
    VkDeferredOperationKHR*                     pDeferredOperation);
```

# Parameters
- [`device`] is the device which owns `operation`.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_deferred_operation`] is a pointer to a handle in which the created [`DeferredOperationKHR`] is returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_deferred_operation`] **must**  be a valid pointer to a [`DeferredOperationKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`VK_KHR_deferred_host_operations`]
- [`AllocationCallbacks`]
- [`DeferredOperationKHR`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        