[`depth_bias_clamp`] specifies whether depth
bias clamping is supported.
If this feature is not enabled, the [`depth_bias_clamp`] member of the
[`PipelineRasterizationStateCreateInfo`] structure  **must**  be set to
0.0 unless the `VK_DYNAMIC_STATE_DEPTH_BIAS` dynamic state is
enabled, and the [`depth_bias_clamp`] parameter to
[`cmd_set_depth_bias`] **must**  be set to 0.0.