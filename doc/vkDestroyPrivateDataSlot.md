[vkDestroyPrivateDataSlot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyPrivateDataSlot.html) - Destroy a private data slot

# C Specifications
To destroy a private data slot, call:
```c
// Provided by VK_VERSION_1_3
void vkDestroyPrivateDataSlot(
    VkDevice                                    device,
    VkPrivateDataSlot                           privateDataSlot,
    const VkAllocationCallbacks*                pAllocator);
```
or the equivalent command
```c
// Provided by VK_EXT_private_data
void vkDestroyPrivateDataSlotEXT(
    VkDevice                                    device,
    VkPrivateDataSlot                           privateDataSlot,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device associated with the creation of the object(s) holding the private data slot.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`private_data_slot`] is the private data slot to destroy.

# Description
## Valid Usage
-    If [`AllocationCallbacks`] were provided when [`private_data_slot`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`private_data_slot`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`private_data_slot`] is not [`crate::Handle::null`], [`private_data_slot`] **must**  be a valid [`PrivateDataSlot`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`private_data_slot`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`private_data_slot`] **must**  be externally synchronized

# Related
- [`VK_EXT_private_data`]
- [`crate::vulkan1_3`]
- [`AllocationCallbacks`]
- [`Device`]
- [`PrivateDataSlot`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        