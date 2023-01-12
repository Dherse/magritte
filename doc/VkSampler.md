[VkSampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkSampler.html) - Opaque handle to a sampler object

# C Specifications
[`Sampler`] objects represent the state of an image sampler which is
used by the implementation to read image data and apply filtering and other
transformations for the shader.Samplers are represented by [`Sampler`] handles:
```c
// Provided by VK_VERSION_1_0
VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSampler)
```

# Related
- [`crate::vulkan1_0`]
- [`DescriptorImageInfo`]
- [`DescriptorSetLayoutBinding`]
- [`ImageViewHandleInfoNVX`]
- [`create_sampler`]
- [`destroy_sampler`]

# Notes and documentation
For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
This documentation is generated from the Vulkan specification and documentation.
The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative Commons Attribution 4.0 International*.
his license explicitely allows adapting the source material as long as proper credit is given.
        