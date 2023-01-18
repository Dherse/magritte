[`GPU_ASSISTED_RESERVE_BINDING_SLOT`]
specifies that the validation layers reserve a descriptor set binding
slot for their own use.
The layer reports a value for
[`PhysicalDeviceLimits`]::`maxBoundDescriptorSets` that is one
less than the value reported by the device.
If the device supports the binding of only one descriptor set, the
validation layer does not perform GPU-assisted validation.
This feature is disabled by default.