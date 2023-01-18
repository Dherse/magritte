[`VIEWPORT`] specifies that the `pViewports`
state in [`PipelineViewportStateCreateInfo`] will be ignored and
 **must**  be set dynamically with [`cmd_set_viewport`] before any drawing
commands.
The number of viewports used by a pipeline is still specified by the
`viewportCount` member of [`PipelineViewportStateCreateInfo`].