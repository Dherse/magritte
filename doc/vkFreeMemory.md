[vkFreeMemory](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkFreeMemory.html) - Free device memory

# C Specifications
To free a memory object, call:
```c
// Provided by VK_VERSION_1_0
void vkFreeMemory(
    VkDevice                                    device,
    VkDeviceMemory                              memory,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the logical device that owns the memory.
- [`memory`] is the [`DeviceMemory`] object to be freed.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
Before freeing a memory object, an application  **must**  ensure the memory
object is no longer in use by the device — for example by command buffers
in the *pending state*.
Memory  **can**  be freed whilst still bound to resources, but those resources
 **must**  not be used afterwards.
Freeing a memory object releases the reference it held, if any, to its
payload.
If there are still any bound images or buffers, the memory object’s payload
 **may**  not be immediately released by the implementation, but  **must**  be
released by the time all bound images and buffers have been destroyed.
Once all references to a payload are released, it is returned to the heap
from which it was allocated.How memory objects are bound to Images and Buffers is described in detail in
the [Resource Memory Association](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#resources-association) section.If a memory object is mapped at the time it is freed, it is implicitly
unmapped.
## Valid Usage
-    All submitted commands that refer to [`memory`] (via images or buffers)  **must**  have completed execution

## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-    If [`memory`] is not [`crate::Handle::null`], [`memory`] **must**  be a valid [`DeviceMemory`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-    If [`memory`] is a valid handle, it  **must**  have been created, allocated, or retrieved from [`device`]

## Host Synchronization
- Host access to [`memory`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Device`]
- [`DeviceMemory`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        