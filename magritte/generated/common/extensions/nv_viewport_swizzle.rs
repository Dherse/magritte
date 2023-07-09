use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VkViewportSwizzleNV")]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct ViewportSwizzleNV {
    pub x: ViewportCoordinateSwizzleNV,
    pub y: ViewportCoordinateSwizzleNV,
    pub z: ViewportCoordinateSwizzleNV,
    pub w: ViewportCoordinateSwizzleNV,
}
#[doc(alias = "VkPipelineViewportSwizzleStateCreateFlagsNV")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PipelineViewportSwizzleStateCreateFlagsNV(u32);
impl PipelineViewportSwizzleStateCreateFlagsNV {
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
}
impl Default for PipelineViewportSwizzleStateCreateFlagsNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
#[cfg(feature = "native")]
unsafe impl crate::conv::IntoLowLevel for PipelineViewportSwizzleStateCreateFlagsNV {
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
unsafe impl crate::conv::FromLowLevel for PipelineViewportSwizzleStateCreateFlagsNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_SPEC_VERSION")]
pub const NV_VIEWPORT_SWIZZLE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_VIEWPORT_SWIZZLE_EXTENSION_NAME")]
pub const NV_VIEWPORT_SWIZZLE_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_viewport_swizzle");
#[doc(alias = "VkViewportCoordinateSwizzleNV")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum ViewportCoordinateSwizzleNV {
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV")]
    PositiveX = 0,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV")]
    NegativeX = 1,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV")]
    PositiveY = 2,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV")]
    NegativeY = 3,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV")]
    PositiveZ = 4,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV")]
    NegativeZ = 5,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV")]
    PositiveW = 6,
    #[doc(alias = "VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV")]
    NegativeW = 7,
}
impl Default for ViewportCoordinateSwizzleNV {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl ViewportCoordinateSwizzleNV {
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
            x if x == Self::PositiveX.bits() => Some(Self::PositiveX),
            x if x == Self::NegativeX.bits() => Some(Self::NegativeX),
            x if x == Self::PositiveY.bits() => Some(Self::PositiveY),
            x if x == Self::NegativeY.bits() => Some(Self::NegativeY),
            x if x == Self::PositiveZ.bits() => Some(Self::PositiveZ),
            x if x == Self::NegativeZ.bits() => Some(Self::NegativeZ),
            x if x == Self::PositiveW.bits() => Some(Self::PositiveW),
            x if x == Self::NegativeW.bits() => Some(Self::NegativeW),
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
unsafe impl crate::conv::IntoLowLevel for ViewportCoordinateSwizzleNV {
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
unsafe impl crate::conv::FromLowLevel for ViewportCoordinateSwizzleNV {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
