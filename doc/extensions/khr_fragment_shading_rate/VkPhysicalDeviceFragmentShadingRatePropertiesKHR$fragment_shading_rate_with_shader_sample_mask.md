[`fragment_shading_rate_with_shader_sample_mask`] specifies whether the
implementation supports reading or writing [`SampleMask`] for
multi-pixel fragments.
If this value is `VK_FALSE`, using that built-in will clamp the
fragment shading rate to (1,1).