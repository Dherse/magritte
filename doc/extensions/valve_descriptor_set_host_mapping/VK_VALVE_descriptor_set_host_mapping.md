[VK_VALVE_descriptor_set_host_mapping](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_VALVE_descriptor_set_host_mapping.html) - device extension

# Description
This extension allows applications to directly query a host pointer for a
[`DescriptorSet`] which  **can**  be used to copy descriptors between
descriptor sets without the use of an API command.
Memory offsets and sizes for descriptors  **can**  be queried from a
[`DescriptorSetLayout`] as well.

# Registered extension number
421

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Contacts
- Hans-Kristian Arntzen [HansKristian-Work](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_VALVE_descriptor_set_host_mapping] @HansKristian-Work%0A<<Here describe the issue or question you have about the VK_VALVE_descriptor_set_host_mapping extension>>)

# New commands
- [`get_descriptor_set_host_mapping_valve`]
- [`get_descriptor_set_layout_host_mapping_info_valve`]

# New structures
- [`DescriptorSetBindingReferenceVALVE`]
- [`DescriptorSetLayoutHostMappingInfoVALVE`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE`]

# New constants
- [`VALVE_DESCRIPTOR_SET_HOST_MAPPING_EXTENSION_NAME`]
- [`VALVE_DESCRIPTOR_SET_HOST_MAPPING_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_BINDING_REFERENCE_VALVE`  - `VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_HOST_MAPPING_INFO_VALVE`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_SET_HOST_MAPPING_FEATURES_VALVE`

# Version history
- Revision 1, 2022-02-22 (Hans-Kristian Arntzen)  - Initial specification

# Other information
* 2022-02-22
* No known IP claims.
*   - Hans-Kristian Arntzen, Valve

# Related
- [`DescriptorSetBindingReferenceVALVE`]
- [`DescriptorSetLayoutHostMappingInfoVALVE`]
- [`PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE`]
- [`get_descriptor_set_host_mapping_valve`]
- [`get_descriptor_set_layout_host_mapping_info_valve`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        