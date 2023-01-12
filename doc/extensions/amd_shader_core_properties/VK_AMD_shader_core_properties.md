[VK_AMD_shader_core_properties](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_AMD_shader_core_properties.html) - device extension

# Description
This extension exposes shader core properties for a target physical device
through the `[`khr_get_physical_device_properties2`]` extension.
Please refer to the example below for proper usage.

# Registered extension number
186

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`khr_get_physical_device_properties2`]`

# Contacts
- Martin Dinkov [mdinkov](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_AMD_shader_core_properties] @mdinkov%0A<<Here describe the issue or question you have about the VK_AMD_shader_core_properties extension>>)

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceShaderCorePropertiesAMD`]

# New constants
- `VK_AMD_SHADER_CORE_PROPERTIES_EXTENSION_NAME`
- `VK_AMD_SHADER_CORE_PROPERTIES_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_CORE_PROPERTIES_AMD`

# Version history
- Revision 2, 2019-06-25 (Matthaeus G. Chajdas)  - Clarified the meaning of a few fields. 
- Revision 1, 2018-02-15 (Martin Dinkov)  - Initial draft.

# Other information
* 2019-06-25
* No known IP claims.
*   - Martin Dinkov, AMD  - Matthaeus G. Chajdas, AMD

# Related
- [`PhysicalDeviceShaderCorePropertiesAMD`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        