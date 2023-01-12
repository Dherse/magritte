[`max_fragment_shading_rate_coverage_samples`] specifies the maximum number
of coverage samples supported in a single fragment.
[`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal
to the product of the `width` and `height` members of
[`max_fragment_size`], and the sample count reported by
[`max_fragment_shading_rate_rasterization_samples`].
[`max_fragment_shading_rate_coverage_samples`] **must**  be less than or equal
to `maxSampleMaskWords` × 32 if
[`fragment_shading_rate_with_shader_sample_mask`] is supported.
This limit is purely informational, and is not validated.