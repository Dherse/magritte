[VK_AMD_display_native_hdr](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_display_native_hdr.html) - device extension

# Description
This extension introduces the following display native HDR features to
Vulkan:
- A new [`ColorSpaceKHR`] enum for setting the native display colorspace. For example, this color space would be set by the swapchain to use the native color space in Freesync2 displays.
- Local dimming control

# Registered extension number
214

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`
- Requires `[`khr_get_surface_capabilities2`]`
- Requires `[`khr_swapchain`]`

# Contacts
- Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_display_native_hdr] @anteru%0A<<Here describe the issue or question you have about the VK_AMD_display_native_hdr extension>>)

# New commands
- [`set_local_dimming_amd`]

# New structures
- Extending [`SurfaceCapabilities2KHR`]:  - [`DisplayNativeHdrSurfaceCapabilitiesAMD`] 
- Extending [`SwapchainCreateInfoKHR`]:  - [`SwapchainDisplayNativeHdrCreateInfoAMD`]

# New constants
- `VK_AMD_DISPLAY_NATIVE_HDR_EXTENSION_NAME`
- `VK_AMD_DISPLAY_NATIVE_HDR_SPEC_VERSION`
- Extending [`ColorSpaceKHR`]:  - `VK_COLOR_SPACE_DISPLAY_NATIVE_AMD` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DISPLAY_NATIVE_HDR_SURFACE_CAPABILITIES_AMD`  - `VK_STRUCTURE_TYPE_SWAPCHAIN_DISPLAY_NATIVE_HDR_CREATE_INFO_AMD`

# Known issues & F.A.Q.
None.

# Version history
- Revision 1, 2018-12-18 (Daniel Rakos)  - Initial revision

# Other information
* 2018-12-18
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Aaron Hagan, AMD  - Aric Cyr, AMD  - Timothy Lottes, AMD  - Derrick Owens, AMD  - Daniel Rakos, AMD

# Related
- [`DisplayNativeHdrSurfaceCapabilitiesAMD`]
- [`SwapchainDisplayNativeHdrCreateInfoAMD`]
- [`set_local_dimming_amd`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        