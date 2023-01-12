[`G8_B8R82PLANE_444_UNORM`] specifies an unsigned
normalized *multi-planar format* that has an 8-bit G component in plane
0, and a two-component, 16-bit BR plane 1 consisting of an 8-bit B
component in byte 0 and an 8-bit R component in byte 1.
Both planes have the same dimensions and each R, G and B component
contributes to a single texel.
The location of each plane when this image is in linear layout can be
determined via [`get_image_subresource_layout`], using
`VK_IMAGE_ASPECT_PLANE_0_BIT` for the G plane, and
`VK_IMAGE_ASPECT_PLANE_1_BIT` for the BR plane.