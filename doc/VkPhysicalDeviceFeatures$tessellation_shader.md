[`tessellation_shader`] specifies
whether tessellation control and evaluation shaders are supported.
If this feature is not enabled, the
`VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT`,
`VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT`,
`VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT`,
`VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT`, and
`VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO` enum
values  **must**  not be used.
This also specifies whether shader modules  **can**  declare the
`Tessellation` capability.