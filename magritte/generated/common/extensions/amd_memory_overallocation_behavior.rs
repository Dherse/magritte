use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME")]
pub const AMD_MEMORY_OVERALLOCATION_BEHAVIOR_EXTENSION_NAME: &'static CStr =
    cstr!("VK_AMD_memory_overallocation_behavior");
#[doc(alias = "VkMemoryOverallocationBehaviorAMD")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum MemoryOverallocationBehaviorAMD {
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD")]
    Default = 0,
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD")]
    Allowed = 1,
    #[doc(alias = "VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD")]
    Disallowed = 2,
}
impl Default for MemoryOverallocationBehaviorAMD {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl MemoryOverallocationBehaviorAMD {
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
unsafe impl crate::conv::IntoLowLevel for MemoryOverallocationBehaviorAMD {
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
unsafe impl crate::conv::FromLowLevel for MemoryOverallocationBehaviorAMD {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
