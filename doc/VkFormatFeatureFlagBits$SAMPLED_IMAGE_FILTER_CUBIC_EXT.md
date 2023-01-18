[`SAMPLED_IMAGE_FILTER_CUBIC_EXT`] specifies
that [`Image`] **can**  be used with a sampler that has either of
`magFilter` or `minFilter` set to `VK_FILTER_CUBIC_EXT`, or
be the source image for a blit with `filter` set to
`VK_FILTER_CUBIC_EXT`.
This bit  **must**  only be exposed for formats that also support the
[`SAMPLED_IMAGE`].
If the format being queried is a depth/stencil format, this only
specifies that the depth aspect is cubic filterable.