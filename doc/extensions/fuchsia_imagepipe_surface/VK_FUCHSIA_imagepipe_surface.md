[VK_FUCHSIA_imagepipe_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_FUCHSIA_imagepipe_surface.html) - instance extension

# Description
The [`VK_FUCHSIA_imagepipe_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`VK_KHR_surface`]` extension) that refers to a Fuchsia
`imagePipeHandle`.

# Registered extension number
215

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Contacts
- Craig Stout [cdotstout](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_FUCHSIA_imagepipe_surface] @cdotstout%0A<<Here describe the issue or question you have about the VK_FUCHSIA_imagepipe_surface extension>>)

# New commands
- [`create_image_pipe_surface_fuchsia`]

# New structures
- [`ImagePipeSurfaceCreateInfoFUCHSIA`]

# New bitmasks
- [`ImagePipeSurfaceCreateFlagsFUCHSIA`]

# New constants
- [`FUCHSIA_IMAGEPIPE_SURFACE_EXTENSION_NAME`]
- [`FUCHSIA_IMAGEPIPE_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_IMAGEPIPE_SURFACE_CREATE_INFO_FUCHSIA`

# Version history
- Revision 1, 2018-07-27 (Craig Stout)  - Initial draft.

# Other information
* 2018-07-27
* No known IP claims.
*   - Craig Stout, Google  - Ian Elliott, Google  - Jesse Hall, Google

# Related
- [`ImagePipeSurfaceCreateFlagsFUCHSIA`]
- [`ImagePipeSurfaceCreateInfoFUCHSIA`]
- [`create_image_pipe_surface_fuchsia`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        