[`max_per_stage_descriptor_sampled_images`] is the maximum number of
sampled images that  **can**  be accessible to a single shader stage in a
pipeline layout.
Descriptors with a type of
`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`, or
`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` count against this limit.
Only descriptors in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.
A descriptor is accessible to a pipeline shader stage when the
`stageFlags` member of the [`DescriptorSetLayoutBinding`]
structure has the bit for that shader stage set.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-combinedimagesampler),
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-sampledimage), and
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformtexelbuffer).