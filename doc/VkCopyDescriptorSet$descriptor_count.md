[`descriptor_count`] is the number of descriptors to copy from the
source to destination.
If [`descriptor_count`] is greater than the number of remaining array
elements in the source or destination binding, those affect consecutive
bindings in a manner similar to [`WriteDescriptorSet`] above.
If the descriptor binding identified by [`src_set`] and
[`src_binding`] has a descriptor type of
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then [`descriptor_count`]
specifies the number of bytes to copy and the remaining array elements
in the source or destination binding refer to the remaining number of
bytes in those.