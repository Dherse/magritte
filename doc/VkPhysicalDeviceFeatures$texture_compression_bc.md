[`texture_compression_bc`] specifies
whether all of the BC compressed texture formats are supported.
If this feature is enabled, then the
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must** 
be supported in `optimalTilingFeatures` for the following formats:
 - `VK_FORMAT_BC1_RGB_UNORM_BLOCK`
 - `VK_FORMAT_BC1_RGB_SRGB_BLOCK`
 - `VK_FORMAT_BC1_RGBA_UNORM_BLOCK`
 - `VK_FORMAT_BC1_RGBA_SRGB_BLOCK`
 - `VK_FORMAT_BC2_UNORM_BLOCK`
 - `VK_FORMAT_BC2_SRGB_BLOCK`
 - `VK_FORMAT_BC3_UNORM_BLOCK`
 - `VK_FORMAT_BC3_SRGB_BLOCK`
 - `VK_FORMAT_BC4_UNORM_BLOCK`
 - `VK_FORMAT_BC4_SNORM_BLOCK`
 - `VK_FORMAT_BC5_UNORM_BLOCK`
 - `VK_FORMAT_BC5_SNORM_BLOCK`
 - `VK_FORMAT_BC6H_UFLOAT_BLOCK`
 - `VK_FORMAT_BC6H_SFLOAT_BLOCK`
 - `VK_FORMAT_BC7_UNORM_BLOCK`
 - `VK_FORMAT_BC7_SRGB_BLOCK`
To query for additional properties, or if the feature is not enabled,
[`get_physical_device_format_properties`] and
[`get_physical_device_image_format_properties`] **can**  be used to check for
supported properties of individual formats as normal.