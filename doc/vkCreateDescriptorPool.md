[vkCreateDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateDescriptorPool.html) - Creates a descriptor pool object

# C Specifications
To create a descriptor pool object, call:
```c
// Provided by VK_VERSION_1_0
VkResult vkCreateDescriptorPool(
    VkDevice                                    device,
    const VkDescriptorPoolCreateInfo*           pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkDescriptorPool*                           pDescriptorPool);
```

# Parameters
- [`device`] is the logical device that creates the descriptor pool.
- [`p_create_info`] is a pointer to a [`DescriptorPoolCreateInfo`] structure specifying the state of the descriptor pool object.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_descriptor_pool`] is a pointer to a [`DescriptorPool`] handle in which the resulting descriptor pool object is returned.

# Description
The created descriptor pool is returned in [`p_descriptor_pool`].
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`DescriptorPoolCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_descriptor_pool`] **must**  be a valid pointer to a [`DescriptorPool`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_FRAGMENTATION_EXT`

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`DescriptorPool`]
- [`DescriptorPoolCreateInfo`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        