[`DEPTH_BIAS_ENABLE`] specifies that the
`depthBiasEnable` state in
[`PipelineRasterizationStateCreateInfo`] will be ignored and  **must** 
be set dynamically with [`cmd_set_depth_bias_enable`] before any drawing
commands.