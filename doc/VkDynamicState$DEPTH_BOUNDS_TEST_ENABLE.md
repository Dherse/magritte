[`DEPTH_BOUNDS_TEST_ENABLE`] specifies that the
`depthBoundsTestEnable` state in
[`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be
set dynamically with [`cmd_set_depth_bounds_test_enable`] before any draw
call.