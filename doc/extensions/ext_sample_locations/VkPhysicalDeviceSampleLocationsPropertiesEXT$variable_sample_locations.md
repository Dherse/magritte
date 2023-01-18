[`variable_sample_locations`]
specifies whether the sample locations used by all pipelines that will
be bound to a command buffer during a subpass  **must**  match.
If set to [`TRUE`], the implementation supports variable sample
locations in a subpass.
If set to [`FALSE`], then the sample locations  **must**  stay constant
in each subpass.