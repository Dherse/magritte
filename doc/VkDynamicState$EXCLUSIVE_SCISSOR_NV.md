[`EXCLUSIVE_SCISSOR_NV`] specifies that the
`pExclusiveScissors` state in
[`PipelineViewportExclusiveScissorStateCreateInfoNV`] will be
ignored and  **must**  be set dynamically with
[`cmd_set_exclusive_scissor_nv`] before any drawing commands.
The number of exclusive scissor rectangles used by a pipeline is still
specified by the `exclusiveScissorCount` member of
[`PipelineViewportExclusiveScissorStateCreateInfoNV`].