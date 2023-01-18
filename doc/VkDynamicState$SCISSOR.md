[`SCISSOR`] specifies that the `pScissors` state
in [`PipelineViewportStateCreateInfo`] will be ignored and  **must**  be
set dynamically with [`cmd_set_scissor`] before any drawing commands.
The number of scissor rectangles used by a pipeline is still specified
by the `scissorCount` member of
[`PipelineViewportStateCreateInfo`].