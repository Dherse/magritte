[`RAY_TRACING_NO_NULL_MISS_SHADERS_KHR`]
specifies that a miss shader will always be present when a miss shader
would be executed.
A NULL miss shader is a miss shader which is effectively
[`SHADER_UNUSED_KHR`], such as from a shader group consisting
entirely of zeros.