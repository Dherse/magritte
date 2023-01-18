[VK_KHR_push_descriptor](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_push_descriptor.html) - device extension

# Description
This extension allows descriptors to be written into the command buffer,
while the implementation is responsible for managing their memory.
Push descriptors may enable easier porting from older APIs and in some cases
can be more efficient than writing descriptors into descriptor sets.

# Registered extension number
81

# Revision
2

# Dependencies
- Requires Vulkan 1.0
- Requires `[`VK_KHR_get_physical_device_properties2`]`

# Contacts
- Jeff Bolz [jeffbolznv](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_push_descriptor] @jeffbolznv%0A<<Here describe the issue or question you have about the VK_KHR_push_descriptor extension>>)

# New commands
- [`cmd_push_descriptor_set_khr`]
If [`VK_KHR_descriptor_update_template`] is supported:
- [`cmd_push_descriptor_set_with_template_khr`]
If [Version 1.1]() is supported:
- [`cmd_push_descriptor_set_with_template_khr`]

# New structures
- Extending [`PhysicalDeviceProperties2`]:  - [`PhysicalDevicePushDescriptorPropertiesKHR`]

# New constants
- [`KHR_PUSH_DESCRIPTOR_EXTENSION_NAME`]
- [`KHR_PUSH_DESCRIPTOR_SPEC_VERSION`]
- Extending [`DescriptorSetLayoutCreateFlagBits`]:  - `VK_DESCRIPTOR_SET_LAYOUT_CREATE_PUSH_DESCRIPTOR_BIT_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES_KHR` 
If [`VK_KHR_descriptor_update_template`] is supported:
- Extending [`DescriptorUpdateTemplateType`]:  - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR` 
If [Version 1.1]() is supported:
- Extending [`DescriptorUpdateTemplateType`]:  - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`

# Version history
- Revision 1, 2016-10-15 (Jeff Bolz)  - Internal revisions 
- Revision 2, 2017-09-12 (Tobias Hector)  - Added interactions with Vulkan 1.1

# Other information
* 2017-09-12
* No known IP claims.
*   - Jeff Bolz, NVIDIA  - Michael Worcester, Imagination Technologies

# Related
- [`PhysicalDevicePushDescriptorPropertiesKHR`]
- [`cmd_push_descriptor_set_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        