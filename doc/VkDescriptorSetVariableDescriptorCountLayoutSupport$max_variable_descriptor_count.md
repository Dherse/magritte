[`max_variable_descriptor_count`] indicates the maximum number of
descriptors supported in the highest numbered binding of the layout, if
that binding is variable-sized.
If the highest numbered binding of the layout has a descriptor type of
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK` then
[`max_variable_descriptor_count`] indicates the maximum byte size
supported for the binding, if that binding is variable-sized.