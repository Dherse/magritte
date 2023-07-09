use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_SPEC_VERSION")]
pub const EXT_VALIDATION_FEATURES_SPEC_VERSION: u32 = 5;
#[doc(alias = "VK_EXT_VALIDATION_FEATURES_EXTENSION_NAME")]
pub const EXT_VALIDATION_FEATURES_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_validation_features");
#[doc(alias = "VkValidationFeatureEnableEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ValidationFeatureEnableEXT {
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT")]
    GpuAssisted = 0,
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT")]
    GpuAssistedReserveBindingSlot = 1,
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT")]
    BestPractices = 2,
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT")]
    DebugPrintf = 3,
    #[doc(alias = "VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT")]
    SynchronizationValidation = 4,
}
impl Default for ValidationFeatureEnableEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ValidationFeatureEnableEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::GpuAssisted.bits() => Some(Self::GpuAssisted),
            x if x == Self::GpuAssistedReserveBindingSlot.bits() => Some(Self::GpuAssistedReserveBindingSlot),
            x if x == Self::BestPractices.bits() => Some(Self::BestPractices),
            x if x == Self::DebugPrintf.bits() => Some(Self::DebugPrintf),
            x if x == Self::SynchronizationValidation.bits() => Some(Self::SynchronizationValidation),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ValidationFeatureEnableEXT {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationFeatureEnableEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkValidationFeatureDisableEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ValidationFeatureDisableEXT {
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_ALL_EXT")]
    All = 0,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT")]
    Shaders = 1,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT")]
    ThreadSafety = 2,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT")]
    ApiParameters = 3,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT")]
    ObjectLifetimes = 4,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT")]
    CoreChecks = 5,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT")]
    UniqueHandles = 6,
    #[doc(alias = "VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT")]
    ShaderValidationCache = 7,
}
impl Default for ValidationFeatureDisableEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ValidationFeatureDisableEXT {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        unsafe { Self::from_bits_unchecked(0) }
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        *self as i32
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::All.bits() => Some(Self::All),
            x if x == Self::Shaders.bits() => Some(Self::Shaders),
            x if x == Self::ThreadSafety.bits() => Some(Self::ThreadSafety),
            x if x == Self::ApiParameters.bits() => Some(Self::ApiParameters),
            x if x == Self::ObjectLifetimes.bits() => Some(Self::ObjectLifetimes),
            x if x == Self::CoreChecks.bits() => Some(Self::CoreChecks),
            x if x == Self::UniqueHandles.bits() => Some(Self::UniqueHandles),
            x if x == Self::ShaderValidationCache.bits() => Some(Self::ShaderValidationCache),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        std::mem::transmute(bits)
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for ValidationFeatureDisableEXT {
    type LowLevel = Self;
    unsafe fn into_low_level(
        &self,
        context: &std::sync::Arc<crate::context::Context>,
        bump: &bumpalo::Bump,
    ) -> Self::LowLevel {
        *self
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::FromLowLevel for ValidationFeatureDisableEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
