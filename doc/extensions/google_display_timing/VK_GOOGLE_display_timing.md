[VK_GOOGLE_display_timing](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_GOOGLE_display_timing.html) - device extension

# Description
This device extension allows an application that uses the
`[`VK_KHR_swapchain`]` extension to obtain information about the
presentation engine’s display, to obtain timing information about each
present, and to schedule a present to happen no earlier than a desired time.
An application can use this to minimize various visual anomalies (e.g.
stuttering).Traditional game and real-time animation applications need to correctly
position their geometry for when the presentable image will be presented to
the user.
To accomplish this, applications need various timing information about the
presentation engine’s display.
They need to know when presentable images were actually presented, and when
they could have been presented.
Applications also need to tell the presentation engine to display an image
no sooner than a given time.
This allows the application to avoid stuttering, so the animation looks
smooth to the user.This extension treats variable-refresh-rate (VRR) displays as if they are
fixed-refresh-rate (FRR) displays.

# Registered extension number
93

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_swapchain`]`

# Contacts
- Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_GOOGLE_display_timing] @ianelliottus%0A<<Here describe the issue or question you have about the VK_GOOGLE_display_timing extension>>)

# New commands
- [`get_past_presentation_timing_google`]
- [`get_refresh_cycle_duration_google`]

# New structures
- [`PastPresentationTimingGOOGLE`]
- [`PresentTimeGOOGLE`]
- [`RefreshCycleDurationGOOGLE`]
- Extending [`PresentInfoKHR`]:  - [`PresentTimesInfoGOOGLE`]

# New constants
- [`GOOGLE_DISPLAY_TIMING_EXTENSION_NAME`]
- [`GOOGLE_DISPLAY_TIMING_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PRESENT_TIMES_INFO_GOOGLE`

# Version history
- Revision 1, 2017-02-14 (Ian Elliott)  - Internal revisions

# Other information
* 2017-02-14
* No known IP claims.
*   - Ian Elliott, Google  - Jesse Hall, Google

# Related
- [`PastPresentationTimingGOOGLE`]
- [`PresentTimeGOOGLE`]
- [`PresentTimesInfoGOOGLE`]
- [`RefreshCycleDurationGOOGLE`]
- [`get_past_presentation_timing_google`]
- [`get_refresh_cycle_duration_google`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        