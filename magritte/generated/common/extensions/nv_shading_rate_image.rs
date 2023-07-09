use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkCoarseSampleLocationNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct CoarseSampleLocationNV {
    #[doc(alias = "pixelX")]
    pub pixel_x: u32,
    #[doc(alias = "pixelY")]
    pub pixel_y: u32,
    pub sample: u32,
}
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_SPEC_VERSION")]
pub const NV_SHADING_RATE_IMAGE_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_NV_SHADING_RATE_IMAGE_EXTENSION_NAME")]
pub const NV_SHADING_RATE_IMAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_shading_rate_image");
#[doc(alias = "VkShadingRatePaletteEntryNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ShadingRatePaletteEntryNV {
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV")]
    NoInvocations = 0,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV")]
    N16InvocationsPerPixel = 1,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV")]
    N8InvocationsPerPixel = 2,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV")]
    N4InvocationsPerPixel = 3,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV")]
    N2InvocationsPerPixel = 4,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV")]
    N1InvocationPerPixel = 5,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV")]
    N1InvocationPer2x1Pixels = 6,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV")]
    N1InvocationPer1x2Pixels = 7,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV")]
    N1InvocationPer2x2Pixels = 8,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV")]
    N1InvocationPer4x2Pixels = 9,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV")]
    N1InvocationPer2x4Pixels = 10,
    #[doc(alias = "VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV")]
    N1InvocationPer4x4Pixels = 11,
}
impl Default for ShadingRatePaletteEntryNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ShadingRatePaletteEntryNV {
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
            x if x == Self::NoInvocations.bits() => Some(Self::NoInvocations),
            x if x == Self::N16InvocationsPerPixel.bits() => Some(Self::N16InvocationsPerPixel),
            x if x == Self::N8InvocationsPerPixel.bits() => Some(Self::N8InvocationsPerPixel),
            x if x == Self::N4InvocationsPerPixel.bits() => Some(Self::N4InvocationsPerPixel),
            x if x == Self::N2InvocationsPerPixel.bits() => Some(Self::N2InvocationsPerPixel),
            x if x == Self::N1InvocationPerPixel.bits() => Some(Self::N1InvocationPerPixel),
            x if x == Self::N1InvocationPer2x1Pixels.bits() => Some(Self::N1InvocationPer2x1Pixels),
            x if x == Self::N1InvocationPer1x2Pixels.bits() => Some(Self::N1InvocationPer1x2Pixels),
            x if x == Self::N1InvocationPer2x2Pixels.bits() => Some(Self::N1InvocationPer2x2Pixels),
            x if x == Self::N1InvocationPer4x2Pixels.bits() => Some(Self::N1InvocationPer4x2Pixels),
            x if x == Self::N1InvocationPer2x4Pixels.bits() => Some(Self::N1InvocationPer2x4Pixels),
            x if x == Self::N1InvocationPer4x4Pixels.bits() => Some(Self::N1InvocationPer4x4Pixels),
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
unsafe impl crate::conv::IntoLowLevel for ShadingRatePaletteEntryNV {
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
unsafe impl crate::conv::FromLowLevel for ShadingRatePaletteEntryNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VkCoarseSampleOrderTypeNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum CoarseSampleOrderTypeNV {
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV")]
    Default = 0,
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV")]
    Custom = 1,
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV")]
    PixelMajor = 2,
    #[doc(alias = "VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV")]
    SampleMajor = 3,
}
impl Default for CoarseSampleOrderTypeNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl CoarseSampleOrderTypeNV {
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
            x if x == Self::Default.bits() => Some(Self::Default),
            x if x == Self::Custom.bits() => Some(Self::Custom),
            x if x == Self::PixelMajor.bits() => Some(Self::PixelMajor),
            x if x == Self::SampleMajor.bits() => Some(Self::SampleMajor),
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
unsafe impl crate::conv::IntoLowLevel for CoarseSampleOrderTypeNV {
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
unsafe impl crate::conv::FromLowLevel for CoarseSampleOrderTypeNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
