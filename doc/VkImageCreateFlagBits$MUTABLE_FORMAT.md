[`MUTABLE_FORMAT`] specifies that the image  **can** 
be used to create a [`ImageView`] with a different format from the
image.
For [multi-planar](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#formats-requiring-sampler-ycbcr-conversion) formats,
[`MUTABLE_FORMAT`] specifies that a
[`ImageView`] can be created of a *plane* of the image.