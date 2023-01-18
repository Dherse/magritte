use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Device, PhysicalDevice, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "timeDomain")]
    time_domain: TimeDomainEXT,
}
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_calibrated_timestamps");
#[doc(alias = "VkTimeDomainEXT")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct TimeDomainEXT(i32);
impl TimeDomainEXT {
    #[doc(alias = "VK_TIME_DOMAIN_DEVICE_EXT")]
    pub const DEVICE: Self = Self(0);
    #[doc(alias = "VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT")]
    pub const CLOCK_MONOTONIC: Self = Self(1);
    #[doc(alias = "VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT")]
    pub const CLOCK_MONOTONIC_RAW: Self = Self(2);
    #[doc(alias = "VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT")]
    pub const QUERY_PERFORMANCE_COUNTER: Self = Self(3);
    ///Default empty flags
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }
    ///The bits of this variant
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
    ///Builds a bitmask from the bits of this variant
    #[inline]
    pub const fn from_bits(bits: i32) -> Option<Self> {
        match bits {
            x if x == Self::DEVICE.bits() => Some(Self(x)),
            x if x == Self::CLOCK_MONOTONIC.bits() => Some(Self(x)),
            x if x == Self::CLOCK_MONOTONIC_RAW.bits() => Some(Self(x)),
            x if x == Self::QUERY_PERFORMANCE_COUNTER.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
pub type FNGetPhysicalDeviceCalibrateableTimeDomainsExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_time_domain_count: *mut u32,
    p_time_domains: *mut TimeDomainEXT,
) -> VulkanResultCodes;
#[doc(alias = "vkGetCalibratedTimestampsEXT")]
pub type FNGetCalibratedTimestampsExt = unsafe extern "system" fn(
    device: Device,
    timestamp_count: u32,
    p_timestamp_infos: *const CalibratedTimestampInfoEXT,
    p_timestamps: *mut u64,
    p_max_deviation: *mut u64,
) -> VulkanResultCodes;
