[vkDestroyDeferredOperationKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDeferredOperationKHR.html) - Destroy a deferred operation handle

# C Specifications
When a deferred operation is completed, the application  **can**  destroy the
tracking object by calling:
```c
// Provided by VK_KHR_deferred_host_operations
void vkDestroyDeferredOperationKHR(
    VkDevice                                    device,
    VkDeferredOperationKHR                      operation,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the device which owns [`operation`].
- [`operation`] is the completed operation to be destroyed.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`operation`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`operation`] was created, [`p_allocator`] **must**  be `NULL`
-  [`operation`] **must**  be completed

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`operation`] is not [`crate::Handle::null`], [`operation`] **must**  be a valid [`DeferredOperationKHR`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`operation`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`operation`] **must**  be externally synchronized

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
        