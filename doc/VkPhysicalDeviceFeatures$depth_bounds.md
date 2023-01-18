[`depth_bounds`] specifies whether depth
bounds tests are supported.
If this feature is not enabled, the `depthBoundsTestEnable` member
of the [`PipelineDepthStencilStateCreateInfo`] structure  **must**  be
set to [`FALSE`].
When `depthBoundsTestEnable` is set to [`FALSE`], the
`minDepthBounds` and `maxDepthBounds` members of the
[`PipelineDepthStencilStateCreateInfo`] structure are ignored.