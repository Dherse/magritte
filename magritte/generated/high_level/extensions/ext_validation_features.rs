pub use crate::common::extensions::ext_validation_features::{
    ValidationFeatureDisableEXT, ValidationFeatureEnableEXT, EXT_VALIDATION_FEATURES_EXTENSION_NAME,
    EXT_VALIDATION_FEATURES_SPEC_VERSION,
};
use crate::{context::Context, vulkan1_0::StructureType};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::Arc;
#[doc(alias = "VkValidationFeaturesEXT")]
#[derive(Clone, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationFeaturesEXT {
    #[doc(alias = "pEnabledValidationFeatures")]
    pub enabled_validation_features: SmallVec<[ValidationFeatureEnableEXT; 8]>,
    #[doc(alias = "pDisabledValidationFeatures")]
    pub disabled_validation_features: SmallVec<[ValidationFeatureDisableEXT; 8]>,
}
impl ValidationFeaturesEXT {
    ///Get a reference to the `enabled_validation_features` field.
    pub fn enabled_validation_features(&self) -> &SmallVec<[ValidationFeatureEnableEXT; 8]> {
        &self.enabled_validation_features
    }
    ///Get a reference to the `disabled_validation_features` field.
    pub fn disabled_validation_features(&self) -> &SmallVec<[ValidationFeatureDisableEXT; 8]> {
        &self.disabled_validation_features
    }
    ///Get a mutable reference to the `enabled_validation_features` field.
    pub fn enabled_validation_features_mut(&mut self) -> &mut SmallVec<[ValidationFeatureEnableEXT; 8]> {
        &mut self.enabled_validation_features
    }
    ///Get a mutable reference to the `disabled_validation_features` field.
    pub fn disabled_validation_features_mut(&mut self) -> &mut SmallVec<[ValidationFeatureDisableEXT; 8]> {
        &mut self.disabled_validation_features
    }
    ///Sets the `enabled_validation_features` field.
    pub fn set_enabled_validation_features(
        &mut self,
        enabled_validation_features: SmallVec<[ValidationFeatureEnableEXT; 8]>,
    ) -> &mut Self {
        self.enabled_validation_features = enabled_validation_features;
        self
    }
    ///Sets the `disabled_validation_features` field.
    pub fn set_disabled_validation_features(
        &mut self,
        disabled_validation_features: SmallVec<[ValidationFeatureDisableEXT; 8]>,
    ) -> &mut Self {
        self.disabled_validation_features = disabled_validation_features;
        self
    }
    ///Sets the `enabled_validation_features` field in a builder way.
    pub fn with_enabled_validation_features(
        mut self,
        enabled_validation_features: SmallVec<[ValidationFeatureEnableEXT; 8]>,
    ) -> Self {
        self.enabled_validation_features = enabled_validation_features;
        self
    }
    ///Sets the `disabled_validation_features` field in a builder way.
    pub fn with_disabled_validation_features(
        mut self,
        disabled_validation_features: SmallVec<[ValidationFeatureDisableEXT; 8]>,
    ) -> Self {
        self.disabled_validation_features = disabled_validation_features;
        self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ValidationFeaturesEXT {
    type LowLevel = crate::native::extensions::ext_validation_features::ValidationFeaturesEXT;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        let len_enabled_validation_features = self.enabled_validation_features.len() as u32;
        let enabled_validation_features = bump
            .alloc_slice_fill_iter(
                self.enabled_validation_features
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        let len_disabled_validation_features = self.disabled_validation_features.len() as u32;
        let disabled_validation_features = bump
            .alloc_slice_fill_iter(
                self.disabled_validation_features
                    .iter()
                    .map(|x| x.into_low_level(context, bump)),
            )
            .as_ptr()
            .cast();
        crate::native::extensions::ext_validation_features::ValidationFeaturesEXT {
            s_type: StructureType::ValidationFeaturesExt,
            p_next: std::ptr::null(),
            enabled_validation_feature_count: len_enabled_validation_features,
            enabled_validation_features: enabled_validation_features,
            disabled_validation_feature_count: len_disabled_validation_features,
            disabled_validation_features: disabled_validation_features,
        }
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationFeaturesEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        let enabled_validation_features_len = value.enabled_validation_feature_count;
        let mut enabled_validation_features = SmallVec::with_capacity(enabled_validation_features_len as usize);
        for i in 0..enabled_validation_features_len {
            enabled_validation_features.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.enabled_validation_features.add(i as usize).read(),
            ));
        }
        let disabled_validation_features_len = value.disabled_validation_feature_count;
        let mut disabled_validation_features = SmallVec::with_capacity(disabled_validation_features_len as usize);
        for i in 0..disabled_validation_features_len {
            disabled_validation_features.push(crate::conv::FromLowLevel::from_low_level(
                context,
                value.disabled_validation_features.add(i as usize).read(),
            ));
        }
        Self {
            enabled_validation_features: enabled_validation_features,
            disabled_validation_features: disabled_validation_features,
        }
    }
}
