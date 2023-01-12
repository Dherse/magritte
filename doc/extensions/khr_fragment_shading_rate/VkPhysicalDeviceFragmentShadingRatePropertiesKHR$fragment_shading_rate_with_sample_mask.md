[`fragment_shading_rate_with_sample_mask`] specifies whether the the
implementation supports setting valid bits of
[`PipelineMultisampleStateCreateInfo`]::`pSampleMask` to `0` for
multi-pixel fragments.
If this value is `VK_FALSE`, zeroing valid bits in the sample mask
will clamp the fragment shading rate to (1,1).