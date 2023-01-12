[`vertex_pipeline_stores_and_atomics`] specifies whether storage buffers
and images support stores and atomic operations in the vertex,
tessellation, and geometry shader stages.
If this feature is not enabled, all storage image, storage texel buffer,
and storage buffer variables used by these stages in shader modules
 **must**  be decorated with the `NonWritable` decoration (or the
`readonly` memory qualifier in GLSL).