use crate::native::vulkan1_3::{FormatFeatureFlagBits2, FormatFeatureFlags2, FormatProperties3};
///See [`FormatFeatureFlags2`]
#[doc(alias = "VkFormatFeatureFlags2KHR")]
pub type FormatFeatureFlags2KHR = FormatFeatureFlags2;
///See [`FormatFeatureFlagBits2`]
#[doc(alias = "VkFormatFeatureFlagBits2KHR")]
pub type FormatFeatureFlagBits2KHR = FormatFeatureFlagBits2;
///See [`FormatProperties3`]
#[doc(alias = "VkFormatProperties3KHR")]
pub type FormatProperties3KHR = FormatProperties3;
pub use crate::common::extensions::khr_format_feature_flags2::{
    KHR_FORMAT_FEATURE_FLAGS_2_EXTENSION_NAME, KHR_FORMAT_FEATURE_FLAGS_2_SPEC_VERSION,
};
