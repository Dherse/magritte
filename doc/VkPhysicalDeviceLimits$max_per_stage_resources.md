[`max_per_stage_resources`] is the
maximum number of resources that  **can**  be accessible to a single shader
stage in a pipeline layout.
Descriptors with a type of
`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`,
`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`,
`VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`,
`VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`,
`VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`,
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER`,
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER`,
`VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC`,
`VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC`, or
`VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` count against this limit.
Only descriptors in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.
For the fragment shader stage the framebuffer color attachments also
count against this limit.