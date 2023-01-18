[`post_subpass_sample_locations`] is a pointer to an array of
[`post_subpass_sample_locations_count`][`SubpassSampleLocationsEXT`]
structures specifying the subpass indices and their corresponding sample
location state.
Each element of [`post_subpass_sample_locations`] **can**  specify the
sample location state to use in the automatic layout transition
performed to transition the depth/stencil attachment used by the
specified subpass to the image layout specified in a dependent subpass
or to the final layout of the attachment in case the specified subpass
is the last subpass using that attachment.
In addition, if
[`PhysicalDeviceSampleLocationsPropertiesEXT`]::`variableSampleLocations`
is [`FALSE`], each element of [`post_subpass_sample_locations`] **must**  specify the sample location state that matches the sample
locations used by all pipelines that will be bound to a command buffer
during the specified subpass.
If `variableSampleLocations` is [`TRUE`], the sample locations
used for rasterization do not depend on
[`post_subpass_sample_locations`].