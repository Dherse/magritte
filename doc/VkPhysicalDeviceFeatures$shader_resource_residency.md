[`shader_resource_residency`]
specifies whether image operations that return resource residency
information are supported in shader code.
If this feature is not enabled, the `OpImageSparse*` instructions
 **must**  not be used in shader code.
This also specifies whether shader modules  **can**  declare the
`SparseResidency` capability.
The feature requires at least one of the `sparseResidency*` features
to be supported.