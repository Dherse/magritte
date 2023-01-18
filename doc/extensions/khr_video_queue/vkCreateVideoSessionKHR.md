[vkCreateVideoSessionKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionKHR.html) - Creates a video session object

# C Specifications
To create a video session object, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkCreateVideoSessionKHR(
    VkDevice                                    device,
    const VkVideoSessionCreateInfoKHR*          pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkVideoSessionKHR*                          pVideoSession);
```

# Parameters
- [`device`] is the logical device that creates the decode or encode session object.
- [`p_create_info`] is a pointer to a [`VideoSessionCreateInfoKHR`] structure containing parameters specifying the creation of the decode or encode session.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_video_session`] is a pointer to a [`VideoSessionKHR`] structure specifying the decode or encode video session object which will be created by this function when it returns `VK_SUCCESS`

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`VideoSessionCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_video_session`] **must**  be a valid pointer to a [`VideoSessionKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_INCOMPATIBLE_DRIVER`  - `VK_ERROR_FEATURE_NOT_PRESENT`

# Related
- [`VK_KHR_video_queue`]
- [`AllocationCallbacks`]
- [`Device`]
- [`VideoSessionCreateInfoKHR`]
- [`VideoSessionKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        