[`depth_clamp`] specifies whether depth
clamping is supported.
If this feature is not enabled, the `depthClampEnable` member of the
[`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to
`VK_FALSE`.
Otherwise, setting `depthClampEnable` to `VK_TRUE` will enable
depth clamping.