[`sparse_residency_image3_d`]
specifies whether the device  **can**  access partially resident 3D images.
If this feature is not enabled, images with an `imageType` of
`VK_IMAGE_TYPE_3D` **must**  not be created with
`VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member
of the [`ImageCreateInfo`] structure.