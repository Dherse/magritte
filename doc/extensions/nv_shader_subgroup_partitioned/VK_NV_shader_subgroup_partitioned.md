[VK_NV_shader_subgroup_partitioned](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_shader_subgroup_partitioned.html) - device extension

# Description
This extension enables support for a new class of
[group operations](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-group-operations) on [subgroups](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-scope-subgroup) via the
[`GL_NV_shader_subgroup_partitioned`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GL_NV_shader_subgroup_partitioned.txt)
GLSL extension and
[`SPV_NV_shader_subgroup_partitioned`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_subgroup_partitioned.html)
SPIR-V extension.
Support for these new operations is advertised via the
`VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV` bit.This extension requires Vulkan 1.1, for general subgroup support.

# Registered extension number
199

# Revision
1

# Dependencies
- Requires Vulkan 1.1

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_shader_subgroup_partitioned] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_NV_shader_subgroup_partitioned extension>>)

# New constants
- [`NV_SHADER_SUBGROUP_PARTITIONED_EXTENSION_NAME`]
- [`NV_SHADER_SUBGROUP_PARTITIONED_SPEC_VERSION`]
- Extending [`SubgroupFeatureFlagBits`]:  - `VK_SUBGROUP_FEATURE_PARTITIONED_BIT_NV`

# Version history
- Revision 1, 2018-03-17 (Jeff Bolz)  - Internal revisions

# Other information
* 2018-03-17
*   - This extension requires [`SPV_NV_shader_subgroup_partitioned`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_shader_subgroup_partitioned.html)  - This extension provides API support for [`GL_NV_shader_subgroup_partitioned`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GL_NV_shader_subgroup_partitioned.txt) 
*   - Jeff Bolz, NVIDIA
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        