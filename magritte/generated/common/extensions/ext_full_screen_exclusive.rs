use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME")]
pub const EXT_FULL_SCREEN_EXCLUSIVE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_full_screen_exclusive");
#[doc(alias = "VkFullScreenExclusiveEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum FullScreenExclusiveEXT {
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT")]
    Default = 0,
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT")]
    Allowed = 1,
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT")]
    Disallowed = 2,
    #[doc(alias = "VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT")]
    ApplicationControlled = 3,
}
impl Default for FullScreenExclusiveEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl FullScreenExclusiveEXT {
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
            x if x == Self::Allowed.bits() => Some(Self::Allowed),
            x if x == Self::Disallowed.bits() => Some(Self::Disallowed),
            x if x == Self::ApplicationControlled.bits() => Some(Self::ApplicationControlled),
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
unsafe impl crate::conv::IntoLowLevel for FullScreenExclusiveEXT {
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
unsafe impl crate::conv::FromLowLevel for FullScreenExclusiveEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
