[`RAY_TRACING_NO_NULL_CLOSEST_HIT_SHADERS_KHR`]
specifies that a closest hit shader will always be present when a
closest hit shader would be executed.
A NULL closest hit shader is a closest hit shader which is effectively
[`SHADER_UNUSED_KHR`], such as from a shader group consisting
entirely of zeros.