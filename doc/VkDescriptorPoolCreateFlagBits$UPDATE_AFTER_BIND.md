[`UPDATE_AFTER_BIND`] specifies that
descriptor sets allocated from this pool  **can**  include bindings with the
`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit set.
It is valid to allocate descriptor sets that have bindings that do not
set the `VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` bit from a
pool that has [`UPDATE_AFTER_BIND`] set.