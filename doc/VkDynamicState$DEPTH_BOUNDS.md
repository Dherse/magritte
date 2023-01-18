[`DEPTH_BOUNDS`] specifies that the
`minDepthBounds` and `maxDepthBounds` states of
[`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be
set dynamically with [`cmd_set_depth_bounds`] before any draws are
performed with a pipeline state with
[`PipelineDepthStencilStateCreateInfo`] member
`depthBoundsTestEnable` set to [`TRUE`].