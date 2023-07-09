use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME")]
pub const NV_FRAGMENT_SHADING_RATE_ENUMS_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_fragment_shading_rate_enums");
#[doc(alias = "VkFragmentShadingRateNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FragmentShadingRateNV {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV")]
    N1InvocationPerPixel = 0,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV")]
    N1InvocationPer1x2Pixels = 1,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV")]
    N1InvocationPer2x1Pixels = 4,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV")]
    N1InvocationPer2x2Pixels = 5,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV")]
    N1InvocationPer2x4Pixels = 6,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV")]
    N1InvocationPer4x2Pixels = 9,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV")]
    N1InvocationPer4x4Pixels = 10,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV")]
    N2InvocationsPerPixel = 11,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV")]
    N4InvocationsPerPixel = 12,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV")]
    N8InvocationsPerPixel = 13,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV")]
    N16InvocationsPerPixel = 14,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV")]
    NoInvocations = 15,
}
impl Default for FragmentShadingRateNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FragmentShadingRateNV {
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
            x if x == Self::N1InvocationPerPixel.bits() => Some(Self::N1InvocationPerPixel),
            x if x == Self::N1InvocationPer1x2Pixels.bits() => Some(Self::N1InvocationPer1x2Pixels),
            x if x == Self::N1InvocationPer2x1Pixels.bits() => Some(Self::N1InvocationPer2x1Pixels),
            x if x == Self::N1InvocationPer2x2Pixels.bits() => Some(Self::N1InvocationPer2x2Pixels),
            x if x == Self::N1InvocationPer2x4Pixels.bits() => Some(Self::N1InvocationPer2x4Pixels),
            x if x == Self::N1InvocationPer4x2Pixels.bits() => Some(Self::N1InvocationPer4x2Pixels),
            x if x == Self::N1InvocationPer4x4Pixels.bits() => Some(Self::N1InvocationPer4x4Pixels),
            x if x == Self::N2InvocationsPerPixel.bits() => Some(Self::N2InvocationsPerPixel),
            x if x == Self::N4InvocationsPerPixel.bits() => Some(Self::N4InvocationsPerPixel),
            x if x == Self::N8InvocationsPerPixel.bits() => Some(Self::N8InvocationsPerPixel),
            x if x == Self::N16InvocationsPerPixel.bits() => Some(Self::N16InvocationsPerPixel),
            x if x == Self::NoInvocations.bits() => Some(Self::NoInvocations),
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
unsafe impl crate::conv::IntoLowLevel for FragmentShadingRateNV {
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
unsafe impl crate::conv::FromLowLevel for FragmentShadingRateNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkFragmentShadingRateTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FragmentShadingRateTypeNV {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV")]
    FragmentSize = 0,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV")]
    Enums = 1,
}
impl Default for FragmentShadingRateTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FragmentShadingRateTypeNV {
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
            x if x == Self::FragmentSize.bits() => Some(Self::FragmentSize),
            x if x == Self::Enums.bits() => Some(Self::Enums),
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
unsafe impl crate::conv::IntoLowLevel for FragmentShadingRateTypeNV {
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
unsafe impl crate::conv::FromLowLevel for FragmentShadingRateTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
