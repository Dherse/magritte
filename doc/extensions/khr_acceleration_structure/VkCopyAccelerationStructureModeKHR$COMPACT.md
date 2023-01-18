[`COMPACT`] creates a more
compact version of an acceleration structure `src` into `dst`.
The acceleration structure `dst` **must**  have been created with a size
at least as large as that returned by
[`cmd_write_acceleration_structures_properties_khr`]
or [`write_acceleration_structures_properties_khr`]
after the build of the acceleration structure specified by `src`.
If `src` contains references to other acceleration structures,
`dst` will reference the same acceleration structures.