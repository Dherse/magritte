[`texture_compression_etc2`]
specifies whether all of the ETC2 and EAC compressed texture formats are
supported.
If this feature is enabled, then the
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT`,
`VK_FORMAT_FEATURE_BLIT_SRC_BIT` and
`VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT` features  **must** 
be supported in `optimalTilingFeatures` for the following formats:
 - `VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK`
 - `VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK`
 - `VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK`
 - `VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK`
 - `VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK`
 - `VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK`
 - `VK_FORMAT_EAC_R11_UNORM_BLOCK`
 - `VK_FORMAT_EAC_R11_SNORM_BLOCK`
 - `VK_FORMAT_EAC_R11G11_UNORM_BLOCK`
 - `VK_FORMAT_EAC_R11G11_SNORM_BLOCK`
To query for additional properties, or if the feature is not enabled,
[`get_physical_device_format_properties`] and
[`get_physical_device_image_format_properties`] **can**  be used to check for
supported properties of individual formats as normal.