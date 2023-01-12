[`min_uniform_buffer_offset_alignment`] is the minimum  **required** 
alignment, in bytes, for the `offset` member of the
[`DescriptorBufferInfo`] structure for uniform buffers.
When a descriptor of type `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` is updated, the
`offset` **must**  be an integer multiple of this limit.
Similarly, dynamic offsets for uniform buffers  **must**  be multiples of
this limit.
The value  **must**  be a power of two.