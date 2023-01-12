[`shader_float16`] indicates
whether 16-bit floats (halfs) are supported in shader code.
This also indicates whether shader modules  **can**  declare the `Float16`
capability.
However, this only enables a subset of the storage classes that SPIR-V
allows for the `Float16` SPIR-V capability: Declaring and using
16-bit floats in the `Private`,
`Workgroup` (for non-Block variables),
and `Function` storage classes is enabled, while declaring them in
the interface storage classes (e.g., `UniformConstant`, `Uniform`,
`StorageBuffer`, `Input`, `Output`, and `PushConstant`) is
not enabled.