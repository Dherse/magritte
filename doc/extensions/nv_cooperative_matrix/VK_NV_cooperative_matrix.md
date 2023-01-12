[VK_NV_cooperative_matrix](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_NV_cooperative_matrix.html) - device extension

# Description
This extension adds support for using cooperative matrix types in SPIR-V.
Cooperative matrix types are medium-sized matrices that are primarily
supported in compute shaders, where the storage for the matrix is spread
across all invocations in some scope (usually a subgroup) and those
invocations cooperate to efficiently perform matrix multiplies.Cooperative matrix types are defined by the
[`SPV_NV_cooperative_matrix`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_cooperative_matrix.html)
SPIR-V extension and can be used with the
[`GL_NV_cooperative_matrix`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_cooperative_matrix.txt)
GLSL extension.This extension includes support for enumerating the matrix types and
dimensions that are supported by the implementation.

# Registered extension number
250

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_NV_cooperative_matrix] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_NV_cooperative_matrix extension>>)

# New commands
- [`get_physical_device_cooperative_matrix_properties_nv`]

# New structures
- [`CooperativeMatrixPropertiesNV`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceCooperativeMatrixFeaturesNV`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceCooperativeMatrixPropertiesNV`]

# New enums
- [`ComponentTypeNV`]
- [`ScopeNV`]

# New constants
- `VK_NV_COOPERATIVE_MATRIX_EXTENSION_NAME`
- `VK_NV_COOPERATIVE_MATRIX_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_COOPERATIVE_MATRIX_PROPERTIES_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_FEATURES_NV`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COOPERATIVE_MATRIX_PROPERTIES_NV`

# Known issues & F.A.Q.
(1) What matrix properties will be supported in practice? **RESOLVED** : In NVIDIA’s initial implementation, we will support:
- AType = BType = fp16 CType = DType = fp16 MxNxK = 16x8x16 scope = Subgroup
- AType = BType = fp16 CType = DType = fp16 MxNxK = 16x8x8 scope = Subgroup
- AType = BType = fp16 CType = DType = fp32 MxNxK = 16x8x16 scope = Subgroup
- AType = BType = fp16 CType = DType = fp32 MxNxK = 16x8x8 scope = Subgroup

# Version history
- Revision 1, 2019-02-05 (Jeff Bolz)  - Internal revisions

# Other information
* 2019-02-05
*   - This extension requires [`SPV_NV_cooperative_matrix`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/NV/SPV_NV_cooperative_matrix.html)  - This extension provides API support for [`GL_NV_cooperative_matrix`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/nv/GLSL_NV_cooperative_matrix.txt) 
*   - Jeff Bolz, NVIDIA  - Markus Tavenrath, NVIDIA  - Daniel Koch, NVIDIA

# Related
- [`ComponentTypeNV`]
- [`CooperativeMatrixPropertiesNV`]
- [`PhysicalDeviceCooperativeMatrixFeaturesNV`]
- [`PhysicalDeviceCooperativeMatrixPropertiesNV`]
- [`ScopeNV`]
- [`get_physical_device_cooperative_matrix_properties_nv`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        