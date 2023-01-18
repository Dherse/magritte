[VK_KHR_shader_subgroup_extended_types](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_shader_subgroup_extended_types.html) - device extension

# Description
This extension enables the Non Uniform Group Operations in SPIR-V to support
8-bit integer, 16-bit integer, 64-bit integer, 16-bit floating-point, and
vectors of these types.

# Registered extension number
176

# Revision
1

# Dependencies
- Requires Vulkan 1.1

# Deprecation state
- *Promoted* to [Vulkan 1.2](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.2-promotions)

# Contacts
- Neil Henning [sheredom](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_shader_subgroup_extended_types] @sheredom%0A<<Here describe the issue or question you have about the VK_KHR_shader_subgroup_extended_types extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR`]

# New constants
- [`KHR_SHADER_SUBGROUP_EXTENDED_TYPES_EXTENSION_NAME`]
- [`KHR_SHADER_SUBGROUP_EXTENDED_TYPES_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES_KHR`

# Version history
- Revision 1, 2019-01-08 (Neil Henning)  - Initial draft

# Other information
* 2019-01-08
* No known IP claims.
*   - Promoted to Vulkan 1.2 Core  - This extension provides API support for [`GLSL_EXT_shader_subgroup_extended_types`](https://github.com/KhronosGroup/GLSL/blob/master/extensions/ext/GLSL_EXT_shader_subgroup_extended_types.txt) 
*   - Jeff Bolz, NVIDIA  - Jan-Harald Fredriksen, Arm  - Neil Henning, AMD  - Daniel Koch, NVIDIA  - Jeff Leger, Qualcomm  - Graeme Leese, Broadcom  - David Neto, Google  - Daniel Rakos, AMD

# Related
- [`PhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        