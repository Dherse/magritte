[VK_EXT_shader_atomic_float2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float2.html) - device extension

# Description
This extension allows a shader to perform 16-bit floating-point atomic
operations on buffer and workgroup memory as well as floating-point atomic
minimum and maximum operations on buffer, workgroup, and image memory.
It advertises the SPIR-V `AtomicFloat16AddEXT` capability which allows
atomic add operations on 16-bit floating-point numbers and the SPIR-V
`AtomicFloat16MinMaxEXT`, `AtomicFloat32MinMaxEXT` and
`AtomicFloat64MinMaxEXT` capabilities which allow atomic minimum and
maximum operations on floating-point numbers.
The supported operations include `OpAtomicFAddEXT`, `OpAtomicFMinEXT`
and `OpAtomicFMaxEXT`.

# Registered extension number
274

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_EXT_shader_atomic_float`]`

# Contacts
- Jason Ekstrand [jekstrand](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_atomic_float2] @jekstrand%0A<<Here describe the issue or question you have about the VK_EXT_shader_atomic_float2 extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`]

# New constants
- [`EXT_SHADER_ATOMIC_FLOAT_2_EXTENSION_NAME`]
- [`EXT_SHADER_ATOMIC_FLOAT_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_2_FEATURES_EXT`

# Known issues & F.A.Q.
1) Should this extension add support for 16-bit image atomics? **RESOLVED** : No.
While Vulkan supports creating storage images with
`VK_FORMAT_R16_SFLOAT` and doing load and store on them, the data in the
shader has a 32-bit representation.
Vulkan currently has no facility for even basic reading or writing such
images using 16-bit float values in the shader.
Adding such functionality would be required before 16-bit image atomics
would make sense and is outside the scope of this extension.

# Version history
- Revision 1, 2020-08-14 (Jason Ekstrand)  - Internal revisions

# Other information
* 2020-08-14
* No known IP claims.
*   - This extension requires the VK_EXT_shader_atomic_float extension.  - This extension requires [`SPV_EXT_shader_atomic_float_min_max`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float_min_max.html) and [`SPV_EXT_shader_atomic_float16_add`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float16_add.html)  - This extension provides API support for [`GLSL_EXT_shader_atomic_float2`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_atomic_float2.txt) 
*   - Jason Ekstrand, Intel

# Related
- [`PhysicalDeviceShaderAtomicFloat2FeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        