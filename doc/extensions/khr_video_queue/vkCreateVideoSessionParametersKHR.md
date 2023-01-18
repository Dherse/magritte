[vkCreateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkCreateVideoSessionParametersKHR.html) - Creates video session video session parameter object

# C Specifications
To create a video session parameters object, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkCreateVideoSessionParametersKHR(
    VkDevice                                    device,
    const VkVideoSessionParametersCreateInfoKHR* pCreateInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkVideoSessionParametersKHR*                pVideoSessionParameters);
```

# Parameters
- [`device`] is the logical device that was used for the creation of the video session object.
- [`p_create_info`] is a pointer to [`VideoSessionParametersCreateInfoKHR`] structure specifying the video session parameters.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_video_session_parameters`] is a pointer to a [`VideoSessionParametersKHR`] handle in which the video session parameters object is returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`p_create_info`] **must**  be a valid pointer to a valid [`VideoSessionParametersCreateInfoKHR`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_video_session_parameters`] **must**  be a valid pointer to a [`VideoSessionParametersKHR`] handle

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_TOO_MANY_OBJECTS`

# Related
- [`VK_KHR_video_queue`]
- [`AllocationCallbacks`]
- [`Device`]
- [`VideoSessionParametersCreateInfoKHR`]
- [`VideoSessionParametersKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        