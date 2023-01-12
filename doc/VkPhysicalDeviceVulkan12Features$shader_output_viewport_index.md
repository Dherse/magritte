[`shader_output_viewport_index`]
indicates whether the implementation supports the
`ShaderViewportIndex` SPIR-V capability enabling variables decorated
with the `ViewportIndex` built-in to be exported from vertex or
tessellation evaluation shaders.
If this feature is not enabled, the `ViewportIndex` built-in
decoration  **must**  not be used on outputs in vertex or tessellation
evaluation shaders.