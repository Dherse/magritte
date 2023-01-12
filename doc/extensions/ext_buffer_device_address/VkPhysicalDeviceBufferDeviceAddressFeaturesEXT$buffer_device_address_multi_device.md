[`buffer_device_address_multi_device`] indicates that the implementation
supports the [`buffer_device_address`] feature for logical devices
created with multiple physical devices.
If this feature is not supported, buffer addresses  **must**  not be queried
on a logical device created with more than one physical device.