[`BLEND_CONSTANTS`] specifies that the
`blendConstants` state in [`PipelineColorBlendStateCreateInfo`]
will be ignored and  **must**  be set dynamically with
[`cmd_set_blend_constants`] before any draws are performed with a
pipeline state with [`PipelineColorBlendAttachmentState`] member
`blendEnable` set to [`TRUE`] and any of the blend functions
using a constant blend color.