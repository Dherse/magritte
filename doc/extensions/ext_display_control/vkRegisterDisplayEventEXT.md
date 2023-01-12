[vkRegisterDisplayEventEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkRegisterDisplayEventEXT.html) - Signal a fence when a display event occurs

# C Specifications
To create a fence that will be signaled when an event occurs on a
[`DisplayKHR`] object, call:
```c
// Provided by VK_EXT_display_control
VkResult vkRegisterDisplayEventEXT(
    VkDevice                                    device,
    VkDisplayKHR                                display,
    const VkDisplayEventInfoEXT*                pDisplayEventInfo,
    const VkAllocationCallbacks*                pAllocator,
    VkFence*                                    pFence);
```

# Parameters
- [`device`] is a logical device associated with [`display`]
- [`display`] is the display on which the event  **may**  occur.
- [`p_display_event_info`] is a pointer to a [`DisplayEventInfoEXT`] structure describing the event of interest to the application.
- [`p_allocator`] controls host memory allocation as described in the [Memory Allocation](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#memory-allocation) chapter.
- [`p_fence`] is a pointer to a handle in which the resulting fence object is returned.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`display`] **must**  be a valid [`DisplayKHR`] handle
-  [`p_display_event_info`] **must**  be a valid pointer to a valid [`DisplayEventInfoEXT`] structure
-    If [`p_allocator`] is not `NULL`, [`p_allocator`] **must**  be a valid pointer to a valid [`AllocationCallbacks`] structure
-  [`p_fence`] **must**  be a valid pointer to a [`Fence`] handle
-    Both of [`device`], and [`display`] **must**  have been created, allocated, or retrieved from the same [`PhysicalDevice`]

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`

# Related
- [`ext_display_control`]
- [`AllocationCallbacks`]
- [`Device`]
- [`DisplayEventInfoEXT`]
- [`DisplayKHR`]
- [`Fence`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        