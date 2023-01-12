[`buffer_device_address_multi_device`] indicates that the implementation
supports the [`buffer_device_address`]
, `rayTracingPipeline` and `rayQuery` features
for logical devices created with multiple physical devices.
If this feature is not supported, buffer
and acceleration structure
addresses  **must**  not be queried on a logical device created with more
than one physical device.