[`sparse_binding`] specifies whether
resource memory  **can**  be managed at opaque sparse block level instead of
at the object level.
If this feature is not enabled, resource memory  **must**  be bound only on a
per-object basis using the [`bind_buffer_memory`] and
[`bind_image_memory`] commands.
In this case, buffers and images  **must**  not be created with
`VK_BUFFER_CREATE_SPARSE_BINDING_BIT` and
`VK_IMAGE_CREATE_SPARSE_BINDING_BIT` set in the `flags` member
of the [`BufferCreateInfo`] and [`ImageCreateInfo`] structures,
respectively.
Otherwise resource memory  **can**  be managed as described in
[Sparse Resource Features](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#sparsememory-sparseresourcefeatures).