[`shader_storage_image_multisample`] specifies whether multisampled
storage images are supported.
If this feature is not enabled, images that are created with a
`usage` that includes `VK_IMAGE_USAGE_STORAGE_BIT` **must**  be
created with `samples` equal to `VK_SAMPLE_COUNT_1_BIT`.
This also specifies whether shader modules  **can**  declare the
`StorageImageMultisample` and `ImageMSArray` capabilities.