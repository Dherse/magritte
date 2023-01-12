[VK_EXT_direct_mode_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_direct_mode_display.html) - instance extension

# Description
This is extension, along with related platform extensions, allows
applications to take exclusive control of displays associated with a native
windowing system.
This is especially useful for virtual reality applications that wish to hide
HMDs (head mounted displays) from the native platform’s display management
system, desktop, and/or other applications.

# Registered extension number
89

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_display`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_direct_mode_display] @cubanismo%0A<<Here describe the issue or question you have about the VK_EXT_direct_mode_display extension>>)

# New commands
- [`release_display_ext`]

# New constants
- `VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME`
- `VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION`

# Known issues & F.A.Q.
1) Should this extension and its related platform-specific extensions
leverage `[`khr_display`]`, or provide separate equivalent interfaces. **RESOLVED** : Use `[`khr_display`]` concepts and objects.
`[`khr_display`]` can be used to enumerate all displays on the system,
including those attached to/in use by a window system or native platform,
but `[`khr_display_swapchain`]` will fail to create a swapchain on
in-use displays.
This extension and its platform-specific children will allow applications to
grab in-use displays away from window systems and/or native platforms,
allowing them to be used with `[`khr_display_swapchain`]`.2) Are separate calls needed to acquire displays and enable direct mode? **RESOLVED** : No, these operations happen in one combined command.
Acquiring a display puts it into direct mode.

# Version history
- Revision 1, 2016-12-13 (James Jones)  - Initial draft

# Other information
* 2016-12-13
* No known IP claims.
*   - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup Griffais, Valve  - Liam Middlebrook, NVIDIA

# Related
- [`release_display_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        