[`template_type`] Specifies the type of the descriptor update template.
If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET` it
 **can**  only be used to update descriptor sets with a fixed
[`descriptor_set_layout`].
If set to `VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_PUSH_DESCRIPTORS_KHR`
it  **can**  only be used to push descriptor sets using the provided
[`pipeline_bind_point`], [`pipeline_layout`], and [`set`] number.