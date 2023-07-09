use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const KHR_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const KHR_GLOBAL_PRIORITY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_global_priority");
#[doc(alias = "VkQueueGlobalPriorityKHR")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum QueueGlobalPriorityKHR {
    Empty = 0,
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_LOW_KHR")]
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_LOW_EXT")]
    Low = 128,
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_KHR")]
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_MEDIUM_EXT")]
    Medium = 256,
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_HIGH_KHR")]
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_HIGH_EXT")]
    High = 512,
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_REALTIME_KHR")]
    #[doc(alias = "VK_QUEUE_GLOBAL_PRIORITY_REALTIME_EXT")]
    Realtime = 1024,
}
impl Default for QueueGlobalPriorityKHR {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl QueueGlobalPriorityKHR {
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
            x if x == Self::Low.bits() => Some(Self::Low),
            x if x == Self::Medium.bits() => Some(Self::Medium),
            x if x == Self::High.bits() => Some(Self::High),
            x if x == Self::Realtime.bits() => Some(Self::Realtime),
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
unsafe impl crate::conv::IntoLowLevel for QueueGlobalPriorityKHR {
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
unsafe impl crate::conv::FromLowLevel for QueueGlobalPriorityKHR {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
