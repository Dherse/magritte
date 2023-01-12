[`G16_B16R162PLANE_444_UNORM`] specifies an unsigned
normalized *multi-planar format* that has a 16-bit G component in each
16-bit word of plane 0, and a two-component, 32-bit BR plane 1
consisting of a 16-bit B component in the word in bytes 0..1, and a
16-bit R component in the word in bytes 2..3.
Both planes have the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane, and
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the BR plane.