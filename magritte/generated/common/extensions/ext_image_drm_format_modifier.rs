use crate::{
    common::{vulkan1_0::FormatFeatureFlags, vulkan1_3::FormatFeatureFlags2},
    cstr,
};
use std::ffi::CStr;
#[doc(alias = "VkDrmFormatModifierPropertiesEXT")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierPropertiesEXT {
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    pub drm_format_modifier_plane_count: u32,
    #[doc(alias = "drmFormatModifierTilingFeatures")]
    pub drm_format_modifier_tiling_features: FormatFeatureFlags,
}
#[doc(alias = "VkDrmFormatModifierProperties2EXT")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct DrmFormatModifierProperties2EXT {
    #[doc(alias = "drmFormatModifier")]
    pub drm_format_modifier: u64,
    #[doc(alias = "drmFormatModifierPlaneCount")]
    pub drm_format_modifier_plane_count: u32,
    #[doc(alias = "drmFormatModifierTilingFeatures")]
    pub drm_format_modifier_tiling_features: FormatFeatureFlags2,
}
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME")]
pub const EXT_IMAGE_DRM_FORMAT_MODIFIER_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_image_drm_format_modifier");
