[`shader_uniform_texel_buffer_array_non_uniform_indexing`] indicates whether
arrays of uniform texel buffers  **can**  be indexed by non-uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER` **must**  not be indexed by
non-uniform integer expressions when aggregated into arrays in shader
code.
This also indicates whether shader modules  **can**  declare the
`UniformTexelBufferArrayNonUniformIndexing` capability.