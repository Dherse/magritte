[`HOST_ONLY_POOL_VALVE`] specifies
that descriptor sets using this layout  **must**  be allocated from a
descriptor pool created with the
`VK_DESCRIPTOR_POOL_CREATE_HOST_ONLY_BIT_VALVE` bit set.
Descriptor set layouts created with this bit have no expressable limit
for maximum number of descriptors per-stage.
Host descriptor sets are limited only by available host memory, but  **may** 
be limited for implementation specific reasons.
Implementations  **may**  limit the number of supported descriptors to
UpdateAfterBind limits or non-UpdateAfterBind limits, whichever is
larger.