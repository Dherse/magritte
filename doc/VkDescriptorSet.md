[VkDescriptorSet](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSet.html) - Opaque handle to a descriptor set object

# C Specifications
Descriptor sets are allocated from descriptor pool objects, and are
represented by [`DescriptorSet`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSet)
```

# Related
- [`crate::vulkan1_0`]
- [`CopyDescriptorSet`]
- [`WriteDescriptorSet`]
- [`allocate_descriptor_sets`]
- [`cmd_bind_descriptor_sets`]
- [`free_descriptor_sets`]
- [`get_descriptor_set_host_mapping_valve`]
- [`update_descriptor_set_with_template`]
- [`update_descriptor_set_with_template_khr`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        