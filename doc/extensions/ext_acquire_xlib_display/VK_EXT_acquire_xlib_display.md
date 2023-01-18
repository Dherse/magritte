[VK_EXT_acquire_xlib_display](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_acquire_xlib_display.html) - instance extension

# Description
This extension allows an application to take exclusive control on a display
currently associated with an X11 screen.
When control is acquired, the display will be deassociated from the X11
screen until control is released or the specified display connection is
closed.
Essentially, the X11 screen will behave as if the monitor has been unplugged
until control is released.

# Registered extension number
90

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_EXT_direct_mode_display`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_acquire_xlib_display] @cubanismo%0A<<Here describe the issue or question you have about the VK_EXT_acquire_xlib_display extension>>)

# New commands
- [`acquire_xlib_display_ext`]
- [`get_rand_r_output_display_ext`]

# New constants
- [`EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME`]
- [`EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION`]

# Known issues & F.A.Q.
1) Should [`acquire_xlib_display_ext`] take an RandR display ID, or a
Vulkan display handle as input? **RESOLVED** : A Vulkan display handle.
Otherwise there would be no way to specify handles to displays that had been
prevented from being included in the X11 display list by some native
platform or vendor-specific mechanism.2) How does an application figure out which RandR display corresponds to a
Vulkan display? **RESOLVED** : A new function, [`get_rand_r_output_display_ext`], is introduced
for this purpose.3) Should [`get_rand_r_output_display_ext`] be part of this extension, or a
general Vulkan / RandR or Vulkan / Xlib extension? **RESOLVED** : To avoid yet another extension, include it in this extension.

# Version history
- Revision 1, 2016-12-13 (James Jones)  - Initial draft

# Other information
* 2016-12-13
* No known IP claims.
*   - Dave Airlie, Red Hat  - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup Griffais, Valve  - Liam Middlebrook, NVIDIA  - Daniel Vetter, Intel

# Related
- [`acquire_xlib_display_ext`]
- [`get_rand_r_output_display_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        