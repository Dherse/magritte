[`STENCIL_TEST_ENABLE`] specifies that the
`stencilTestEnable` state in
[`PipelineDepthStencilStateCreateInfo`] will be ignored and  **must**  be
set dynamically with [`cmd_set_stencil_test_enable`] before any draw
call.