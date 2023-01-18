[`unnormalized_coordinates`]
controls whether to use unnormalized or normalized texel coordinates to
address texels of the image.
When set to [`TRUE`], the range of the image coordinates used to
lookup the texel is in the range of zero to the image size in each
dimension.
When set to [`FALSE`] the range of image coordinates is zero to
one.When [`unnormalized_coordinates`] is [`TRUE`], images the sampler is
used with in the shader have the following requirements:
 - The `viewType` **must**  be either `VK_IMAGE_VIEW_TYPE_1D` or `VK_IMAGE_VIEW_TYPE_2D`.
 - The image view  **must**  have a single layer and a single mip level.
When [`unnormalized_coordinates`] is [`TRUE`], image built-in
functions in the shader that use the sampler have the following
requirements:
 - The functions  **must**  not use projection.
 - The functions  **must**  not use offsets.