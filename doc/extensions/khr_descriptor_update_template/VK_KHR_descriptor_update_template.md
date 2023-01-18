[VK_KHR_descriptor_update_template](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_descriptor_update_template.html) - device extension

# Description
Applications may wish to update a fixed set of descriptors in a large number
of descriptor sets very frequently, i.e. during initializaton phase or if it
is required to rebuild descriptor sets for each frame.
For those cases it is also not unlikely that all information required to
update a single descriptor set is stored in a single struct.
This extension provides a way to update a fixed set of descriptors in a
single [`DescriptorSet`] with a pointer to a user defined data structure
describing the new descriptors.

# Registered extension number
86

# Revision
1

# Dependencies
- Requires Vulkan 1.0

# Deprecation state
- *Promoted* to [Vulkan 1.1](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#versions-1.1-promotions)

# Contacts
- Markus Tavenrath [mtavenrath](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_descriptor_update_template] @mtavenrath%0A<<Here describe the issue or question you have about the VK_KHR_descriptor_update_template extension>>)

# New object types
- [`DescriptorUpdateTemplateKHR`]

# New commands
- [`create_descriptor_update_template_khr`]
- [`destroy_descriptor_update_template_khr`]
- [`update_descriptor_set_with_template_khr`]
If [`VK_KHR_push_descriptor`] is supported:
- [`cmd_push_descriptor_set_with_template_khr`]

# New structures
- [`DescriptorUpdateTemplateCreateInfoKHR`]
- [`DescriptorUpdateTemplateEntryKHR`]

# New enums
- [`DescriptorUpdateTemplateTypeKHR`]

# New bitmasks
- [`DescriptorUpdateTemplateCreateFlagsKHR`]

# New constants
- [`KHR_DESCRIPTOR_UPDATE_TEMPLATE_EXTENSION_NAME`]
- [`KHR_DESCRIPTOR_UPDATE_TEMPLATE_SPEC_VERSION`]
- Extending [`DescriptorUpdateTemplateType`]:  - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR` 
- Extending [`ObjectType`]:  - `VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR` 
- Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO_KHR` 
If [`VK_EXT_debug_report`] is supported:
- Extending [`DebugReportObjectTypeEXT`]:  - `VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT` 
If [`VK_KHR_push_descriptor`] is supported:
- Extending [`DescriptorUpdateTemplateType`]:  - `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`

# Version history
- Revision 1, 2016-01-11 (Markus Tavenrath)  - Initial draft

# Other information
* 2017-09-05
* No known IP claims.
*   - Interacts with `[`VK_KHR_push_descriptor`]`  - Promoted to Vulkan 1.1 Core 
*   - Jeff Bolz, NVIDIA  - Michael Worcester, Imagination Technologies

# Related
- [`DescriptorUpdateTemplateCreateFlagsKHR`]
- [`DescriptorUpdateTemplateCreateInfoKHR`]
- [`DescriptorUpdateTemplateEntryKHR`]
- [`DescriptorUpdateTemplateKHR`]
- [`DescriptorUpdateTemplateTypeKHR`]
- [`create_descriptor_update_template_khr`]
- [`destroy_descriptor_update_template_khr`]
- [`update_descriptor_set_with_template_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        