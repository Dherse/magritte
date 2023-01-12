[`multi_viewport`] specifies whether more
than one viewport is supported.
If this feature is not enabled:
 - The `viewportCount` and `scissorCount` members of the [`PipelineViewportStateCreateInfo`] structure  **must**  be set to 1.
 - The `firstViewport` and `viewportCount` parameters to the [`cmd_set_viewport`] command  **must**  be set to 0 and 1, respectively.
 - The `firstScissor` and `scissorCount` parameters to the [`cmd_set_scissor`] command  **must**  be set to 0 and 1, respectively.
 - The `exclusiveScissorCount` member of the [`PipelineViewportExclusiveScissorStateCreateInfoNV`] structure  **must**  be set to 0 or 1.
 - The `firstExclusiveScissor` and `exclusiveScissorCount` parameters to the [`cmd_set_exclusive_scissor_nv`] command  **must**  be set to 0 and 1, respectively.