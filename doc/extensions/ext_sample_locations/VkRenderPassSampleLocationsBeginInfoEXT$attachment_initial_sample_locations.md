[`attachment_initial_sample_locations`] is a pointer to an array of
[`attachment_initial_sample_locations_count`][`AttachmentSampleLocationsEXT`] structures specifying the
attachment indices and their corresponding sample location state.
Each element of [`attachment_initial_sample_locations`] **can**  specify the
sample location state to use in the automatic layout transition
performed to transition a depth/stencil attachment from the initial
layout of the attachment to the image layout specified for the
attachment in the first subpass using it.