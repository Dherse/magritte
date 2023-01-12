[`shader_input_attachment_array_dynamic_indexing`] indicates whether arrays
of input attachments  **can**  be indexed by dynamically uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` **must**  be indexed only by
constant integral expressions when aggregated into arrays in shader
code.
This also indicates whether shader modules  **can**  declare the
`InputAttachmentArrayDynamicIndexing` capability.