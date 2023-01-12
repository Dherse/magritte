[vkDestroyInstance](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyInstance.html) - Destroy an instance of Vulkan

# C Specifications
To destroy an instance, call:
```c
// Provided by VK_VERSION_1_0
void vkDestroyInstance(
    VkInstance                                  instance,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`instance`] is the handle of the instance to destroy.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage
-    All child objects created using [`instance`] **must**  have been destroyed prior to destroying [`instance`]
-    If [`AllocationCallbacks`] were provided when [`instance`] was created, a compatible set of callbacks  **must**  be provided here
-    If no [`AllocationCallbacks`] were provided when [`instance`] was created, [`p_allocator`] **must**  be `NULL`

## Valid Usage (Implicit)
-    If [`instance`] is not `NULL`, [`instance`] **must**  be a valid [`Instance`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure

## Host Synchronization
- Host access to [`instance`] **must**  be externally synchronized
- Host access to all [`PhysicalDevice`] objects enumerated from [`instance`] **must**  be externally synchronized

# Related
- [`crate::vulkan1_0`]
- [`AllocationCallbacks`]
- [`Instance`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        