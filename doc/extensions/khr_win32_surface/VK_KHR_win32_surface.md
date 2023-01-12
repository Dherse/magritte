[VK_KHR_win32_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_win32_surface.html) - instance extension

# Description
The [`khr_win32_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`khr_surface`]` extension) that refers to a Win32 `HWND`, as
well as a query to determine support for rendering to the windows desktop.

# Registered extension number
10

# Revision
6

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_surface`]`

# Contacts
- Jesse Hall [critsec](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_win32_surface] @critsec%0A<<Here describe the issue or question you have about the VK_KHR_win32_surface extension>>)
- Ian Elliott [ianelliottus](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_win32_surface] @ianelliottus%0A<<Here describe the issue or question you have about the VK_KHR_win32_surface extension>>)

# New commands
- [`create_win32_surface_khr`]
- [`get_physical_device_win32_presentation_support_khr`]

# New structures
- [`Win32SurfaceCreateInfoKHR`]

# New bitmasks
- [`Win32SurfaceCreateFlagsKHR`]

# New constants
- `VK_KHR_WIN32_SURFACE_EXTENSION_NAME`
- `VK_KHR_WIN32_SURFACE_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR`

# Known issues & F.A.Q.
1) Does Win32 need a way to query for compatibility between a particular
physical device and a specific screen? Compatibility between a physical
device and a window generally only depends on what screen the window is on.
However, there is not an obvious way to identify a screen without already
having a window on the screen. **RESOLVED** : No.
While it may be useful, there is not a clear way to do this on Win32.
However, a method was added to query support for presenting to the windows
desktop as a whole.2) If a native window object (`HWND`) is used by one graphics API, and
then is later used by a different graphics API (one of which is Vulkan), can
these uses interfere with each other? **RESOLVED** : Yes.Uses of a window object by multiple graphics APIs results in undefined
behavior.
Such behavior may succeed when using one Vulkan implementation but fail when
using a different Vulkan implementation.
Potential failures include:
- Creating then destroying a flip presentation model DXGI swapchain on a window object can prevent [`create_swapchain_khr`] from succeeding on the same window object.
- Creating then destroying a [`SwapchainKHR`] on a window object can prevent creation of a bitblt model DXGI swapchain on the same window object.
- Creating then destroying a [`SwapchainKHR`] on a window object can effectively `SetPixelFormat` to a different format than the format chosen by an OpenGL application.
- Creating then destroying a [`SwapchainKHR`] on a window object on one [`PhysicalDevice`] can prevent [`create_swapchain_khr`] from succeeding on the same window object, but on a different [`PhysicalDevice`] that is associated with a different Vulkan ICD.
In all cases the problem can be worked around by creating a new window
object.Technical details include:
- Creating a DXGI swapchain over a window object can alter the object for the remainder of its lifetime. The alteration persists even after the DXGI swapchain has been destroyed. This alteration can make it impossible for a conformant Vulkan implementation to create a [`SwapchainKHR`] over the same window object. Mention of this alteration can be found in the remarks section of the MSDN documentation for `DXGI_SWAP_EFFECT`.
- Calling GDI’s `SetPixelFormat` (needed by OpenGL’s WGL layer) on a window object alters the object for the remainder of its lifetime. The MSDN documentation for `SetPixelFormat` explains that a window object’s pixel format can be set only one time.
- Creating a [`SwapchainKHR`] over a window object can alter the object for its remaining lifetime. Either of the above alterations may occur as a side effect of [`create_swapchain_khr`].

# Version history
- Revision 1, 2015-09-23 (Jesse Hall)  - Initial draft, based on the previous contents of VK_EXT_KHR_swapchain (later renamed VK_EXT_KHR_surface). 
- Revision 2, 2015-10-02 (James Jones)  - Added presentation support query for win32 desktops. 
- Revision 3, 2015-10-26 (Ian Elliott)  - Renamed from VK_EXT_KHR_win32_surface to VK_KHR_win32_surface. 
- Revision 4, 2015-11-03 (Daniel Rakos)  - Added allocation callbacks to vkCreateWin32SurfaceKHR. 
- Revision 5, 2015-11-28 (Daniel Rakos)  - Updated the surface create function to take a pCreateInfo structure. 
- Revision 6, 2017-04-24 (Jeff Juliano)  - Add issue 2 addressing reuse of a native window object in a different Graphics API, or by a different Vulkan ICD.

# Other information
* 2017-04-24
* No known IP claims.
*   - Patrick Doane, Blizzard  - Jason Ekstrand, Intel  - Ian Elliott, LunarG  - Courtney Goeltzenleuchter, LunarG  - Jesse Hall, Google  - James Jones, NVIDIA  - Antoine Labour, Google  - Jon Leech, Khronos  - David Mao, AMD  - Norbert Nopper, Freescale  - Alon Or-bach, Samsung  - Daniel Rakos, AMD  - Graham Sellers, AMD  - Ray Smith, ARM  - Jeff Vigil, Qualcomm  - Chia-I Wu, LunarG

# Related
- [`Win32SurfaceCreateFlagsKHR`]
- [`Win32SurfaceCreateInfoKHR`]
- [`create_win32_surface_khr`]
- [`get_physical_device_win32_presentation_support_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        