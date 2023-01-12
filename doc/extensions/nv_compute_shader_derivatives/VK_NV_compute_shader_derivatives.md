[VK_NV_compute_shader_derivatives](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_compute_shader_derivatives.html) - device extension

# Description
This extension adds Vulkan support for the
[`SPV_NV_compute_shader_derivatives`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_compute_shader_derivatives.html)
SPIR-V extension.The SPIR-V extension provides two new execution modes, both of which allow
compute shaders to use built-ins that evaluate compute derivatives
explicitly or implicitly.
Derivatives will be computed via differencing over a 2x2 group of shader
invocations.
The `DerivativeGroupQuadsNV` execution mode assembles shader invocations
into 2x2 groups, where each group has x and y coordinates of the local
invocation ID of the form (2m+{0,1}, 2n+{0,1}).
The `DerivativeGroupLinearNV` execution mode assembles shader invocations
into 2x2 groups, where each group has local invocation index values of the
form 4m+{0,1,2,3}.

# Registered extension number
202

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Pat Brown [nvpbrown](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_compute_shader_derivatives] @nvpbrown%0A<<Here describe the issue or question you have about the VK_NV_compute_shader_derivatives extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]

# New constants
- `VK_NV_COMPUTE_SHADER_DERIVATIVES_EXTENSION_NAME`
- `VK_NV_COMPUTE_SHADER_DERIVATIVES_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_NV`

# Known issues & F.A.Q.
(1) Should we specify that the groups of four shader invocations used for
derivatives in a compute shader are the same groups of four invocations that
form a “quad” in shader subgroups? **RESOLVED** : Yes.

# Version history
- Revision 1, 2018-07-19 (Pat Brown)  - Initial draft

# Other information
* 2018-07-19
* No known IP claims.
*   - This extension requires [`SPV_NV_compute_shader_derivatives`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_compute_shader_derivatives.html)  - This extension provides API support for [`GL_NV_compute_shader_derivatives`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_compute_shader_derivatives.txt) 
*   - Pat Brown, NVIDIA

# Related
- [`PhysicalDeviceComputeShaderDerivativesFeaturesNV`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        