[`VIEWPORT_W_SCALING_NV`] specifies that the
`pViewportScalings` state in
[`PipelineViewportWScalingStateCreateInfoNV`] will be ignored and
 **must**  be set dynamically with [`cmd_set_viewport_w_scaling_nv`] before
any draws are performed with a pipeline state with
[`PipelineViewportWScalingStateCreateInfoNV`] member
`viewportScalingEnable` set to [`TRUE`]