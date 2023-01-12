[`shader_sampled_image_array_non_uniform_indexing`] indicates whether arrays
of samplers or sampled images  **can**  be indexed by non-uniform integer
expressions in shader code.
If this feature is not enabled, resources with a descriptor type of
`VK_DESCRIPTOR_TYPE_SAMPLER`,
`VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`, or
`VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE` **must**  not be indexed by
non-uniform integer expressions when aggregated into arrays in shader
code.
This also indicates whether shader modules  **can**  declare the
`SampledImageArrayNonUniformIndexing` capability.