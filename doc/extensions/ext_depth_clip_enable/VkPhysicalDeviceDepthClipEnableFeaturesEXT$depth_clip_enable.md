[`depth_clip_enable`] indicates that the
implementation supports setting the depth clipping operation explicitly
via the [`PipelineRasterizationDepthClipStateCreateInfoEXT`]
pipeline state.
Otherwise depth clipping is only enabled when
[`PipelineRasterizationStateCreateInfo`]::`depthClampEnable` is
set to [`FALSE`].