[VK_EXT_vertex_attribute_divisor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_vertex_attribute_divisor.html) - device extension

# Description
This extension allows instance-rate vertex attributes to be repeated for
certain number of instances instead of advancing for every instance when
instanced rendering is enabled.

# Registered extension number
191

# Revision
3

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Vikram Kushwaha [vkushwaha](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_vertex_attribute_divisor] @vkushwaha%0A<<Here describe the issue or question you have about the VK_EXT_vertex_attribute_divisor extension>>)

# New structures
- [`VertexInputBindingDivisorDescriptionEXT`]
- Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:  - [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`] 
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`] 
- Extending [`PipelineVertexInputStateCreateInfo`]:  - [`PipelineVertexInputDivisorStateCreateInfoEXT`]

# New constants
- [`EXT_VERTEX_ATTRIBUTE_DIVISOR_EXTENSION_NAME`]
- [`EXT_VERTEX_ATTRIBUTE_DIVISOR_SPEC_VERSION`]
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES_EXT`  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES_EXT`  - `VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO_EXT`

# Known issues & F.A.Q.
1) What is the effect of a non-zero value for `firstInstance`? **RESOLVED** : The Vulkan API should follow the OpenGL convention and offset
attribute fetching by `firstInstance` while computing vertex attribute
offsets.2) Should zero be an allowed divisor? **RESOLVED** : Yes.
A zero divisor means the vertex attribute is repeated for all instances.

# Version history
- Revision 1, 2017-12-04 (Vikram Kushwaha)  - First Version 
- Revision 2, 2018-07-16 (Jason Ekstrand)  - Adjust the interaction between `divisor` and `firstInstance` to match the OpenGL convention.  - Disallow divisors of zero. 
- Revision 3, 2018-08-03 (Vikram Kushwaha)  - Allow a zero divisor.  - Add a physical device features structure to query/enable this feature.

# Other information
* 2018-08-03
* No known IP claims.
*   - Vikram Kushwaha, NVIDIA  - Jason Ekstrand, Intel

# Related
- [`PhysicalDeviceVertexAttributeDivisorFeaturesEXT`]
- [`PhysicalDeviceVertexAttributeDivisorPropertiesEXT`]
- [`PipelineVertexInputDivisorStateCreateInfoEXT`]
- [`VertexInputBindingDivisorDescriptionEXT`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        