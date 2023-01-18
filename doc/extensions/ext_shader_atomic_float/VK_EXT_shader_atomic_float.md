[VK_EXT_shader_atomic_float](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_shader_atomic_float.html) - device extension

# Description
This extension allows a shader to contain floating-point atomic operations
on buffer, workgroup, and image memory.
It also advertises the SPIR-V `AtomicFloat32AddEXT` and
`AtomicFloat64AddEXT` capabilities that allows atomic addition on
floating-points numbers.
The supported operations include `OpAtomicFAddEXT`,
`OpAtomicExchange`, `OpAtomicLoad` and `OpAtomicStore`.

# Registered extension number
261

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Vikram Kushwaha [vkushwaha-nv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_shader_atomic_float] @vkushwaha-nv%0A<<Here describe the issue or question you have about the VK_EXT_shader_atomic_float extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]

# New constants
- [`EXT_SHADER_ATOMIC_FLOAT_EXTENSION_NAME`]
- [`EXT_SHADER_ATOMIC_FLOAT_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_FLOAT_FEATURES_EXT`

# Version history
- Revision 1, 2020-07-15 (Vikram Kushwaha)  - Internal revisions

# Other information
* 2020-07-15
* No known IP claims.
*   - This extension requires [`SPV_EXT_shader_atomic_float_add`](https://htmlpreview.github.io/?https://github.com/KhronosGroup/SPIRV-Registry/blob/master/extensions/EXT/SPV_EXT_shader_atomic_float_add.html)  - This extension provides API support for [`GL_EXT_shader_atomic_float`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_atomic_float.txt) 
*   - Vikram Kushwaha, NVIDIA  - Jeff Bolz, NVIDIA

# Related
- [`PhysicalDeviceShaderAtomicFloatFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        