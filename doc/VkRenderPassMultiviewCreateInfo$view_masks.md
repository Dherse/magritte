[`view_masks`] is a pointer to an array of [`subpass_count`] view
masks, where each mask is a bitfield of view indices describing which
views rendering is broadcast to in each subpass, when multiview is
enabled.
If [`subpass_count`] is zero, each view mask is treated as zero.