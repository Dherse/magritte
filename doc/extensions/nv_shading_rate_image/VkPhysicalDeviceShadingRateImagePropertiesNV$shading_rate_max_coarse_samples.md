[`shading_rate_max_coarse_samples`]
specifies the maximum number of coverage samples supported in a single
fragment.
If the product of the fragment size derived from the base shading rate
and the number of coverage samples per pixel exceeds this limit, the
final shading rate will be adjusted so that its product does not exceed
the limit.