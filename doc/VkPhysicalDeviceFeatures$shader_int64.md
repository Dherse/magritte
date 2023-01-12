[`shader_int64`] specifies whether 64-bit
integers (signed and unsigned) are supported in shader code.
If this feature is not enabled, 64-bit integer types  **must**  not be used
in shader code.
This also specifies whether shader modules  **can**  declare the `Int64`
capability.
Declaring and using 64-bit integers is enabled for all storage classes
that SPIR-V allows with the `Int64` capability.