[`mesh_output_per_primitive_granularity`] is the granularity with which
mesh outputs qualified as per-primitive are allocated.
The value can be used to compute the memory size used by the mesh
shader, which must be less than or equal to
[`max_mesh_total_memory_size`].