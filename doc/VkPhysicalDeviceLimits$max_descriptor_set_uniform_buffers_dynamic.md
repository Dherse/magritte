[`max_descriptor_set_uniform_buffers_dynamic`] is the maximum number of
dynamic uniform buffers that  **can**  be included in a pipeline layout.
Descriptors with a type of
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` count against this
limit.
Only descriptors in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#descriptorsets-uniformbufferdynamic).