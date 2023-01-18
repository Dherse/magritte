[VK_EXT_full_screen_exclusive](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_full_screen_exclusive.html) - device extension

# Description
This extension allows applications to set the policy for swapchain creation
and presentation mechanisms relating to full-screen access.
Implementations may be able to acquire exclusive access to a particular
display for an application window that covers the whole screen.
This can increase performance on some systems by bypassing composition,
however it can also result in disruptive or expensive transitions in the
underlying windowing system when a change occurs.Applications can choose between explicitly disallowing or allowing this
behavior, letting the implementation decide, or managing this mode of
operation directly using the new [`acquire_full_screen_exclusive_mode_ext`]
and [`release_full_screen_exclusive_mode_ext`] commands.

# Registered extension number
256

# Revision
4

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`
- Requires `[`VK_KHR_surface`]`
- Requires `[`VK_KHR_get_surface_capabilities2`]`
- Requires `[`VK_KHR_swapchain`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_full_screen_exclusive] @cubanismo%0A<<Here describe the issue or question you have about the VK_EXT_full_screen_exclusive extension>>)

# New commands
- [`acquire_full_screen_exclusive_mode_ext`]
- [`get_physical_device_surface_present_modes2_ext`]
- [`release_full_screen_exclusive_mode_ext`]
If [`VK_KHR_device_group`] is supported:
- [`get_device_group_surface_present_modes2_ext`]
If [Version 1.1]() is supported:
- [`get_device_group_surface_present_modes2_ext`]

# New structures
- Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  - [`SurfaceFullScreenExclusiveInfoEXT`] 
- Extending [`SurfaceCapabilities2KHR`]:  - [`SurfaceCapabilitiesFullScreenExclusiveEXT`] 
If [`VK_KHR_win32_surface`] is supported:
- Extending [`PhysicalDeviceSurfaceInfo2KHR`], [`SwapchainCreateInfoKHR`]:  - [`SurfaceFullScreenExclusiveWin32InfoEXT`]

# New enums
- [`FullScreenExclusiveEXT`]

# New constants
- [`EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME`]
- [`EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION`]
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_FULL_SCREEN_EXCLUSIVE_EXT`  - `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_INFO_EXT` 
If [`VK_KHR_win32_surface`] is supported:
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_FULL_SCREEN_EXCLUSIVE_WIN32_INFO_EXT`

# Known issues & F.A.Q.
1) What should the extension & flag be called? **RESOLVED** : VK_EXT_full_screen_exclusive.Other options considered (prior to the app-controlled mode) were:
- VK_EXT_smooth_fullscreen_transition
- VK_EXT_fullscreen_behavior
- VK_EXT_fullscreen_preference
- VK_EXT_fullscreen_hint
- VK_EXT_fast_fullscreen_transition
- VK_EXT_avoid_fullscreen_exclusive
2) Do we need more than a boolean toggle? **RESOLVED** : Yes.Using an enum with default/allowed/disallowed/app-controlled enables
applications to accept driver default behavior, specifically override it in
either direction without implying the driver is ever required to use
full-screen exclusive mechanisms, or manage this mode explicitly.3) Should this be a KHR or EXT extension? **RESOLVED** : EXT, in order to allow it to be shipped faster.4) Can the fullscreen hint affect the surface capabilities, and if so,
should the hint also be specified as input when querying the surface
capabilities? **RESOLVED** : Yes on both accounts.While the hint does not guarantee a particular fullscreen mode will be used
when the swapchain is created, it can sometimes imply particular modes will
NOT be used.
If the driver determines that it will opt-out of using a particular mode
based on the policy, and knows it can only support certain capabilities if
that mode is used, it would be confusing at best to the application to
report those capabilities in such cases.
Not allowing implementations to report this state to applications could
result in situations where applications are unable to determine why
swapchain creation fails when they specify certain hint values, which could
result in never- terminating surface creation loops.5) Should full-screen be one word or two? **RESOLVED** : Two words."Fullscreen" is not in my dictionary, and web searches did not turn up
definitive proof that it is a colloquially accepted compound word.
Documentation for the corresponding Windows API mechanisms dithers.
The text consistently uses a hyphen, but none-the-less, there is a
SetFullscreenState method in the DXGI swapchain object.
Given this inconclusive external guidance, it is best to adhere to the
Vulkan style guidelines and avoid inventing new compound words.

# Version history
- Revision 4, 2019-03-12 (Tobias Hector)  - Added application-controlled mode, and related functions  - Tidied up appendix 
- Revision 3, 2019-01-03 (James Jones)  - Renamed to VK_EXT_full_screen_exclusive  - Made related adjustments to the tri-state enumerant names. 
- Revision 2, 2018-11-27 (James Jones)  - Renamed to VK_KHR_fullscreen_behavior  - Switched from boolean flag to tri-state enum 
- Revision 1, 2018-11-06 (James Jones)  - Internal revision

# Other information
* 2019-03-12
* No known IP claims.
*   - Interacts with Vulkan 1.1  - Interacts with `[`VK_KHR_device_group`]`  - Interacts with `[`VK_KHR_win32_surface`]` 
*   - Hans-Kristian Arntzen, ARM  - Slawomir Grajewski, Intel  - Tobias Hector, AMD  - James Jones, NVIDIA  - Daniel Rakos, AMD  - Jeff Juliano, NVIDIA  - Joshua Schnarr, NVIDIA  - Aaron Hagan, AMD

# Related
- [`FullScreenExclusiveEXT`]
- [`SurfaceCapabilitiesFullScreenExclusiveEXT`]
- [`SurfaceFullScreenExclusiveInfoEXT`]
- [`acquire_full_screen_exclusive_mode_ext`]
- [`get_physical_device_surface_present_modes2_ext`]
- [`release_full_screen_exclusive_mode_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        