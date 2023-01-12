[VkDescriptorUpdateTemplate](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorUpdateTemplate.html) - Opaque handle to a descriptor update template

# C Specifications
A descriptor update template specifies a mapping from descriptor update
information in host memory to descriptors in a descriptor set.
It is designed to avoid passing redundant information to the driver when
frequently updating the same set of descriptors in descriptor sets.Descriptor update template objects are represented by
[`DescriptorUpdateTemplate`] handles:
```c
// Provided by VK_VERSION_1_1
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorUpdateTemplate)
```
or the equivalent
```c
// Provided by VK_KHR_descriptor_update_template
typedef VkDescriptorUpdateTemplate VkDescriptorUpdateTemplateKHR;
```

# Related
- [`crate::vulkan1_1`]
- [`cmd_push_descriptor_set_with_template_khr`]
- [`create_descriptor_update_template`]
- [`create_descriptor_update_template_khr`]
- [`destroy_descriptor_update_template`]
- [`destroy_descriptor_update_template_khr`]
- [`update_descriptor_set_with_template`]
- [`update_descriptor_set_with_template_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        