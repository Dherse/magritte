use crate::native::vulkan1_0::{BaseInStructure, StructureType};
#[doc(alias = "VkValidationFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationFeaturesEXT {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "enabledValidationFeatureCount")]
    pub enabled_validation_feature_count: u32,
    #[doc(alias = "pEnabledValidationFeatures")]
    pub enabled_validation_features: *const ValidationFeatureEnableEXT,
    #[doc(alias = "disabledValidationFeatureCount")]
    pub disabled_validation_feature_count: u32,
    #[doc(alias = "pDisabledValidationFeatures")]
    pub disabled_validation_features: *const ValidationFeatureDisableEXT,
}
impl Default for ValidationFeaturesEXT {
    fn default() -> Self {
        Self {
            s_type: StructureType::ValidationFeaturesExt,
            p_next: unsafe { std::mem::zeroed() },
            enabled_validation_feature_count: unsafe { std::mem::zeroed() },
            enabled_validation_features: unsafe { std::mem::zeroed() },
            disabled_validation_feature_count: unsafe { std::mem::zeroed() },
            disabled_validation_features: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::ext_validation_features::{
    ValidationFeatureDisableEXT, ValidationFeatureEnableEXT, EXT_VALIDATION_FEATURES_EXTENSION_NAME,
    EXT_VALIDATION_FEATURES_SPEC_VERSION,
};
