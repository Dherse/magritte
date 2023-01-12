[vkGetVideoSessionMemoryRequirementsKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetVideoSessionMemoryRequirementsKHR.html) - Get Memory Requirements

# C Specifications
To get memory requirements for a video session, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkGetVideoSessionMemoryRequirementsKHR(
    VkDevice                                    device,
    VkVideoSessionKHR                           videoSession,
    uint32_t*                                   pVideoSessionMemoryRequirementsCount,
    VkVideoGetMemoryPropertiesKHR*              pVideoSessionMemoryRequirements);
```

# Parameters
- [`device`] is the logical device that owns the video session.
- [`video_session`] is the video session to query.
- [`p_video_session_memory_requirements_count`] is a pointer to an integer related to the number of memory heap requirements available or queried, as described below.
- [`p_video_session_memory_requirements`] is `NULL` or a pointer to an array of [`VideoGetMemoryPropertiesKHR`] structures in which the memory heap requirements of the video session are returned.

# Description
If [`p_video_session_memory_requirements`] is `NULL`, then the number of
memory heap types required for the video session is returned in
[`p_video_session_memory_requirements_count`].
Otherwise, [`p_video_session_memory_requirements_count`] **must**  point to a
variable set by the user with the number of elements in the
[`p_video_session_memory_requirements`] array, and on return the variable is
overwritten with the number of formats actually written to
[`p_video_session_memory_requirements`].
If [`p_video_session_memory_requirements_count`] is less than the number of
memory heap types required for the video session, then at most
[`p_video_session_memory_requirements_count`] elements will be written to
[`p_video_session_memory_requirements`], and `VK_INCOMPLETE` will be
returned, instead of `VK_SUCCESS`, to indicate that not all required
memory heap types were returned.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
-  [`p_video_session_memory_requirements_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_video_session_memory_requirements_count`] is not `0`, and [`p_video_session_memory_requirements`] is not `NULL`, [`p_video_session_memory_requirements`] **must**  be a valid pointer to an array of [`p_video_session_memory_requirements_count`][`VideoGetMemoryPropertiesKHR`] structures
-  [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`khr_video_queue`]
- [`Device`]
- [`VideoGetMemoryPropertiesKHR`]
- [`VideoSessionKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        