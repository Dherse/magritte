use crate::cstr;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_calibrated_timestamps");
#[doc(alias = "VkTimeDomainEXT")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub enum TimeDomainEXT {
    #[doc(alias = "VK_TIME_DOMAIN_DEVICE_EXT")]
    Device = 0,
    #[doc(alias = "VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT")]
    ClockMonotonic = 1,
    #[doc(alias = "VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT")]
    ClockMonotonicRaw = 2,
    #[doc(alias = "VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT")]
    QueryPerformanceCounter = 3,
}
impl Default for TimeDomainEXT {
    #[inline]
    fn default() -> Self {
        Self::empty()
    }
}
impl TimeDomainEXT {
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
            x if x == Self::Device.bits() => Some(Self::Device),
            x if x == Self::ClockMonotonic.bits() => Some(Self::ClockMonotonic),
            x if x == Self::ClockMonotonicRaw.bits() => Some(Self::ClockMonotonicRaw),
            x if x == Self::QueryPerformanceCounter.bits() => Some(Self::QueryPerformanceCounter),
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
unsafe impl crate::conv::IntoLowLevel for TimeDomainEXT {
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
unsafe impl crate::conv::FromLowLevel for TimeDomainEXT {
    unsafe fn from_low_level(
        context: &std::sync::Arc<crate::context::Context>,
        value: <Self as crate::conv::IntoLowLevel>::LowLevel,
    ) -> Self {
        value
    }
}
