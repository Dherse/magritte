[`G12X4_B12X4_R12X43PLANE_444_UNORM_3PACK16`] specifies an
unsigned normalized *multi-planar format* that has a 12-bit G component
in the top 12 bits of each 16-bit word of plane 0, a 12-bit B component
in the top 12 bits of each 16-bit word of plane 1, and a 12-bit R
component in the top 12 bits of each 16-bit word of plane 2, with the
bottom 4 bits of each word unused.
Each plane has the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane,
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the B plane, and
`VK_IMAGE_ASPECT_PLANE_2_BIT` for the R plane.