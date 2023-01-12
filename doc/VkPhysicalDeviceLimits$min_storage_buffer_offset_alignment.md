[`min_storage_buffer_offset_alignment`] is the minimum  **required** 
alignment, in bytes, for the `offset` member of the
[`DescriptorBufferInfo`] structure for storage buffers.
When a descriptor of type `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` is updated, the
`offset` **must**  be an integer multiple of this limit.
Similarly, dynamic offsets for storage buffers  **must**  be multiples of
this limit.
The value  **must**  be a power of two.