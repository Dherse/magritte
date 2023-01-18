[VK_KHR_android_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_android_surface.html) - instance extension

# Description
The [`VK_KHR_android_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`VK_KHR_surface`]` extension) that refers to an
[`ANativeWindow`], Android’s native surface type.
The [`ANativeWindow`] represents the producer endpoint of any buffer
queue, regardless of consumer endpoint.
Common consumer endpoints for `ANativeWindows` are the system window
compositor, video encoders, and application-specific compositors importing
the images through a `SurfaceTexture`.

# Registered extension number
9

# Revision
6

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_android_surface] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_android_surface extension>>)

# New base types
- [`ANativeWindow`]

# New commands
- [`create_android_surface_khr`]

# New structures
- [`AndroidSurfaceCreateInfoKHR`]

# New bitmasks
- [`AndroidSurfaceCreateFlagsKHR`]

# New constants
- [`KHR_ANDROID_SURFACE_EXTENSION_NAME`]
- [`KHR_ANDROID_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR`

# Known issues & F.A.Q.
1) Does Android need a way to query for compatibility between a particular
physical device (and queue family?) and a specific Android display? **RESOLVED** : No.
Currently on Android, any physical device is expected to be able to present
to the system compositor, and all queue families must support the necessary
image layout transitions and synchronization operations.

# Version history
- Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft. 
- Revision 2, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_android_surface to VK_KHR_android_surface. 
- Revision 3, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to surface creation function. 
- Revision 4, 2015-11-10 (Jesse Hall)  - Removed VK_ERROR_INVALID_ANDROID_WINDOW_KHR. 
- Revision 5, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a pCreateInfo structure. 
- Revision 6, 2016-01-14 (James Jones)  - Moved VK_ERROR_NATIVE_WINDOW_IN_USE_KHR from the VK_KHR_android_surface to the VK_KHR_surface extension.

# Other information
* 2016-01-14
* No known IP claims.
*   - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour, Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach, Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm  - Chia-I Wu, LunarG

# Related
- [`ANativeWindow`]
- [`AndroidSurfaceCreateFlagsKHR`]
- [`AndroidSurfaceCreateInfoKHR`]
- [`create_android_surface_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        