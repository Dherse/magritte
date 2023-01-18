[`GEOMETRY_SHADER_INVOCATIONS`]
specifies that queries managed by the pool will count the number of
geometry shader invocations.
This counter’s value is incremented each time a geometry shader is
[invoked](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#shaders-geometry-execution).
In the case of [instanced geometry shaders](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#geometry-invocations), the
geometry shader invocations count is incremented for each separate
instanced invocation.