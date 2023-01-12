[VK_EXT_shader_subgroup_vote](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_subgroup_vote.html) - device extension

# Description
This extension adds support for the following SPIR-V extension in Vulkan:
- `SPV_KHR_subgroup_vote`
This extension provides new SPIR-V instructions:
- `OpSubgroupAllKHR`,
- `OpSubgroupAnyKHR`, and
- `OpSubgroupAllEqualKHR`.
to compute the composite of a set of boolean conditions across a group of
shader invocations that are running concurrently (a *subgroup*).
These composite results may be used to execute shaders more efficiently on a
[`PhysicalDevice`].When using GLSL source-based shader languages, the following shader
functions from GL_ARB_shader_group_vote can map to these SPIR-V
instructions:
- `anyInvocationARB`() → `OpSubgroupAnyKHR`,
- `allInvocationsARB`() → `OpSubgroupAllKHR`, and
- `allInvocationsEqualARB`() → `OpSubgroupAllEqualKHR`.
The subgroup across which the boolean conditions are evaluated is
implementation-dependent, and this extension provides no guarantee over how
individual shader invocations are assigned to subgroups.
In particular, a subgroup has no necessary relationship with the compute
shader *local workgroup* — any pair of shader invocations in a compute
local workgroup may execute in different subgroups as used by these
instructions.Compute shaders operate on an explicitly specified group of threads (a local
workgroup), but many implementations will also group non-compute shader
invocations and execute them concurrently.
When executing code like
```c
if (condition) {
  result = do_fast_path();
} else {
  result = do_general_path();
}
```
where `condition` diverges between invocations, an implementation might
first execute `do_fast_path`() for the invocations where `condition`
is true and leave the other invocations dormant.
Once `do_fast_path`() returns, it might call `do_general_path`() for
invocations where `condition` is `false` and leave the other
invocations dormant.
In this case, the shader executes  **both**  the fast and the general path and
might be better off just using the general path for all invocations.This extension provides the ability to avoid divergent execution by
evaluating a condition across an entire subgroup using code like:
```c
if (allInvocationsARB(condition)) {
  result = do_fast_path();
} else {
  result = do_general_path();
}
```
The built-in function `allInvocationsARB`() will return the same value
for all invocations in the group, so the group will either execute
`do_fast_path`() or `do_general_path`(), but never both.
For example, shader code might want to evaluate a complex function
iteratively by starting with an approximation of the result and then
refining the approximation.
Some input values may require a small number of iterations to generate an
accurate result (`do_fast_path`) while others require a larger number
(`do_general_path`).
In another example, shader code might want to evaluate a complex function
(`do_general_path`) that can be greatly simplified when assuming a
specific value for one of its inputs (`do_fast_path`).

# Registered extension number
66

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Deprecated* by [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-new-features)

# Contacts
- Daniel Koch [dgkoch](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_subgroup_vote] @dgkoch%0A<<Here describe the issue or question you have about the VK_EXT_shader_subgroup_vote extension>>)

# New constants
- `VK_EXT_SHADER_SUBGROUP_VOTE_EXTENSION_NAME`
- `VK_EXT_SHADER_SUBGROUP_VOTE_SPEC_VERSION`

# Version history
- Revision 1, 2016-11-28 (Daniel Koch)  - Initial draft

# Other information
* 2016-11-28
* No known IP claims.
*   - This extension requires [`SPV_KHR_subgroup_vote`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/KHR/SPV_KHR_subgroup_vote.html)  - This extension provides API support for [`GL_ARB_shader_group_vote`](https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_shader_group_vote.txt) 
*   - Neil Henning, Codeplay  - Daniel Koch, NVIDIA Corporation
# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        