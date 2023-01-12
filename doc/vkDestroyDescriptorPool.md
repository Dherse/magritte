[vkDestroyDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyDescriptorPool.html) - Destroy a descriptor pool object

# C Specifications
To destroy a descriptor pool, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyDescriptorPool(
    VkDevice                                    device,
    VkDescriptorPool                            descriptorPool,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the descriptor pool.
- [`descriptor_pool`] is the descriptor pool to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
When a pool is destroyed, all descriptor sets allocated from the pool are
implicitly freed and become invalid.
Descriptor sets allocated from a given pool do not need to be freed before
destroying that descriptor pool.
## Valid Usage
-    All submitted commands that refer to [`descriptor_pool`] (via any allocated descriptor sets)  **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`descriptor_pool`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`descriptor_pool`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`descriptor_pool`] is not [`crate::Handle::null`], [`descriptor_pool`] **must**  be a valid [`DescriptorPool`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`descriptor_pool`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`descriptor_pool`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`DescriptorPool`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        