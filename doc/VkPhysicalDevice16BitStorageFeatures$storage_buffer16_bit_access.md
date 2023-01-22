[`storage_buffer16_bit_access`] specifies whether objects in the
    `StorageBuffer`,
`ShaderRecordBufferKHR`,
    or `PhysicalStorageBuffer`
    storage class with the `Block` decoration  **can**  have 16-bit integer
    and 16-bit floating-point members.
    If this feature is not enabled, 16-bit integer or 16-bit floating-point
    members  **must**  not be used in such objects.
    This also specifies whether shader modules  **can**  declare the
    `StorageBuffer16BitAccess` capability.