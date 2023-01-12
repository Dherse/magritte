[`texture_compression_astc_hdr`] indicates whether all of the ASTC HDR
compressed texture formats are supported.
If this feature is enabled, then the
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must** 
be supported in `optimalTilingFeatures` for the following formats:
 - `VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK`
 - `VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK`
To query for additional properties, or if the feature is not enabled,
[`get_physical_device_format_properties`] and
[`get_physical_device_image_format_properties`] **can**  be used to check for
supported properties of individual formats as normal.