#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_calibrated_timestamps");
///[VkTimeDomainEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkTimeDomainEXT.html) - Supported time domains
///# C Specifications
///The set of supported time domains consists of:
///```c
///// Provided by VK_EXT_calibrated_timestamps
///typedef enum VkTimeDomainEXT {
///    VK_TIME_DOMAIN_DEVICE_EXT = 0,
///    VK_TIME_DOMAIN_CLOCK_MONOTONIC_EXT = 1,
///    VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_EXT = 2,
///    VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_EXT = 3,
///} VkTimeDomainEXT;
///```
///# Description
/// - [`TIME_DOMAIN_DEVICE`] specifies the device time domain.
///Timestamp values in this time domain use the same units and are
///comparable with device timestamp values captured using
///[`CmdWriteTimestamp`]
///or [`CmdWriteTimestamp2`]
///and are defined to be incrementing according to the
///[timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.
/// - [`TIME_DOMAIN_CLOCK_MONOTONIC`] specifies the CLOCK_MONOTONIC
///time domain available on POSIX platforms.
///Timestamp values in this time domain are in units of nanoseconds and are
///comparable with platform timestamp values captured using the POSIX
///clock_gettime API as computed by this example:
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`TIME_DOMAIN_CLOCK_MONOTONIC_RAW`] specifies the
///CLOCK_MONOTONIC_RAW time domain available on POSIX platforms.
///Timestamp values in this time domain are in units of nanoseconds and are
///comparable with platform timestamp values captured using the POSIX
///clock_gettime API as computed by this example:
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC_RAW, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER`] specifies the
///performance counter (QPC) time domain available on Windows.
///Timestamp values in this time domain are in the same units as those
///provided by the Windows QueryPerformanceCounter API and are comparable
///with platform timestamp values captured using that API as computed by
///this example:
///```c
///LARGE_INTEGER counter;
///QueryPerformanceCounter(&counter);
///return counter.QuadPart;
///```
///# Related
/// - [`VK_EXT_calibrated_timestamps`]
/// - [`CalibratedTimestampInfoEXT`]
/// - [`GetPhysicalDeviceCalibrateableTimeDomainsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkTimeDomainEXT")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct TimeDomainEXT(i32);
impl const Default for TimeDomainEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for TimeDomainEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("TimeDomainEXT")
            .field(match *self {
                Self::TIME_DOMAIN_DEVICE => &"TIME_DOMAIN_DEVICE",
                Self::TIME_DOMAIN_CLOCK_MONOTONIC => &"TIME_DOMAIN_CLOCK_MONOTONIC",
                Self::TIME_DOMAIN_CLOCK_MONOTONIC_RAW => &"TIME_DOMAIN_CLOCK_MONOTONIC_RAW",
                Self::TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER => &"TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER",
                other => unreachable!("invalid value for `TimeDomainEXT`: {:?}", other),
            })
            .finish()
    }
}
impl TimeDomainEXT {
    ///[`TIME_DOMAIN_DEVICE`] specifies the device time domain.
    ///Timestamp values in this time domain use the same units and are
    ///comparable with device timestamp values captured using
    ///[`CmdWriteTimestamp`]
    ///or [`CmdWriteTimestamp2`]
    ///and are defined to be incrementing according to the
    ///[timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.
    pub const TIME_DOMAIN_DEVICE: Self = Self(0);
    ///[`TIME_DOMAIN_CLOCK_MONOTONIC`] specifies the CLOCK_MONOTONIC
    ///time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    pub const TIME_DOMAIN_CLOCK_MONOTONIC: Self = Self(1);
    ///[`TIME_DOMAIN_CLOCK_MONOTONIC_RAW`] specifies the
    ///CLOCK_MONOTONIC_RAW time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    pub const TIME_DOMAIN_CLOCK_MONOTONIC_RAW: Self = Self(2);
    ///[`TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER`] specifies the
    ///performance counter (QPC) time domain available on Windows.
    ///Timestamp values in this time domain are in the same units as those
    ///provided by the Windows QueryPerformanceCounter API and are comparable
    ///with platform timestamp values captured using that API as computed by
    ///this example:
    pub const TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER: Self = Self(3);
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self.0
    }
}
