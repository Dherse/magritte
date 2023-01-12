[`view_offsets`] is a pointer to an array of [`dependency_count`]
view offsets, one for each dependency.
If [`dependency_count`] is zero, each dependency’s view offset is
treated as zero.
Each view offset controls which views in the source subpass the views in
the destination subpass depend on.