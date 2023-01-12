[`descriptor_count`] is the number of descriptors to update.
If [`descriptor_count`] is greater than the number of remaining array
elements in the destination binding, those affect consecutive bindings
in a manner similar to [`WriteDescriptorSet`] above.
If the descriptor binding identified by [`dst_binding`] has a
descriptor type of `VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
[`descriptor_count`] specifies the number of bytes to update and the
remaining array elements in the destination binding refer to the
remaining number of bytes in it.