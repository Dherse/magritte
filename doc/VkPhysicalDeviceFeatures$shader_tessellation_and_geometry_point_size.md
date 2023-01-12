[`shader_tessellation_and_geometry_point_size`] specifies whether the
`PointSize` built-in decoration is available in the tessellation
control, tessellation evaluation, and geometry shader stages.
If this feature is not enabled, members decorated with the
`PointSize` built-in decoration  **must**  not be read from or written to
and all points written from a tessellation or geometry shader will have
a size of 1.0.
This also specifies whether shader modules  **can**  declare the
`TessellationPointSize` capability for tessellation control and
evaluation shaders, or if the shader modules  **can**  declare the
`GeometryPointSize` capability for geometry shaders.
An implementation supporting this feature  **must**  also support one or both
of the [[`tessellation_shader`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-tessellationShader) or
[[`geometry_shader`]](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#features-geometryShader) features.