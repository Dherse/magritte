[`max_extra_primitive_overestimation_size`] is the maximum size in pixels
of extra overestimation the implementation supports in the pipeline
state.
A value of 0.0 means the implementation does not support any additional
overestimation of the generating primitive during conservative
rasterization.
A value above 0.0 allows the application to further increase the size of
the generating primitive during conservative rasterization
overestimation.