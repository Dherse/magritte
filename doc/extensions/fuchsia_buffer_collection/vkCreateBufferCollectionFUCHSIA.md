[vkCreateBufferCollectionFUCHSIA](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateBufferCollectionFUCHSIA.html) - Create a new buffer collection

# C Specifications
To create an [`BufferCollectionFUCHSIA`] for Vulkan to participate in
the buffer collection:
```c
// Provided by VK_FUCHSIA_buffer_collection
VkResult vkCreateBufferCollectionFUCHSIA(
    VkDevice                                    device,
    const VkBufferCollectionCreateInfoFUCHSIA*  pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkBufferCollectionFUCHSIA*                  pCollection);
```

# Parameters
- [`device`] is the logical device that creates the [`BufferCollectionFUCHSIA`]
- [`p_create_info`] is a pointer to a [`BufferCollectionCreateInfoFUCHSIA`] structure containing parameters affecting creation of the buffer collection
- [`p_allocator`] is a pointer to a [`AllocationCallbacks`] structure controlling host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter
- `pBufferCollection` is a pointer to a [`BufferCollectionFUCHSIA`] handle in which the resulting buffer collection object is returned

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`BufferCollectionCreateInfoFUCHSIA`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_collection`] **must**  be a valid pointer to a [`BufferCollectionFUCHSIA`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_INVALID_EXTERNAL_HANDLE`  - `VK_ERROR_INITIALIZATION_FAILED` 

## Host AccessAll functions referencing a [`BufferCollectionFUCHSIA`] **must**  be
externally synchronized with the exception of
[`create_buffer_collection_fuchsia`].

# Related
- [`VK_FUCHSIA_buffer_collection`]
- [`AllocationCallbacks`]
- [`BufferCollectionCreateInfoFUCHSIA`]
- [`BufferCollectionFUCHSIA`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        