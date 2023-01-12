[`shader_float64`] specifies whether 64-bit
floats (doubles) are supported in shader code.
If this feature is not enabled, 64-bit floating-point types  **must**  not be
used in shader code.
This also specifies whether shader modules  **can**  declare the `Float64`
capability.
Declaring and using 64-bit floats is enabled for all storage classes
that SPIR-V allows with the `Float64` capability.