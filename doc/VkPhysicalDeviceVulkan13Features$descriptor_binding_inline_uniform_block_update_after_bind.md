[`descriptor_binding_inline_uniform_block_update_after_bind`]
indicates whether the implementation supports updating inline uniform
block descriptors after a set is bound.
If this feature is not enabled,
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
`VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK`.