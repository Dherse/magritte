[`descriptor_set_layout`] is the descriptor set layout used to build the
descriptor update template.
All descriptor sets which are going to be updated through the newly
created descriptor update template  **must**  be created with a layout that
matches (is the same as, or defined identically to) this layout.
This parameter is ignored if [`template_type`] is not
`VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET`.