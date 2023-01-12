[`workgroup_memory_explicit_layout8_bit_access`] indicates whether objects
in the `Workgroup` storage class with the `Block` decoration  **can** 
have 8-bit integer members.
If this feature is not enabled, 8-bit integer members  **must**  not be used
in such objects.
This also indicates whether shader modules  **can**  declare the
`WorkgroupMemoryExplicitLayout8BitAccessKHR` capability.