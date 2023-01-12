[VK_NV_glsl_shader](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_glsl_shader.html) - device extension

# Description
This extension allows GLSL shaders written to the `GL_KHR_vulkan_glsl`
extension specification to be used instead of SPIR-V.
The implementation will automatically detect whether the shader is SPIR-V or
GLSL, and compile it appropriately.

# Registered extension number
13

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Deprecated* without replacement

# Contacts
- Piers Daniell [pdaniell-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_glsl_shader] @pdaniell-nv%0A<<Here describe the issue or question you have about the VK_NV_glsl_shader extension>>)

# New constants
- `VK_NV_GLSL_SHADER_EXTENSION_NAME`
- `VK_NV_GLSL_SHADER_SPEC_VERSION`
- Extending [`VulkanResultCodes`]:  - `VK_ERROR_INVALID_SHADER_NV`

# Version history
- Revision 1, 2016-02-14 (Piers Daniell)  - Initial draft

# Other information
* 2016-02-14
* No known IP claims.
*   - Piers Daniell, NVIDIA
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        