[`DEPTH_BIAS`] specifies that the
`depthBiasConstantFactor`, `depthBiasClamp` and
`depthBiasSlopeFactor` states in
[`PipelineRasterizationStateCreateInfo`] will be ignored and  **must** 
be set dynamically with [`cmd_set_depth_bias`] before any draws are
performed with `depthBiasEnable` in
[`PipelineRasterizationStateCreateInfo`] set to [`TRUE`].