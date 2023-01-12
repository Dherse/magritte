[VK_AMD_negative_viewport_height](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_negative_viewport_height.html) - device extension

# Description
This extension allows an application to specify a negative viewport height.
The result is that the viewport transformation will flip along the y-axis.
- Allow negative height to be specified in the [`Viewport::height`] field to perform y-inversion of the clip-space to framebuffer-space transform. This allows apps to avoid having to use `gl_Position.y = -gl_Position.y` in shaders also targeting other APIs.

# Registered extension number
36

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Obsoleted* by `[`khr_maintenance1`]` extension  - Which in turn was *promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_negative_viewport_height] @anteru%0A<<Here describe the issue or question you have about the VK_AMD_negative_viewport_height extension>>)

# New constants
- `VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_EXTENSION_NAME`
- `VK_AMD_NEGATIVE_VIEWPORT_HEIGHT_SPEC_VERSION`

# Version history
- Revision 1, 2016-09-02 (Matthaeus Chajdas)  - Initial draft

# Other information
* 2016-09-02
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Graham Sellers, AMD  - Baldur Karlsson
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        