[`sparse_residency_aliased`]
specifies whether the physical device  **can**  correctly access data aliased
into multiple locations.
If this feature is not enabled, the
`VK_BUFFER_CREATE_SPARSE_ALIASED_BIT` and
`VK_IMAGE_CREATE_SPARSE_ALIASED_BIT` enum values  **must**  not be used
in `flags` members of the [`BufferCreateInfo`] and
[`ImageCreateInfo`] structures, respectively.