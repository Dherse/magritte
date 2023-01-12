[`G16_B16_R163PLANE_444_UNORM`] specifies an unsigned
normalized *multi-planar format* that has a 16-bit G component in each
16-bit word of plane 0, a 16-bit B component in each 16-bit word of
plane 1, and a 16-bit R component in each 16-bit word of plane 2.
Each plane has the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane,
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the B plane, and
`VK_IMAGE_ASPECT_PLANE_2_BIT` for the R plane.