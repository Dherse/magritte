[`UPDATE_AFTER_BIND_POOL`]
specifies that descriptor sets using this layout  **must**  be allocated from
a descriptor pool created with the
`VK_DESCRIPTOR_POOL_CREATE_UPDATE_AFTER_BIND_BIT` bit set.
Descriptor set layouts created with this bit set have alternate limits
for the maximum number of descriptors per-stage and per-pipeline layout.
The non-UpdateAfterBind limits only count descriptors in sets created
without this flag.
The UpdateAfterBind limits count all descriptors, but the limits  **may**  be
higher than the non-UpdateAfterBind limits.