[vkDestroyBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBufferCollectionFUCHSIA.html) - Destroy a buffer collection

# C Specifications
To release a [`BufferCollectionFUCHSIA`]:
```c
// Provided by VK_FUCHSIA_buffer_collection
void vkDestroyBufferCollectionFUCHSIA(
    VkDevice                                    device,
    VkBufferCollectionFUCHSIA                   collection,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
- [`collection`] is the [`BufferCollectionFUCHSIA`] handle
- [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter

# Description
## Valid Usage
-  [`Image`] and [`Buffer`] objects that referenced [`collection`] upon creation by inclusion of a [`BufferCollectionImageCreateInfoFUCHSIA`] or [`BufferCollectionBufferCreateInfoFUCHSIA`] chained to their [`ImageCreateInfo`] or [`BufferCreateInfo`] structures respectively,  **may**  outlive [`collection`].

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`collection`] **must**  be a valid [`BufferCollectionFUCHSIA`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`collection`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`AllocationCallbacks`]
- [`BufferCollectionFUCHSIA`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        