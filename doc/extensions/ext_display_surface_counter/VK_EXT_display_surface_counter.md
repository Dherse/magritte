[VK_EXT_display_surface_counter](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_display_surface_counter.html) - instance extension

# Description
This extension defines a vertical blanking period counter associated with
display surfaces.
It provides a mechanism to query support for such a counter from a
[`SurfaceKHR`] object.

# Registered extension number
91

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_display`]`

# Contacts
- James Jones [cubanismo](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_display_surface_counter] @cubanismo%0A<<Here describe the issue or question you have about the VK_EXT_display_surface_counter extension>>)

# New commands
- [`get_physical_device_surface_capabilities2_ext`]

# New structures
- [`SurfaceCapabilities2EXT`]

# New enums
- [`SurfaceCounterFlagBitsEXT`]

# New bitmasks
- [`SurfaceCounterFlagsEXT`]

# New constants
- [`EXT_DISPLAY_SURFACE_COUNTER_EXTENSION_NAME`]
- [`EXT_DISPLAY_SURFACE_COUNTER_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES2_EXT`  - `VK_STRUCTURE_TYPE_SURFACE_CAPABILITIES_2_EXT`

# Version history
- Revision 1, 2016-12-13 (James Jones)  - Initial draft

# Other information
* 2016-12-13
* No known IP claims.
*   - Pierre Boudier, NVIDIA  - James Jones, NVIDIA  - Damien Leone, NVIDIA  - Pierre-Loup Griffais, Valve  - Daniel Vetter, Intel

# Related
- [`SurfaceCapabilities2EXT`]
- [`SurfaceCounterFlagBitsEXT`]
- [`SurfaceCounterFlagsEXT`]
- [`get_physical_device_surface_capabilities2_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        