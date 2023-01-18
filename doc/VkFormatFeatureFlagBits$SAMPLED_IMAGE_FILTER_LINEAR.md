[`SAMPLED_IMAGE_FILTER_LINEAR`] specifies that
if [`SAMPLED_IMAGE`] is also set, an image view
 **can**  be used with a sampler that has either of `magFilter` or
`minFilter` set to `VK_FILTER_LINEAR`, or `mipmapMode` set
to `VK_SAMPLER_MIPMAP_MODE_LINEAR`.
If [`BLIT_SRC`] is also set, an image can be
used as the `srcImage` to
[`cmd_blit_image2`] and [`cmd_blit_image`]
with a `filter` of `VK_FILTER_LINEAR`.
This bit  **must**  only be exposed for formats that also support the
[`SAMPLED_IMAGE`] or
[`BLIT_SRC`].If the format being queried is a depth/stencil format, this bit only
specifies that the depth aspect (not the stencil aspect) of an image of this
format supports linear filtering, and that linear filtering of the depth
aspect is supported whether depth compare is enabled in the sampler or not.
Where depth comparison is supported it  **may**  be linear filtered whether this
bit is present or not, but where this bit is not present the filtered value
 **may**  be computed in an implementation-dependent manner which differs from
the normal rules of linear filtering.
The resulting value  **must**  be in the range [0,1] and  **should**  be
proportional to, or a weighted average of, the number of comparison passes
or failures.