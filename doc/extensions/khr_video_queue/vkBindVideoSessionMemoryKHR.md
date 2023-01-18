[vkBindVideoSessionMemoryKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkBindVideoSessionMemoryKHR.html) - Bind Video Memory

# C Specifications
To attach memory to a video session object, call:
```c
// Provided by VK_KHR_video_queue
VkResult vkBindVideoSessionMemoryKHR(
    VkDevice                                    device,
    VkVideoSessionKHR                           videoSession,
    uint32_t                                    videoSessionBindMemoryCount,
    const VkVideoBindMemoryKHR*                 pVideoSessionBindMemories);
```

# Parameters
- [`device`] is the logical device that owns the video session’s memory.
- [`video_session`] is the video session to be bound with device memory.
- [`video_session_bind_memory_count`] is the number of [`p_video_session_bind_memories`] to be bound.
- [`p_video_session_bind_memories`] is a pointer to an array of [`VideoBindMemoryKHR`] structures specifying memory regions to be bound to a device memory heap.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`video_session`] **must**  be a valid [`VideoSessionKHR`] handle
-  [`p_video_session_bind_memories`] **must**  be a valid pointer to an array of [`video_session_bind_memory_count`] valid [`VideoBindMemoryKHR`] structures
-  [`video_session_bind_memory_count`] **must**  be greater than `0`
-  [`video_session`] **must**  have been created, allocated, or retrieved from [`device`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`  - `VK_ERROR_INITIALIZATION_FAILED`

# Related
- [`VK_KHR_video_queue`]
- [`Device`]
- [`VideoBindMemoryKHR`]
- [`VideoSessionKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        