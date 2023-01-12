[`handle_types`] is a bitmask of
[`ExternalMemoryHandleTypeFlagBitsNV`] specifying one or more memory
handle types that  **may**  be exported.
Multiple handle types  **may**  be requested for the same allocation as long
as they are compatible, as reported by
[`get_physical_device_external_image_format_properties_nv`].