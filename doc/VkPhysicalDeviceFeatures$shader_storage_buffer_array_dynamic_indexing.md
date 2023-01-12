[`shader_storage_buffer_array_dynamic_indexing`] specifies whether arrays
of storage buffers  **can**  be indexed by dynamically uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` **must**  be indexed only by
constant integral expressions when aggregated into arrays in shader
code.
This also specifies whether shader modules  **can**  declare the
`StorageBufferArrayDynamicIndexing` capability.