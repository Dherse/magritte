[vkGetPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPastPresentationTimingGOOGLE.html) - Obtain timing of a previously-presented image

# C Specifications
The implementation will maintain a limited amount of history of timing
information about previous presents.
Because of the asynchronous nature of the presentation engine, the timing
information for a given [`queue_present_khr`] command will become
available some time later.
These time values can be asynchronously queried, and will be returned if
available.
All time values are in nanoseconds, relative to a monotonically-increasing
clock (e.g. `CLOCK_MONOTONIC` (see clock_gettime(2)) on Android and Linux).To asynchronously query the presentation engine, for newly-available timing
information about one or more previous presents to a given swapchain, call:
```c
// Provided by VK_GOOGLE_display_timing
VkResult vkGetPastPresentationTimingGOOGLE(
    VkDevice                                    device,
    VkSwapchainKHR                              swapchain,
    uint32_t*                                   pPresentationTimingCount,
    VkPastPresentationTimingGOOGLE*             pPresentationTimings);
```

# Parameters
- [`device`] is the device associated with [`swapchain`].
- [`swapchain`] is the swapchain to obtain presentation timing information duration for.
- [`p_presentation_timing_count`] is a pointer to an integer related to the number of [`PastPresentationTimingGOOGLE`] structures to query, as described below.
- [`p_presentation_timings`] is either `NULL` or a pointer to an array of [`PastPresentationTimingGOOGLE`] structures.

# Description
If [`p_presentation_timings`] is `NULL`, then the number of newly-available
timing records for the given [`swapchain`] is returned in
[`p_presentation_timing_count`].
Otherwise, [`p_presentation_timing_count`] **must**  point to a variable set by
the user to the number of elements in the [`p_presentation_timings`] array,
and on return the variable is overwritten with the number of structures
actually written to [`p_presentation_timings`].
If the value of [`p_presentation_timing_count`] is less than the number of
newly-available timing records, at most [`p_presentation_timing_count`]
structures will be written, and `VK_INCOMPLETE` will be returned instead
of `VK_SUCCESS`, to indicate that not all the available timing records
were returned.
## Valid Usage (Implicit)
-  [`device`] **must**  be a valid [`Device`] handle
-  [`swapchain`] **must**  be a valid [`SwapchainKHR`] handle
-  [`p_presentation_timing_count`] **must**  be a valid pointer to a `uint32_t` value
-    If the value referenced by [`p_presentation_timing_count`] is not `0`, and [`p_presentation_timings`] is not `NULL`, [`p_presentation_timings`] **must**  be a valid pointer to an array of [`p_presentation_timing_count`][`PastPresentationTimingGOOGLE`] structures
-    Both of [`device`], and [`swapchain`] **must**  have been created, allocated, or retrieved from the same [`Instance`]

## Host Synchronization
- Host access to [`swapchain`] **must**  be externally synchronized

## Return Codes
*   - `VK_SUCCESS`  - `VK_INCOMPLETE` 
*   - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_DEVICE_LOST`  - `VK_ERROR_OUT_OF_DATE_KHR`  - `VK_ERROR_SURFACE_LOST_KHR`

# Related
- [`google_display_timing`]
- [`Device`]
- [`PastPresentationTimingGOOGLE`]
- [`SwapchainKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        