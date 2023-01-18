use crate::{
    cstr,
    vulkan1_3::{FormatFeatureFlagBits2, FormatFeatureFlags2, FormatProperties3},
};
use std::ffi::CStr;
///See [`FormatFeatureFlags2`]
#[doc(alias = "VkFormatFeatureFlags2KHR")]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
///See [`FormatFeatureFlagBits2`]
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
pub type FormatFeatureFlagBits2KHR = FormatFeatureFlagBits2;
///See [`FormatProperties3`]
#[doc(alias = "VkFormatProperties3KHR")]
pub type FormatProperties3KHR = FormatProperties3;
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME")]
pub const KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_format_feature_flags2");
