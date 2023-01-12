[`shader_cull_distance`] specifies
whether cull distances are supported in shader code.
If this feature is not enabled, any members decorated with the
`CullDistance` built-in decoration  **must**  not be read from or written
to in shader modules.
This also specifies whether shader modules  **can**  declare the
`CullDistance` capability.