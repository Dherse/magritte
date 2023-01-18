[vkGetRefreshCycleDurationGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetRefreshCycleDurationGOOGLE.html) - Obtain the RC duration of the PE's display

# C Specifications
To query the duration of a refresh cycle (RC) for the presentation engine’s
display, call:
```c
// Provided by VK_GOOGLE_display_timing
VkResult vkGetRefreshCycleDurationGOOGLE(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    VkRefreshCycleDurationGOOGLE*               pDisplayTimingProperties);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to obtain the refresh duration for.
- [`p_display_timing_properties`] is a pointer to a [`RefreshCycleDurationGOOGLE`] structure.

# Description
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-  [`p_display_timing_properties`] **must**  be a valid pointer to a [`RefreshCycleDurationGOOGLE`] structure
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`VK_GOOGLE_display_timing`]
- [`Device`]
- [`RefreshCycleDurationGOOGLE`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        