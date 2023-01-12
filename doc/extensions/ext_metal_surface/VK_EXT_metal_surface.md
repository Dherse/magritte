[VK_EXT_metal_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_metal_surface.html) - instance extension

# Description
The [`ext_metal_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`khr_surface`]` extension) from [`CaMetalLayer`], which is
the native rendering surface of Apple’s Metal framework.

# Registered extension number
218

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_surface`]`

# Contacts
- Dzmitry Malyshau [kvark](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_metal_surface] @kvark%0A<<Here describe the issue or question you have about the VK_EXT_metal_surface extension>>)

# New base types
- [`CaMetalLayer`]

# New commands
- [`create_metal_surface_ext`]

# New structures
- [`MetalSurfaceCreateInfoEXT`]

# New bitmasks
- [`MetalSurfaceCreateFlagsEXT`]

# New constants
- `VK_EXT_METAL_SURFACE_EXTENSION_NAME`
- `VK_EXT_METAL_SURFACE_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_METAL_SURFACE_CREATE_INFO_EXT`

# Version history
- Revision 1, 2018-10-01 (Dzmitry Malyshau)  - Initial version

# Other information
* 2018-10-01
* No known IP claims.
*   - Dzmitry Malyshau, Mozilla Corp.

# Related
- [`CaMetalLayer`]
- [`MetalSurfaceCreateFlagsEXT`]
- [`MetalSurfaceCreateInfoEXT`]
- [`create_metal_surface_ext`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        