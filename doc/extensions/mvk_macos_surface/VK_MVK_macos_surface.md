[VK_MVK_macos_surface](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_MVK_macos_surface.html) - instance extension

# Description
The [`VK_MVK_macos_surface`] extension is an instance extension.
It provides a mechanism to create a [`SurfaceKHR`] object (defined by
the `[`VK_KHR_surface`]` extension) based on an `NSView`, the native
surface type of macOS, which is underpinned by a [`CaMetalLayer`], to
support rendering to the surface using Apple’s Metal framework.

# Registered extension number
124

# Revision
3

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_surface`]`

# Deprecation state
- *Deprecated* by `[`VK_EXT_metal_surface`]` extension

# Contacts
- Bill Hollings [billhollings](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_MVK_macos_surface] @billhollings%0A<<Here describe the issue or question you have about the VK_MVK_macos_surface extension>>)

# New commands
- [`create_mac_os_surface_mvk`]

# New structures
- [`MacOsSurfaceCreateInfoMVK`]

# New bitmasks
- [`MacOsSurfaceCreateFlagsMVK`]

# New constants
- [`MVK_MACOS_SURFACE_EXTENSION_NAME`]
- [`MVK_MACOS_SURFACE_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_MACOS_SURFACE_CREATE_INFO_MVK`

# Version history
- Revision 1, 2017-02-15 (Bill Hollings)  - Initial draft. 
- Revision 2, 2017-02-24 (Bill Hollings)  - Minor syntax fix to emphasize firm requirement for `NSView` to be backed by a [`CaMetalLayer`]. 
- Revision 3, 2020-07-31 (Bill Hollings)  - Update documentation on requirements for `NSView`.  - Mark as deprecated by [`VK_EXT_metal_surface`].

# Other information
* 2020-07-31
* No known IP claims.
*   - Bill Hollings, The Brenwill Workshop Ltd.

# Related
- [`MacOsSurfaceCreateFlagsMVK`]
- [`MacOsSurfaceCreateInfoMVK`]
- [`create_mac_os_surface_mvk`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        