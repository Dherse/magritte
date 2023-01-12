[`G10X6_B10X6R10X62PLANE_444_UNORM_3PACK16`] specifies an
unsigned normalized *multi-planar format* that has a 10-bit G component
in the top 10 bits of each 16-bit word of plane 0, and a two-component,
32-bit BR plane 1 consisting of a 10-bit B component in the top 10 bits
of the word in bytes 0..1, and a 10-bit R component in the top 10 bits
of the word in bytes 2..3, the bottom 6 bits of each word unused.
Both planes have the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane, and
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the BR plane.