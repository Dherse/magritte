[`fragment_stores_and_atomics`]
specifies whether storage buffers and images support stores and atomic
operations in the fragment shader stage.
If this feature is not enabled, all storage image, storage texel buffer,
and storage buffer variables used by the fragment stage in shader
modules  **must**  be decorated with the `NonWritable` decoration (or the
`readonly` memory qualifier in GLSL).