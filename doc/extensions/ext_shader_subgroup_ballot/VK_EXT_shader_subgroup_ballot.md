[VK_EXT_shader_subgroup_ballot](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_subgroup_ballot.html) - device extension

# Description
This extension adds support for the following SPIR-V extension in Vulkan:
- `SPV_KHR_shader_ballot`
This extension provides the ability for a group of invocations, which
execute in parallel, to do limited forms of cross-invocation communication
via a group broadcast of a invocation value, or broadcast of a bitarray
representing a predicate value from each invocation in the group.This extension provides access to a number of additional built-in shader
variables in Vulkan:
- `SubgroupEqMaskKHR`, containing the subgroup mask of the current subgroup invocation,
- `SubgroupGeMaskKHR`, containing the subgroup mask of the invocations greater than or equal to the current invocation,
- `SubgroupGtMaskKHR`, containing the subgroup mask of the invocations greater than the current invocation,
- `SubgroupLeMaskKHR`, containing the subgroup mask of the invocations less than or equal to the current invocation,
- `SubgroupLtMaskKHR`, containing the subgroup mask of the invocations less than the current invocation,
- `SubgroupLocalInvocationId`, containing the index of an invocation within a subgroup, and
- `SubgroupSize`, containing the maximum number of invocations in a subgroup.
Additionally, this extension provides access to the new SPIR-V instructions:
- `OpSubgroupBallotKHR`,
- `OpSubgroupFirstInvocationKHR`, and
- `OpSubgroupReadInvocationKHR`,
When using GLSL source-based shader languages, the following variables and
shader functions from GL_ARB_shader_ballot can map to these SPIR-V built-in
decorations and instructions:
- `in uint64_t gl_SubGroupEqMaskARB;` → `SubgroupEqMaskKHR`,
- `in uint64_t gl_SubGroupGeMaskARB;` → `SubgroupGeMaskKHR`,
- `in uint64_t gl_SubGroupGtMaskARB;` → `SubgroupGtMaskKHR`,
- `in uint64_t gl_SubGroupLeMaskARB;` → `SubgroupLeMaskKHR`,
- `in uint64_t gl_SubGroupLtMaskARB;` → `SubgroupLtMaskKHR`,
- `in uint gl_SubGroupInvocationARB;` → `SubgroupLocalInvocationId`,
- `uniform uint gl_SubGroupSizeARB;` → `SubgroupSize`,
- `ballotARB`() → `OpSubgroupBallotKHR`,
- `readFirstInvocationARB`() → `OpSubgroupFirstInvocationKHR`, and
- `readInvocationARB`() → `OpSubgroupReadInvocationKHR`.

# Registered extension number
65

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Deprecated* by [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-new-features)

# Contacts
- Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_subgroup_ballot] @dgkoch%0A<<Here describe the issue or question you have about the VK_EXT_shader_subgroup_ballot extension>>)

# New constants
- `VK_EXT_SHADER_SUBGROUP_BALLOT_EXTENSION_NAME`
- `VK_EXT_SHADER_SUBGROUP_BALLOT_SPEC_VERSION`

# Version history
- Revision 1, 2016-11-28 (Daniel Koch)  - Initial draft

# Other information
* 2016-11-28
* No known IP claims.
*   - This extension requires [`SPV_KHR_shader_ballot`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_shader_ballot.html)  - This extension provides API support for [`GL_ARB_shader_ballot`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_ballot.txt) 
*   - Jeff Bolz, NVIDIA  - Neil Henning, Codeplay  - Daniel Koch, NVIDIA Corporation
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        