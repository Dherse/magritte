[`VK_COPY_ACCELERATION_STRUCTURE_MODE_KHR`] creates a direct
copy of the acceleration structure specified in `src` into the one
specified by `dst`.
The `dst` acceleration structure  **must**  have been created with the
same parameters as `src`.
If `src` contains references to other acceleration structures,
`dst` will reference the same acceleration structures.