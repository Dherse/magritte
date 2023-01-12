[VK_KHR_surface_protected_capabilities](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_surface_protected_capabilities.html) - instance extension

# Description
This extension extends [`SurfaceCapabilities2KHR`], providing
applications a way to query whether swapchains  **can**  be created with the
`VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR` flag set.Vulkan 1.1 added (optional) support for protect memory and protected
resources including buffers (`VK_BUFFER_CREATE_PROTECTED_BIT`), images
(`VK_IMAGE_CREATE_PROTECTED_BIT`), and swapchains
(`VK_SWAPCHAIN_CREATE_PROTECTED_BIT_KHR`).
However, on implementations which support multiple windowing systems, not
all window systems  **may**  be able to provide a protected display path.This extension provides a way to query if a protected swapchain created for
a surface (and thus a specific windowing system)  **can**  be displayed on
screen.
It extends the existing [`SurfaceCapabilities2KHR`] structure with a new
[`SurfaceProtectedCapabilitiesKHR`] structure from which the application
 **can**  obtain information about support for protected swapchain creation
through [`get_physical_device_surface_capabilities2_khr`].

# Registered extension number
240

# Revision
1

# Dependencies
- Requires Vulkan 1.1
- Requires `[`khr_get_surface_capabilities2`]`

# Contacts
- Sandeep Shinde [sashinde](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_surface_protected_capabilities] @sashinde%0A<<Here describe the issue or question you have about the VK_KHR_surface_protected_capabilities extension>>)

# New structures
- Extending [`SurfaceCapabilities2KHR`]:  - [`SurfaceProtectedCapabilitiesKHR`]

# New constants
- `VK_KHR_SURFACE_PROTECTED_CAPABILITIES_EXTENSION_NAME`
- `VK_KHR_SURFACE_PROTECTED_CAPABILITIES_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_PROTECTED_CAPABILITIES_KHR`

# Version history
- Revision 1, 2018-12-18 (Sandeep Shinde, Daniel Koch)  - Internal revisions.

# Other information
* 2018-12-18
* No known IP claims.
*   - Sandeep Shinde, NVIDIA  - James Jones, NVIDIA  - Daniel Koch, NVIDIA

# Related
- [`SurfaceProtectedCapabilitiesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        