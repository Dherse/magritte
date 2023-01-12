[`compatible_handle_types`] is a bitmask of
[`ExternalMemoryHandleTypeFlagBitsNV`] containing a bit set for
every external handle type that  **may**  be specified simultaneously with
the handle type specified by
[`get_physical_device_external_image_format_properties_nv`]::`externalHandleType`
when calling [`allocate_memory`], or 0 if the external memory handle
type is 0.
[`compatible_handle_types`] will always contain
[`get_physical_device_external_image_format_properties_nv`]::`externalHandleType`