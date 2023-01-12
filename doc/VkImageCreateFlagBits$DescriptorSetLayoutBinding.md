Image data read as an image sampler will have undefined values if the
sampler was not created with `flags` containing
`VK_SAMPLER_CREATE_SUBSAMPLED_BIT_EXT` or was not sampled through
the use of a combined image sampler with an immutable sampler in
[`DescriptorSetLayoutBinding`].