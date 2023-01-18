[VK_KHR_xcb_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_xcb_surface.html) - instance extension

# Description
The [`VK_KHR_xcb_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`VK_KHR_surface`]` extension) that refers to an X11 [`Window`],
using the XCB client-side library, as well as a query to determine support
for rendering via XCB.

# Registered extension number
6

# Revision
6

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xcb_surface] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_xcb_surface extension>>)
- Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_xcb_surface] @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_xcb_surface extension>>)

# New commands
- [`create_xcb_surface_khr`]
- [`get_physical_device_xcb_presentation_support_khr`]

# New structures
- [`XcbSurfaceCreateInfoKHR`]

# New bitmasks
- [`XcbSurfaceCreateFlagsKHR`]

# New constants
- [`KHR_XCB_SURFACE_EXTENSION_NAME`]
- [`KHR_XCB_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR`

# Known issues & F.A.Q.
1) Does XCB need a way to query for compatibility between a particular
physical device and a specific screen? This would be a more general query
than [`get_physical_device_surface_support_khr`]: If it returned
[`TRUE`], then the physical device could be assumed to support
presentation to any window on that screen. **RESOLVED** : Yes, this is needed for toolkits that want to create a
[`Device`] before creating a window.
To ensure the query is reliable, it must be made against a particular X
visual rather than the screen in general.

# Version history
- Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft, based on the previous contents of VK_EXT_KHR_swapchain (later renamed VK_EXT_KHR_surface). 
- Revision 2, 2015-10-02 (James Jones)  - Added presentation support query for an (xcb_connection_t*, xcb_visualid_t) pair.  - Removed “root” parameter from CreateXcbSurfaceKHR(), as it is redundant when a window on the same screen is specified as well.  - Adjusted wording of issue #1 and added agreed upon resolution. 
- Revision 3, 2015-10-14 (Ian Elliott)  - Removed “root” parameter from CreateXcbSurfaceKHR() in one more place. 
- Revision 4, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_xcb_surface to VK_KHR_xcb_surface. 
- Revision 5, 2015-10-23 (Daniel Rakos)  - Added allocation callbacks to vkCreateXcbSurfaceKHR. 
- Revision 6, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a pCreateInfo structure.

# Other information
* 2015-11-28
* No known IP claims.
*   - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour, Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach, Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm  - Chia-I Wu, LunarG

# Related
- [`XcbSurfaceCreateFlagsKHR`]
- [`XcbSurfaceCreateInfoKHR`]
- [`create_xcb_surface_khr`]
- [`get_physical_device_xcb_presentation_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        