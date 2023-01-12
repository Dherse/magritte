[`descriptor_binding_storage_image_update_after_bind`] indicates whether the
implementation supports updating storage image descriptors after a set
is bound.
If this feature is not enabled,
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`.