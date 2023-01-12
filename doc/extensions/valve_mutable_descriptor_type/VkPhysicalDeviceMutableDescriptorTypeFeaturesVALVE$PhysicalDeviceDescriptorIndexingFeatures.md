`VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT` with
`descriptorType` of `VK_DESCRIPTOR_TYPE_MUTABLE_VALVE` relaxes
the list of required descriptor types to the descriptor types which
have the corresponding update-after-bind feature enabled in
[`PhysicalDeviceDescriptorIndexingFeatures`].