[VK_KHR_get_display_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_get_display_properties2.html) - instance extension

# Description
This extension provides new entry points to query device display properties
and capabilities in a way that can be easily extended by other extensions,
without introducing any further entry points.
This extension can be considered the `[`khr_display`]` equivalent of
the `[`khr_get_physical_device_properties2`]` extension.

# Registered extension number
122

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_display`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_get_display_properties2] @cubanismo%0A<<Here describe the issue or question you have about the VK_KHR_get_display_properties2 extension>>)

# New commands
- [`get_display_mode_properties2_khr`]
- [`get_display_plane_capabilities2_khr`]
- [`get_physical_device_display_plane_properties2_khr`]
- [`get_physical_device_display_properties2_khr`]

# New structures
- [`DisplayModeProperties2KHR`]
- [`DisplayPlaneCapabilities2KHR`]
- [`DisplayPlaneInfo2KHR`]
- [`DisplayPlaneProperties2KHR`]
- [`DisplayProperties2KHR`]

# New constants
- `VK_KHR_GET_DISPLAY_PROPERTIES_2_EXTENSION_NAME`
- `VK_KHR_GET_DISPLAY_PROPERTIES_2_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DISPLAY_MODE_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_CAPABILITIES_2_KHR`  - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_INFO_2_KHR`  - `VK_STRUCTURE_TYPE_DISPLAY_PLANE_PROPERTIES_2_KHR`  - `VK_STRUCTURE_TYPE_DISPLAY_PROPERTIES_2_KHR`

# Known issues & F.A.Q.
1) What should this extension be named? **RESOLVED** : [`khr_get_display_properties2`].
Other alternatives:
- `VK_KHR_display2`
- One extension, combined with `VK_KHR_surface_capabilites2`.
2) Should extensible input structs be added for these new functions: **RESOLVED** :
- [`get_physical_device_display_properties2_khr`]: No. The only current input is a [`PhysicalDevice`]. Other inputs would not make sense.
- [`get_physical_device_display_plane_properties2_khr`]: No. The only current input is a [`PhysicalDevice`]. Other inputs would not make sense.
- [`get_display_mode_properties2_khr`]: No. The only current inputs are a [`PhysicalDevice`] and a [`DisplayModeKHR`]. Other inputs would not make sense.
3) Should additional display query functions be extended? **RESOLVED** :
- [`get_display_plane_supported_displays_khr`]: No. Extensions should instead extend [`get_display_plane_capabilities_khr`]().

# Version history
- Revision 1, 2017-02-21 (James Jones)  - Initial draft.

# Other information
* 2017-02-21
* No known IP claims.
*   - Ian Elliott, Google  - James Jones, NVIDIA

# Related
- [`DisplayModeProperties2KHR`]
- [`DisplayPlaneCapabilities2KHR`]
- [`DisplayPlaneInfo2KHR`]
- [`DisplayPlaneProperties2KHR`]
- [`DisplayProperties2KHR`]
- [`get_display_mode_properties2_khr`]
- [`get_display_plane_capabilities2_khr`]
- [`get_physical_device_display_plane_properties2_khr`]
- [`get_physical_device_display_properties2_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        