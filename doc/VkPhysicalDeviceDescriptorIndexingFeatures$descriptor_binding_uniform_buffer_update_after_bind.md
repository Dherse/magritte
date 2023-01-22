[`descriptor_binding_uniform_buffer_update_after_bind`] indicates whether
the implementation supports updating uniform buffer descriptors after a
set is bound.
If this feature is not enabled,
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`.