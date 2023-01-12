[VkDescriptorPool](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkDescriptorPool.html) - Opaque handle to a descriptor pool object

# C Specifications
A *descriptor pool* maintains a pool of descriptors, from which descriptor
sets are allocated.
Descriptor pools are externally synchronized, meaning that the application
 **must**  not allocate and/or free descriptor sets from the same pool in
multiple threads simultaneously.Descriptor pools are represented by [`DescriptorPool`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorPool)
```

# Related
- [`crate::vulkan1_0`]
- [`DescriptorSetAllocateInfo`]
- [`create_descriptor_pool`]
- [`destroy_descriptor_pool`]
- [`free_descriptor_sets`]
- [`reset_descriptor_pool`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        