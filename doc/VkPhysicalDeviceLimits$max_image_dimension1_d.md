[`max_image_dimension1_d`] is the largest
dimension (`width`) that is guaranteed to be supported for all
images created with an `imageType` of `VK_IMAGE_TYPE_1D`.
Some combinations of image parameters (format, usage, etc.)  **may**  allow
support for larger dimensions, which  **can**  be queried using
[`get_physical_device_image_format_properties`].