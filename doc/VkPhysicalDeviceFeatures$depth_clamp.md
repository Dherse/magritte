[`depth_clamp`] specifies whether depth
clamping is supported.
If this feature is not enabled, the `depthClampEnable` member of the
[`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to
[`FALSE`].
Otherwise, setting `depthClampEnable` to [`TRUE`] will enable
depth clamping.