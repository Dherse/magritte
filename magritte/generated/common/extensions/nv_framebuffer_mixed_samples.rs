use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkPipelineCoverageModulationStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineCoverageModulationStateCreateFlagsNV(u32);
impl PipelineCoverageModulationStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for PipelineCoverageModulationStateCreateFlagsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineCoverageModulationStateCreateFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for PipelineCoverageModulationStateCreateFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME")]
pub const NV_FRAMEBUFFER_MIXED_SAMPLES_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_framebuffer_mixed_samples");
#[doc(alias = "VkCoverageModulationModeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CoverageModulationModeNV {
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_NONE_NV")]
    None = 0,
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_RGB_NV")]
    Rgb = 1,
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_ALPHA_NV")]
    Alpha = 2,
    #[doc(alias = "VK_COVERAGE_MODULATION_MODE_RGBA_NV")]
    Rgba = 3,
}
impl Default for CoverageModulationModeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl CoverageModulationModeNV {
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
            x if x == Self::None.bits() => Some(Self::None),
            x if x == Self::Rgb.bits() => Some(Self::Rgb),
            x if x == Self::Alpha.bits() => Some(Self::Alpha),
            x if x == Self::Rgba.bits() => Some(Self::Rgba),
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
unsafe impl crate::conv::IntoLowLevel for CoverageModulationModeNV {
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
unsafe impl crate::conv::FromLowLevel for CoverageModulationModeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
