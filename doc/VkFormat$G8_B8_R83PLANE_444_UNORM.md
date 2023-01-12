[`G8_B8_R83PLANE_444_UNORM`] specifies an unsigned
normalized *multi-planar format* that has an 8-bit G component in plane
0, an 8-bit B component in plane 1, and an 8-bit R component in plane 2.
Each plane has the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane,
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the B plane, and
`VK_IMAGE_ASPECT_PLANE_2_BIT` for the R plane.