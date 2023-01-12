[`descriptor_type`] is a [`DescriptorType`] specifying the type of
each descriptor in [`image_info`], [`buffer_info`], or
[`texel_buffer_view`], as described below.
If [`DescriptorSetLayoutBinding`] for [`dst_set`] at
[`dst_binding`] is not equal to `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE`,
[`descriptor_type`] **must** 
be the same type as the [`descriptor_type`] specified in
[`DescriptorSetLayoutBinding`] for [`dst_set`] at [`dst_binding`].
The type of the descriptor also controls which array the descriptors are
taken from.