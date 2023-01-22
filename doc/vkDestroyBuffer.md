[vkDestroyBuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyBuffer.html) - Destroy a buffer object

# C Specifications
To destroy a buffer, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyBuffer(
    VkDevice                                    device,
    VkBuffer                                    buffer,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that destroys the buffer.
- [`buffer`] is the buffer to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    All submitted commands that refer to [`buffer`], either directly or via a [`BufferView`],  **must**  have completed execution
-    If [`AllocationCallbacks`] were provided when [`buffer`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`buffer`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`buffer`] is not [`crate::Handle::null`], [`buffer`] **must**  be a valid [`Buffer`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`buffer`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`buffer`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Buffer`]
- [`Device`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        