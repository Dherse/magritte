[VK_EXT_acquire_drm_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_drm_display.html) - instance extension

# Description
This extension allows an application to take exclusive control of a display
using the Direct Rendering Manager (DRM) interface.
When acquired, the display will be under full control of the application
until the display is either released or the connector is unplugged.

# Registered extension number
286

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_EXT_direct_mode_display`]`

# Contacts
- Drew DeVault [sir@cmpwn.com]()

# New commands
- [`acquire_drm_display_ext`]
- [`get_drm_display_ext`]

# New constants
- [`EXT_ACQUIRE_DRM_DISPLAY_EXTENSION_NAME`]
- [`EXT_ACQUIRE_DRM_DISPLAY_SPEC_VERSION`]

# Known issues & F.A.Q.
None.

# Version history
- Revision 1, 2021-05-11 (Simon Zeni)  - Initial draft

# Other information
* 2021-06-09
* No known IP claims.
*   - Simon Zeni, Status Holdings, Ltd.

# Related
- [`acquire_drm_display_ext`]
- [`get_drm_display_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        