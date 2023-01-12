[`RAY_TRACING_NO_NULL_INTERSECTION_SHADERS_KHR`]
specifies that an intersection shader will always be present when an
intersection shader would be executed.
A NULL intersection shader is an intersection shader which is
effectively `VK_SHADER_UNUSED_KHR`, such as from a shader group
consisting entirely of zeros.