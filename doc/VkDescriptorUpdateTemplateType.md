[VkDescriptorUpdateTemplateType](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplateType.html) - Indicates the valid usage of the descriptor update template

# C Specifications
The descriptor update template type is determined by the
[`DescriptorUpdateTemplateCreateInfo::template_type`] property,
which takes the following values:
```c
// Provided by VK_VERSION_1_1
typedef enum VkDescriptorUpdateTemplateType {
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET = 0,
  // Provided by VK_VERSION_1_1 with VK_KHR_push_descriptor, VK_KHR_descriptor_update_template with VK_KHR_push_descriptor
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR = 1,
  // Provided by VK_KHR_descriptor_update_template
    VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET_KHR = VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET,
} VkDescriptorUpdateTemplateType;
```
or the equivalent
```c
// Provided by VK_KHR_descriptor_update_template
typedef VkDescriptorUpdateTemplateType VkDescriptorUpdateTemplateTypeKHR;
```

# Description
- [`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE`] specifies that the descriptor update template will be used for descriptor set updates only.
- [`PUSH_DESCRIPTORS_KHR`] specifies that the descriptor update template will be used for push descriptor updates only.

# Related
- [`crate::vulkan1_1`]
- [`DescriptorUpdateTemplateCreateInfo`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        