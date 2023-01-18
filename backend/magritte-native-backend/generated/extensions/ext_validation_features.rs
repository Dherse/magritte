use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, StructureType},
};
use std::ffi::CStr;
#[doc(alias = "VkValidationFeaturesEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ValidationFeaturesEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "enabledValidationFeatureCount")]
    enabled_validation_feature_count: u32,
    #[doc(alias = "pEnabledValidationFeatures")]
    enabled_validation_features: *const ValidationFeatureEnableEXT,
    #[doc(alias = "disabledValidationFeatureCount")]
    disabled_validation_feature_count: u32,
    #[doc(alias = "pDisabledValidationFeatures")]
    disabled_validation_features: *const ValidationFeatureDisableEXT,
}
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_SPEC_VERSION")]
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 5;
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME")]
pub const EXT_VALIDATION_FEATURES_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_validation_features");
#[doc(alias = "VkValidationFeatureEnableEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ValidationFeatureEnableEXT(i32);
impl ValidationFeatureEnableEXT {
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT")]
    pub const GPU_ASSISTED: Self = Self(0);
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT")]
    pub const GPU_ASSISTED_RESERVE_BINDING_SLOT: Self = Self(1);
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT")]
    pub const BEST_PRACTICES: Self = Self(2);
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT")]
    pub const DEBUG_PRINTF: Self = Self(3);
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT")]
    pub const SYNCHRONIZATION_VALIDATION: Self = Self(4);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::GPU_ASSISTED.bits() => Some(Self(x)),
            x if x == Self::GPU_ASSISTED_RESERVE_BINDING_SLOT.bits() => Some(Self(x)),
            x if x == Self::BEST_PRACTICES.bits() => Some(Self(x)),
            x if x == Self::DEBUG_PRINTF.bits() => Some(Self(x)),
            x if x == Self::SYNCHRONIZATION_VALIDATION.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "VkValidationFeatureDisableEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct ValidationFeatureDisableEXT(i32);
impl ValidationFeatureDisableEXT {
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_ALL_EXT")]
    pub const ALL: Self = Self(0);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT")]
    pub const SHADERS: Self = Self(1);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT")]
    pub const THREAD_SAFETY: Self = Self(2);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT")]
    pub const API_PARAMETERS: Self = Self(3);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT")]
    pub const OBJECT_LIFETIMES: Self = Self(4);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT")]
    pub const CORE_CHECKS: Self = Self(5);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT")]
    pub const UNIQUE_HANDLES: Self = Self(6);
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT")]
    pub const SHADER_VALIDATION_CACHE: Self = Self(7);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::ALL.bits() => Some(Self(x)),
            x if x == Self::SHADERS.bits() => Some(Self(x)),
            x if x == Self::THREAD_SAFETY.bits() => Some(Self(x)),
            x if x == Self::API_PARAMETERS.bits() => Some(Self(x)),
            x if x == Self::OBJECT_LIFETIMES.bits() => Some(Self(x)),
            x if x == Self::CORE_CHECKS.bits() => Some(Self(x)),
            x if x == Self::UNIQUE_HANDLES.bits() => Some(Self(x)),
            x if x == Self::SHADER_VALIDATION_CACHE.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
