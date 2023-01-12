[VK_EXT_primitive_topology_list_restart](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_primitive_topology_list_restart.html) - device extension

# Description
This extension allows list primitives to use the primitive restart index
value.
This provides a more efficient implementation when layering OpenGL
functionality on Vulkan by avoiding emulation which incurs data copies.

# Registered extension number
357

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Shahbaz Youssefi [syoussefi](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_primitive_topology_list_restart] @syoussefi%0A<<Here describe the issue or question you have about the VK_EXT_primitive_topology_list_restart extension>>)

# New structures
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]

# New constants
- `VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_EXTENSION_NAME`
- `VK_EXT_PRIMITIVE_TOPOLOGY_LIST_RESTART_SPEC_VERSION`
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIMITIVE_TOPOLOGY_LIST_RESTART_FEATURES_EXT`

# Version history
- Revision 0, 2020-09-14 (Courtney Goeltzenleuchter)  - Internal revisions 
- Revision 1, 2021-01-11 (Shahbaz Youssefi)  - Add the `primitiveTopologyPatchListRestart` feature  - Internal revisions

# Other information
* 2021-01-11
* No known IP claims.
*   - Courtney Goeltzenleuchter, Google  - Shahbaz Youssefi, Google

# Related
- [`PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        