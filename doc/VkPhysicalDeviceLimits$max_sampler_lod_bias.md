[`max_sampler_lod_bias`] is the maximum
absolute sampler LOD bias.
The sum of the `mipLodBias` member of the [`SamplerCreateInfo`]
structure and the `Bias` operand of image sampling operations in
shader modules (or 0 if no `Bias` operand is provided to an image
sampling operation) are clamped to the range
[-[`max_sampler_lod_bias`],+[`max_sampler_lod_bias`]].
See [https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-mipLodBias](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#samplers-mipLodBias).