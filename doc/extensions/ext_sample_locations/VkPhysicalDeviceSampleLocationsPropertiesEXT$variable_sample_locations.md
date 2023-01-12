[`variable_sample_locations`]
specifies whether the sample locations used by all pipelines that will
be bound to a command buffer during a subpass  **must**  match.
If set to `VK_TRUE`, the implementation supports variable sample
locations in a subpass.
If set to `VK_FALSE`, then the sample locations  **must**  stay constant
in each subpass.