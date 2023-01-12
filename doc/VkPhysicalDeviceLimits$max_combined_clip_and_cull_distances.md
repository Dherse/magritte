[`max_combined_clip_and_cull_distances`] is the maximum combined number of
clip and cull distances that  **can**  be used in a single shader stage.
The sum of the sizes of any pair of arrays declared with the
`ClipDistance` and `CullDistance` built-in decoration used by a
single shader stage in a shader module  **must**  be less than or equal to
this limit.