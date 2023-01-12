[vkCreatePrivateDataSlot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreatePrivateDataSlot.html) - Create a slot for private data storage

# C Specifications
To create a private data slot, call:
```c
// Provided by VK_VERSION_1_3
VkResult vkCreatePrivateDataSlot(
    VkDevice                                    device,
    const VkPrivateDataSlotCreateInfo*          pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkPrivateDataSlot*                          pPrivateDataSlot);
```
or the equivalent command
```c
// Provided by VK_EXT_private_data
VkResult vkCreatePrivateDataSlotEXT(
    VkDevice                                    device,
    const VkPrivateDataSlotCreateInfo*          pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkPrivateDataSlot*                          pPrivateDataSlot);
```

# Parameters
- [`device`] is the logical device associated with the creation of the object(s) holding the private data slot.
- [`p_create_info`] is a pointer to a [`PrivateDataSlotCreateInfo`]
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_private_data_slot`] is a pointer to a [`PrivateDataSlot`] handle in which the resulting private data slot is returned

# Description
## Valid Usage
-    The [`privateData`](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-privateData) feature  **must**  be enabled

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`PrivateDataSlotCreateInfo`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_private_data_slot`] **must**  be a valid pointer to a [`PrivateDataSlot`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_private_data`]
- [`crate::vulkan1_3`]
- [`AllocationCallbacks`]
- [`Device`]
- [`PrivateDataSlot`]
- [`PrivateDataSlotCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        