[`texture_compression_astc_ldr`]
specifies whether all of the ASTC LDR compressed texture formats are
supported.
If this feature is enabled, then the
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must** 
be supported in `optimalTilingFeatures` for the following formats:
 - `VK_FORMAT_ASTC_4x4_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_4x4_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_5x4_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_5x4_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_5x5_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_5x5_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_6x5_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_6x5_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_6x6_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_6x6_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_8x5_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_8x5_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_8x6_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_8x6_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_8x8_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_8x8_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_10x5_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_10x5_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_10x6_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_10x6_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_10x8_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_10x8_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_10x10_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_10x10_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_12x10_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_12x10_SRGB_BLOCK`
 - `VK_FORMAT_ASTC_12x12_UNORM_BLOCK`
 - `VK_FORMAT_ASTC_12x12_SRGB_BLOCK`
To query for additional properties, or if the feature is not enabled,
[`get_physical_device_format_properties`] and
[`get_physical_device_image_format_properties`] **can**  be used to check for
supported properties of individual formats as normal.