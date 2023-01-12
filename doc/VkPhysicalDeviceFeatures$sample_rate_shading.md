[`sample_rate_shading`] specifies whether
[Sample Shading](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#primsrast-sampleshading) and multisample interpolation
are supported.
If this feature is not enabled, the `sampleShadingEnable` member of
the [`PipelineMultisampleStateCreateInfo`] structure  **must**  be set to
`VK_FALSE` and the `minSampleShading` member is ignored.
This also specifies whether shader modules  **can**  declare the
`SampleRateShading` capability.