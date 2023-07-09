use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_SPEC_VERSION")]
pub const EXT_LINE_RASTERIZATION_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_LINE_RASTERIZATION_EXTENSION_NAME")]
pub const EXT_LINE_RASTERIZATION_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_line_rasterization");
#[doc(alias = "VkLineRasterizationModeEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum LineRasterizationModeEXT {
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_DEFAULT_EXT")]
    Default = 0,
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_EXT")]
    Rectangular = 1,
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_BRESENHAM_EXT")]
    Bresenham = 2,
    #[doc(alias = "VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH_EXT")]
    RectangularSmooth = 3,
}
impl Default for LineRasterizationModeEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl LineRasterizationModeEXT {
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
            x if x == Self::Rectangular.bits() => Some(Self::Rectangular),
            x if x == Self::Bresenham.bits() => Some(Self::Bresenham),
            x if x == Self::RectangularSmooth.bits() => Some(Self::RectangularSmooth),
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
unsafe impl crate::conv::IntoLowLevel for LineRasterizationModeEXT {
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
unsafe impl crate::conv::FromLowLevel for LineRasterizationModeEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
