[`cooperative_matrix_supported_stages`] is a bitfield of
[`ShaderStageFlagBits`] describing the shader stages that
cooperative matrix instructions are supported in.
[`cooperative_matrix_supported_stages`] will have the
`VK_SHADER_STAGE_COMPUTE_BIT` bit set if any of the physical
device’s queues support `VK_QUEUE_COMPUTE_BIT`.