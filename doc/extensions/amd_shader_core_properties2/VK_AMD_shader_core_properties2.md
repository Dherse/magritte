[VK_AMD_shader_core_properties2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties2.html) - device extension

# Description
This extension exposes additional shader core properties for a target
physical device through the `[`VK_KHR_get_physical_device_properties2`]`
extension.

# Registered extension number
228

# Revision
1

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_AMD_shader_core_properties`]`

# Contacts
- Matthaeus G. Chajdas [anteru](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_core_properties2] @anteru%0A<<Here describe the issue or question you have about the VK_AMD_shader_core_properties2 extension>>)

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShaderCoreProperties2AMD`]

# New enums
- [`ShaderCorePropertiesFlagBitsAMD`]

# New bitmasks
- [`ShaderCorePropertiesFlagsAMD`]

# New constants
- [`AMD_SHADER_CORE_PROPERTIES_2_EXTENSION_NAME`]
- [`AMD_SHADER_CORE_PROPERTIES_2_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_2_AMD`

# Version history
- Revision 1, 2019-07-26 (Matthaeus G. Chajdas)  - Initial draft.

# Other information
* 2019-07-26
* No known IP claims.
*   - Matthaeus G. Chajdas, AMD  - Tobias Hector, AMD

# Related
- [`PhysicalDeviceShaderCoreProperties2AMD`]
- [`ShaderCorePropertiesFlagBitsAMD`]
- [`ShaderCorePropertiesFlagsAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        