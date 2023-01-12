[`max_image_dimension2_d`] is the largest
dimension (`width` or `height`) that is guaranteed to be
supported for all images created with an `imageType` of
`VK_IMAGE_TYPE_2D` and without
`VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT` set in `flags`.
Some combinations of image parameters (format, usage, etc.)  **may**  allow
support for larger dimensions, which  **can**  be queried using
[`get_physical_device_image_format_properties`].