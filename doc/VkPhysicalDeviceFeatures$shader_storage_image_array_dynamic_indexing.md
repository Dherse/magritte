[`shader_storage_image_array_dynamic_indexing`] specifies whether arrays of
storage images  **can**  be indexed by dynamically uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE` **must**  be indexed only by constant
integral expressions when aggregated into arrays in shader code.
This also specifies whether shader modules  **can**  declare the
`StorageImageArrayDynamicIndexing` capability.