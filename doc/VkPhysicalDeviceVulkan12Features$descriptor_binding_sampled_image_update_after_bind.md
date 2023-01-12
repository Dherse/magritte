[`descriptor_binding_sampled_image_update_after_bind`] indicates whether the
implementation supports updating sampled image descriptors after a set
is bound.
If this feature is not enabled,
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` **must**  not be used with
`VK_DESCRIPTOR_TYPE_SAMPLER`,
`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`.