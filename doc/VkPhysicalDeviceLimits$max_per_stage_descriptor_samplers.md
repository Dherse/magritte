[`max_per_stage_descriptor_samplers`] is the maximum number of samplers
that  **can**  be accessible to a single shader stage in a pipeline layout.
Descriptors with a type of `VK_DESCRIPTOR_TYPE_SAMPLER` or
`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER` count against this
limit.
Only descriptors in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.
A descriptor is accessible to a shader stage when the `stageFlags`
member of the [`DescriptorSetLayoutBinding`] structure has the bit
for that shader stage set.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampler) and
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler).