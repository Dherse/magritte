[`STENCIL_WRITE_MASK`] specifies that the
`writeMask` state in [`PipelineDepthStencilStateCreateInfo`] for
both `front` and `back` will be ignored and  **must**  be set
dynamically with [`cmd_set_stencil_write_mask`] before any draws are
performed with a pipeline state with
[`PipelineDepthStencilStateCreateInfo`] member
`stencilTestEnable` set to [`TRUE`]