[VkPastPresentationTimingGOOGLE](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPastPresentationTimingGOOGLE.html) - Structure containing timing information about a previously-presented image

# C Specifications
The [`PastPresentationTimingGOOGLE`] structure is defined as:
```c
// Provided by VK_GOOGLE_display_timing
typedef struct VkPastPresentationTimingGOOGLE {
    uint32_t    presentID;
    uint64_t    desiredPresentTime;
    uint64_t    actualPresentTime;
    uint64_t    earliestPresentTime;
    uint64_t    presentMargin;
} VkPastPresentationTimingGOOGLE;
```

# Members
- [`present_id`] is an application-provided value that was given to a previous [`queue_present_khr`] command via [`PresentTimeGOOGLE`]::[`present_id`] (see below). It  **can**  be used to uniquely identify a previous present with the [`queue_present_khr`] command.
- [`desired_present_time`] is an application-provided value that was given to a previous [`queue_present_khr`] command via [`PresentTimeGOOGLE`]::[`desired_present_time`]. If non-zero, it was used by the application to indicate that an image not be presented any sooner than [`desired_present_time`].
- [`actual_present_time`] is the time when the image of the `swapchain` was actually displayed.
- [`earliest_present_time`] is the time when the image of the `swapchain` could have been displayed. This  **may**  differ from [`actual_present_time`] if the application requested that the image be presented no sooner than [`PresentTimeGOOGLE`]::[`desired_present_time`].
- [`present_margin`] is an indication of how early the [`queue_present_khr`] command was processed compared to how soon it needed to be processed, and still be presented at [`earliest_present_time`].

# Description
The results for a given `swapchain` and [`present_id`] are only
returned once from [`get_past_presentation_timing_google`].The application  **can**  use the [`PastPresentationTimingGOOGLE`] values to
occasionally adjust its timing.
For example, if [`actual_present_time`] is later than expected (e.g. one
`refreshDuration` late), the application may increase its target IPD to
a higher multiple of `refreshDuration` (e.g. decrease its frame rate
from 60Hz to 30Hz).
If [`actual_present_time`] and [`earliest_present_time`] are consistently
different, and if [`present_margin`] is consistently large enough, the
application may decrease its target IPD to a smaller multiple of
`refreshDuration` (e.g. increase its frame rate from 30Hz to 60Hz).
If [`actual_present_time`] and [`earliest_present_time`] are same, and if
[`present_margin`] is consistently high, the application may delay the
start of its input-render-present loop in order to decrease the latency
between user input and the corresponding present (always leaving some margin
in case a new image takes longer to render than the previous image).
An application that desires its target IPD to always be the same as
`refreshDuration`, can also adjust features until
[`actual_present_time`] is never late and [`present_margin`] is
satisfactory.

# Related
- [`google_display_timing`]
- [`get_past_presentation_timing_google`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        