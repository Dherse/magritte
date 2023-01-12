[VK_KHR_shader_draw_parameters](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_draw_parameters.html) - device extension

# Description
This extension adds support for the following SPIR-V extension in Vulkan:
- `SPV_KHR_shader_draw_parameters`
The extension provides access to three additional built-in shader variables
in Vulkan:
- `BaseInstance`, containing the `firstInstance` parameter passed to drawing commands,
- `BaseVertex`, containing the `firstVertex` or `vertexOffset` parameter passed to drawing commands, and
- `DrawIndex`, containing the index of the draw call currently being processed from an indirect drawing call.
When using GLSL source-based shader languages, the following variables from
`GL_ARB_shader_draw_parameters` can map to these SPIR-V built-in
decorations:
- `in int gl_BaseInstanceARB;` → `BaseInstance`,
- `in int gl_BaseVertexARB;` → `BaseVertex`, and
- `in int gl_DrawIDARB;` → `DrawIndex`.

# Registered extension number
64

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_draw_parameters] @dgkoch%0A<<Here describe the issue or question you have about the VK_KHR_shader_draw_parameters extension>>)

# New constants
- `VK_KHR_SHADER_DRAW_PARAMETERS_EXTENSION_NAME`
- `VK_KHR_SHADER_DRAW_PARAMETERS_SPEC_VERSION`

# Known issues & F.A.Q.
1) Is this the same functionality as `GL_ARB_shader_draw_parameters`? **RESOLVED** : It is actually a superset, as it also adds in support for
arrayed drawing commands.In GL for `GL_ARB_shader_draw_parameters`, `gl_BaseVertexARB` holds the
integer value passed to the parameter to the command that resulted in the
current shader invocation.
In the case where the command has no `baseVertex` parameter, the value of
`gl_BaseVertexARB` is zero.
This means that `gl_BaseVertexARB` = `baseVertex` (for
`glDrawElements` commands with `baseVertex`) or 0.
In particular there are no `glDrawArrays` commands that take a
`baseVertex` parameter.Now in Vulkan, we have `BaseVertex` = `vertexOffset` (for indexed
drawing commands) or `firstVertex` (for arrayed drawing commands), and
so Vulkan’s version is really a superset of GL functionality.

# Version history
- Revision 1, 2016-10-05 (Daniel Koch)  - Internal revisions

# Other information
* 2017-09-05
* No known IP claims.
*   - This extension requires [`SPV_KHR_shader_draw_parameters`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_shader_draw_parameters.html)  - This extension provides API support for [`GL_ARB_shader_draw_parameters`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_draw_parameters.txt)  - Promoted to Vulkan 1.1 Core 
*   - Daniel Koch, NVIDIA Corporation  - Jeff Bolz, NVIDIA  - Daniel Rakos, AMD  - Jan-Harald Fredriksen, ARM  - John Kessenich, Google  - Stuart Smith, IMG
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        