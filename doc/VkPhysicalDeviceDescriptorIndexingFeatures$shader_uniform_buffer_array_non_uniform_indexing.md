[`shader_uniform_buffer_array_non_uniform_indexing`] indicates whether
arrays of uniform buffers  **can**  be indexed by non-uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` **must**  not be indexed by
non-uniform integer expressions when aggregated into arrays in shader
code.
This also indicates whether shader modules  **can**  declare the
`UniformBufferArrayNonUniformIndexing` capability.