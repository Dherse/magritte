[`max_per_stage_descriptor_storage_buffers`] is the maximum number of
storage buffers that  **can**  be accessible to a single shader stage in a
pipeline layout.
Descriptors with a type of `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` count against this
limit.
Only descriptors in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.
A descriptor is accessible to a pipeline shader stage when the
`stageFlags` member of the [`DescriptorSetLayoutBinding`]
structure has the bit for that shader stage set.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebuffer) and
[https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-storagebufferdynamic).