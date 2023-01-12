[`shader_int8`] indicates
whether 8-bit integers (signed and unsigned) are supported in shader
code.
This also indicates whether shader modules  **can**  declare the `Int8`
capability.
However, this only enables a subset of the storage classes that SPIR-V
allows for the `Int8` SPIR-V capability: Declaring and using 8-bit
integers in the `Private`,
`Workgroup` (for non-Block variables),
and `Function` storage classes is enabled, while declaring them in
the interface storage classes (e.g., `UniformConstant`, `Uniform`,
`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
not enabled.