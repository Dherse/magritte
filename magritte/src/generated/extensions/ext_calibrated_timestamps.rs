//![VK_EXT_calibrated_timestamps](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html) - device extension
//!# Description
//!This extension provides an interface to query calibrated timestamps obtained
//!quasi simultaneously from two time domains.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_calibrated_timestamps]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_EXT_calibrated_timestamps extension>>)
//!# New functions & commands
//! - [`GetCalibratedTimestampsEXT`]
//! - [`GetPhysicalDeviceCalibrateableTimeDomainsEXT`]
//!# New structures
//! - [`CalibratedTimestampInfoEXT`]
//!# New enums
//! - [`TimeDomainEXT`]
//!# New constants
//! - [`EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME`]
//! - [`EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION`]
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Is the device timestamp value returned in the same time domain as the
//!timestamp values written by [`CmdWriteTimestamp`]?**RESOLVED**: Yes.2) What time domain is the
//! host timestamp returned in?**RESOLVED**: A query is provided to determine the calibrateable time
//! domains.
//!The expected host time domain used on Windows is that of
//!QueryPerformanceCounter, and on Linux that of CLOCK_MONOTONIC.3) Should we support other time
//! domain combinations than just one host and
//!the device time domain?**RESOLVED**: Supporting that would need the application to query the set
//! of
//!supported time domains, while supporting only one host and the device time
//!domain would only need a query for the host time domain type.
//!The proposed API chooses the general approach for the sake of extensibility.4) Should we use
//! CLOCK_MONOTONIC_RAW instead of CLOCK_MONOTONIC?**RESOLVED**: CLOCK_MONOTONIC is usable in a
//! wider set of situations, however,
//!it is subject to NTP adjustments so some use cases may prefer
//!CLOCK_MONOTONIC_RAW.
//!Thus this extension allows both to be exposed.5) How can the application extrapolate future
//! device timestamp values from
//!the calibrated timestamp value?**RESOLVED**: [`PhysicalDeviceLimits::timestamp_period`] makes it
//!possible to calculate future device timestamps as follows:6) In what queue are timestamp values
//! in time domain
//!`VK_TIME_DOMAIN_DEVICE_EXT` captured by
//![`GetCalibratedTimestampsEXT`]?**RESOLVED**: An implementation supporting this extension will
//! have all its
//!VkQueue share the same time domain.
//!```c
//!futureTimestamp = calibratedTimestamp + deltaNanoseconds / timestampPeriod
//!```
//!6) Can the host and device timestamp values drift apart over longer periods
//!of time?**RESOLVED**: Yes, especially as some time domains by definition allow for
//!that to happen (e.g. CLOCK_MONOTONIC is subject to NTP adjustments).
//!Thus it is recommended that applications re-calibrate from time to time.7) Should we add a query
//! for reporting the maximum deviation of the
//!timestamp values returned by calibrated timestamp queries?**RESOLVED**: A global query seems
//! inappropriate and difficult to enforce.
//!However, it is possible to return the maximum deviation any single
//!calibrated timestamp query can have by sampling one of the time domains
//!twice as follows:
//!```c
//!timestampX = timestampX_before = SampleTimeDomain(X)
//!for each time domain Y != X
//!    timestampY = SampleTimeDomain(Y)
//!timestampX_after = SampleTimeDomain(X)
//!maxDeviation = timestampX_after - timestampX_before
//!```
//!8) Can the maximum deviation reported ever be zero?**RESOLVED**: Unless the tick of each clock
//! corresponding to the set of time
//!domains coincides and all clocks can literally be sampled simutaneously,
//!there is not really a possibility for the maximum deviation to be zero, so
//!by convention the maximum deviation is always at least the maximum of the
//!length of the ticks of the set of time domains calibrated and thus can never
//!be zero.
//!# Version History
//! - Revision 2, 2021-03-16 (Lionel Landwerlin)
//! - Specify requirement on device timestamps
//! - Revision 1, 2018-10-04 (Daniel Rakos)
//! - Internal revisions.
//!# Other info
//! * 2018-10-04
//! * No known IP claims.
//!*
//! - Matthaeus G. Chajdas, AMD
//! - Alan Harrison, AMD
//! - Derrick Owens, AMD
//! - Daniel Rakos, AMD
//! - Jason Ekstrand, Intel
//! - Keith Packard, Valve
//!# Related
//! - [`CalibratedTimestampInfoEXT`]
//! - [`TimeDomainEXT`]
//! - [`GetCalibratedTimestampsEXT`]
//! - [`GetPhysicalDeviceCalibrateableTimeDomainsEXT`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
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
