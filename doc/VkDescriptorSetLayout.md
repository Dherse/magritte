[VkDescriptorSetLayout](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorSetLayout.html) - Opaque handle to a descriptor set layout object

# C Specifications
A descriptor set layout object is defined by an array of zero or more
descriptor bindings.
Each individual descriptor binding is specified by a descriptor type, a
count (array size) of the number of descriptors in the binding, a set of
shader stages that  **can**  access the binding, and (if using immutable
samplers) an array of sampler descriptors.Descriptor set layout objects are represented by [`DescriptorSetLayout`]
handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSetLayout)
```

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSetAllocateInfo`]
- [`DescriptorSetBindingReferenceVALVE`]
- [`DescriptorUpdateTemplateCreateInfo`]
- [`PipelineLayoutCreateInfo`]
- [`create_descriptor_set_layout`]
- [`destroy_descriptor_set_layout`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        