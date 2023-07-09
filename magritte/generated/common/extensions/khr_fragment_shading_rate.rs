use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION")]
pub const KHR_FRAGMENT_SHADING_RATE_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME")]
pub const KHR_FRAGMENT_SHADING_RATE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_fragment_shading_rate");
#[doc(alias = "VkFragmentShadingRateCombinerOpKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FragmentShadingRateCombinerOpKHR {
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR")]
    Keep = 0,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR")]
    Replace = 1,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR")]
    Min = 2,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR")]
    Max = 3,
    #[doc(alias = "VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR")]
    Mul = 4,
}
impl Default for FragmentShadingRateCombinerOpKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FragmentShadingRateCombinerOpKHR {
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
            x if x == Self::Keep.bits() => Some(Self::Keep),
            x if x == Self::Replace.bits() => Some(Self::Replace),
            x if x == Self::Min.bits() => Some(Self::Min),
            x if x == Self::Max.bits() => Some(Self::Max),
            x if x == Self::Mul.bits() => Some(Self::Mul),
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
unsafe impl crate::conv::IntoLowLevel for FragmentShadingRateCombinerOpKHR {
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
unsafe impl crate::conv::FromLowLevel for FragmentShadingRateCombinerOpKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
