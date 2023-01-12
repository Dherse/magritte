[`max_descriptor_set_acceleration_structures`] is the maximum number of
acceleration structure descriptors that  **can**  be included in descriptor
bindings in a pipeline layout across all pipeline shader stages and
descriptor set numbers.
Descriptor bindings with a descriptor type of
`VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR` count against this
limit.
Only descriptor bindings in descriptor set layouts created without the
`VK_DESCRIPTOR_SET_LAYOUT_CREATE_UPDATE_AFTER_BIND_POOL_BIT` bit set
count against this limit.