[`sparse_residency2_samples`]
specifies whether the physical device  **can**  access partially resident 2D
images with 2 samples per pixel.
If this feature is not enabled, images with an `imageType` of
`VK_IMAGE_TYPE_2D` and `samples` set to
`VK_SAMPLE_COUNT_2_BIT` **must**  not be created with
`VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT` set in the `flags` member
of the [`ImageCreateInfo`] structure.