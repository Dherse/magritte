[`workgroup_memory_explicit_layout16_bit_access`] indicates whether objects
in the `Workgroup` storage class with the `Block` decoration  **can** 
have 16-bit integer and 16-bit floating-point members.
If this feature is not enabled, 16-bit integer or 16-bit floating-point
members  **must**  not be used in such objects.
This also indicates whether shader modules  **can**  declare the
`WorkgroupMemoryExplicitLayout16BitAccessKHR` capability.