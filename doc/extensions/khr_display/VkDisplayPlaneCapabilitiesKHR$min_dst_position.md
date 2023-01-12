[`min_dst_position`], [`max_dst_position`], [`min_dst_extent`],
[`max_dst_extent`] all have similar semantics to their corresponding
`*Src*` equivalents, but apply to the output region within the mode
rather than the input region within the source image.
Unlike the `*Src*` offsets, [`min_dst_position`] and
[`max_dst_position`] **may**  contain negative values.