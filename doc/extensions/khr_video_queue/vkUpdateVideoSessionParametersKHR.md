[vkUpdateVideoSessionParametersKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkUpdateVideoSessionParametersKHR.html) - Update video session video session parameter object

# C Specifications
To update, add, or remove video session parameters state, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkUpdateVideoSessionParametersKHR(
    VkDevice                                    device,
    VkVideoSessionParametersKHR                 videoSessionParameters,
    const VkVideoSessionParametersUpdateInfoKHR* pUpdateInfo);
```

# Parameters
- [`device`] is the logical device that was used for the creation of the video session object.
- [`video_session_parameters`] is the video session object that is going to be updated.
- [`p_update_info`] is a pointer to a [`VideoSessionParametersUpdateInfoKHR`] structure containing the session parameters update information.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`video_session_parameters`] **must**  be a valid [`VideoSessionParametersKHR`] handle
-  [`p_update_info`] **must**  be a valid pointer to a valid [`VideoSessionParametersUpdateInfoKHR`] structure

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_INITIALIZATION_FAILED`  - `VK_ERROR_TOO_MANY_OBJECTS`

# Related
- [`VK_KHR_video_queue`]
- [`Device`]
- [`VideoSessionParametersKHR`]
- [`VideoSessionParametersUpdateInfoKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        