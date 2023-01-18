[vkDestroyVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkDestroyVideoSessionKHR.html) - Destroy decode session object

# C Specifications
To destroy a decode session object, call:
```c
// Provided by VK_KHR_video_queue
void vkDestroyVideoSessionKHR(
    VkDevice                                    device,
    VkVideoSessionKHR                           videoSession,
    const VkAllocationCallbacks*                pAllocator);
```

# Parameters
- [`device`] is the device that was used for the creation of the video session.
- [`video_session`] is the decode or encode video session to be destroyed.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]

# Related
- [`VK_KHR_video_queue`]
- [`AllocationCallbacks`]
- [`Device`]
- [`VideoSessionKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        