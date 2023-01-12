[`descriptor_binding_storage_texel_buffer_update_after_bind`] indicates
whether the implementation supports updating storage texel buffer
descriptors after a set is bound.
If this feature is not enabled,
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.