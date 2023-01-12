[`descriptor_count`] is the number of descriptors to update.
If the descriptor binding identified by [`dst_set`] and
[`dst_binding`] has a descriptor type of
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`, then
[`descriptor_count`] specifies the number of bytes to update.
Otherwise,
[`descriptor_count`] is one of
 - the number of elements in [`image_info`]
 - the number of elements in [`buffer_info`]
 - the number of elements in [`texel_buffer_view`]
 - a value matching the `dataSize` member of a [`WriteDescriptorSetInlineUniformBlock`] structure in the [`p_next`] chain
 - a value matching the `accelerationStructureCount` of a [`WriteDescriptorSetAccelerationStructureKHR`] structure in the [`p_next`] chain