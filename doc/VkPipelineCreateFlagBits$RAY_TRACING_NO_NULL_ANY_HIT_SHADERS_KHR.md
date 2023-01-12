[`RAY_TRACING_NO_NULL_ANY_HIT_SHADERS_KHR`]
specifies that an any-hit shader will always be present when an any-hit
shader would be executed.
A NULL any-hit shader is an any-hit shader which is effectively
`VK_SHADER_UNUSED_KHR`, such as from a shader group consisting
entirely of zeros.