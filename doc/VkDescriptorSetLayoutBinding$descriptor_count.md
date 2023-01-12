[`descriptor_count`] is the number of descriptors contained in the
binding, accessed in a shader as an
array, except if [`descriptor_type`] is
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` in which case
[`descriptor_count`] is the size in bytes of the inline uniform block.
If [`descriptor_count`] is zero this binding entry is reserved and the
resource  **must**  not be accessed from any stage via this binding within
any pipeline using the set layout.