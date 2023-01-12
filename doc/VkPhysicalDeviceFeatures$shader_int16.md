[`shader_int16`] specifies whether 16-bit
integers (signed and unsigned) are supported in shader code.
If this feature is not enabled, 16-bit integer types  **must**  not be used
in shader code.
This also specifies whether shader modules  **can**  declare the `Int16`
capability.
However, this only enables a subset of the storage classes that SPIR-V
allows for the `Int16` SPIR-V capability: Declaring and using 16-bit
integers in the `Private`,
`Workgroup` (for non-Block variables),
and `Function` storage classes is enabled, while declaring them in
the interface storage classes (e.g., `UniformConstant`, `Uniform`,
`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
not enabled.