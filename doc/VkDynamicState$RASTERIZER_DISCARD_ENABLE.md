[`RASTERIZER_DISCARD_ENABLE`] specifies that the
`rasterizerDiscardEnable` state in
[`PipelineRasterizationStateCreateInfo`] will be ignored and  **must** 
be set dynamically with [`cmd_set_rasterizer_discard_enable`] before any
drawing commands.