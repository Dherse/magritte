[`sparse_residency_buffer`] specifies
whether the device  **can**  access partially resident buffers.
If this feature is not enabled, buffers  **must**  not be created with
`VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags`
member of the [`BufferCreateInfo`] structure.