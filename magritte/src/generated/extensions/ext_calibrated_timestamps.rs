//![VK_EXT_calibrated_timestamps](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_EXT_calibrated_timestamps.html) - device extension
//!# Description
//!This extension provides an interface to query calibrated timestamps obtained
//!quasi simultaneously from two time domains.
//!# Revision
//!2
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`khr_get_physical_device_properties2`]`
//!# Contacts
//! - Daniel Rakos [drakos-amd](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_EXT_calibrated_timestamps]
//!   @drakos-amd%0A<<Here describe the issue or question you have about the
//!   VK_EXT_calibrated_timestamps extension>>)
//!# New functions & commands
//! - [`get_calibrated_timestamps_ext`]
//! - [`get_physical_device_calibrateable_time_domains_ext`]
//!# New structures
//! - [`CalibratedTimestampInfoEXT`]
//!# New enums
//! - [`TimeDomainEXT`]
//!# New constants
//! - [`EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME`]
//! - [`EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION`]
//! - Extending [`StructureType`]:  - `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
//!# Known issues & F.A.Q
//!1) Is the device timestamp value returned in the same time domain as the
//!timestamp values written by [`cmd_write_timestamp`]? **RESOLVED** : Yes.2) What time domain is
//! the host timestamp returned in? **RESOLVED** : A query is provided to determine the
//! calibrateable time domains.
//!The expected host time domain used on Windows is that of
//!QueryPerformanceCounter, and on Linux that of CLOCK_MONOTONIC.3) Should we support other time
//! domain combinations than just one host and
//!the device time domain? **RESOLVED** : Supporting that would need the application to query the
//! set of
//!supported time domains, while supporting only one host and the device time
//!domain would only need a query for the host time domain type.
//!The proposed API chooses the general approach for the sake of extensibility.4) Should we use
//! CLOCK_MONOTONIC_RAW instead of CLOCK_MONOTONIC? **RESOLVED** : CLOCK_MONOTONIC is usable in a
//! wider set of situations, however,
//!it is subject to NTP adjustments so some use cases may prefer
//!CLOCK_MONOTONIC_RAW.
//!Thus this extension allows both to be exposed.5) How can the application extrapolate future
//! device timestamp values from
//!the calibrated timestamp value? **RESOLVED** : [`PhysicalDeviceLimits::timestamp_period`] makes
//! it
//!possible to calculate future device timestamps as follows:6) In what queue are timestamp values
//! in time domain
//!`VK_TIME_DOMAIN_DEVICE_EXT` captured by
//![`get_calibrated_timestamps_ext`]? **RESOLVED** : An implementation supporting this extension
//! will have all its
//!VkQueue share the same time domain.
//!```c
//!futureTimestamp = calibratedTimestamp + deltaNanoseconds / timestampPeriod
//!```
//!6) Can the host and device timestamp values drift apart over longer periods
//!of time? **RESOLVED** : Yes, especially as some time domains by definition allow for
//!that to happen (e.g. CLOCK_MONOTONIC is subject to NTP adjustments).
//!Thus it is recommended that applications re-calibrate from time to time.7) Should we add a query
//! for reporting the maximum deviation of the
//!timestamp values returned by calibrated timestamp queries? **RESOLVED** : A global query seems
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
//!8) Can the maximum deviation reported ever be zero? **RESOLVED** : Unless the tick of each clock
//! corresponding to the set of time
//!domains coincides and all clocks can literally be sampled simutaneously,
//!there is not really a possibility for the maximum deviation to be zero, so
//!by convention the maximum deviation is always at least the maximum of the
//!length of the ticks of the set of time domains calibrated and thus can never
//!be zero.
//!# Version History
//! - Revision 2, 2021-03-16 (Lionel Landwerlin)  - Specify requirement on device timestamps
//! - Revision 1, 2018-10-04 (Daniel Rakos)  - Internal revisions.
//!# Other info
//! * 2018-10-04
//! * No known IP claims.
//! * - Matthaeus G. Chajdas, AMD  - Alan Harrison, AMD  - Derrick Owens, AMD  - Daniel Rakos, AMD
//!   - Jason Ekstrand, Intel  - Keith Packard, Valve
//!# Related
//! - [`CalibratedTimestampInfoEXT`]
//! - [`TimeDomainEXT`]
//! - [`get_calibrated_timestamps_ext`]
//! - [`get_physical_device_calibrateable_time_domains_ext`]
//!
//!# Notes and documentation
//!For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
//!
//!This documentation is generated from the Vulkan specification and documentation.
//!The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
//! Commons Attribution 4.0 International*.
//!This license explicitely allows adapting the source material as long as proper credit is given.
use crate::{
    vulkan1_0::{BaseInStructure, Device, Instance, PhysicalDevice, StructureType, VulkanResultCodes},
    AsRaw, SmallVec, Unique, VulkanResult,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::{ffi::CStr, marker::PhantomData};
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION")]
pub const EXT_CALIBRATED_TIMESTAMPS_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME")]
pub const EXT_CALIBRATED_TIMESTAMPS_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_EXT_calibrated_timestamps");
///[vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) - Query calibrateable time domains
///# C Specifications
///To query the set of time domains for which a physical device supports
///timestamp calibration, call:
///```c
///// Provided by VK_EXT_calibrated_timestamps
///VkResult vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
///    VkPhysicalDevice                            physicalDevice,
///    uint32_t*                                   pTimeDomainCount,
///    VkTimeDomainEXT*                            pTimeDomains);
///```
///# Parameters
/// - [`physical_device`] is the physical device from which to query the set of calibrateable time
///   domains.
/// - [`p_time_domain_count`] is a pointer to an integer related to the number of calibrateable time
///   domains available or queried, as described below.
/// - [`p_time_domains`] is either `NULL` or a pointer to an array of [`TimeDomainEXT`] values,
///   indicating the supported calibrateable time domains.
///# Description
///If [`p_time_domains`] is `NULL`, then the number of calibrateable time
///domains supported for the given [`physical_device`] is returned in
///[`p_time_domain_count`].
///Otherwise, [`p_time_domain_count`] **must**  point to a variable set by the user
///to the number of elements in the [`p_time_domains`] array, and on return the
///variable is overwritten with the number of values actually written to
///[`p_time_domains`].
///If the value of [`p_time_domain_count`] is less than the number of
///calibrateable time domains supported, at most [`p_time_domain_count`] values
///will be written to [`p_time_domains`], and `VK_INCOMPLETE` will be
///returned instead of `VK_SUCCESS`, to indicate that not all the available
///time domains were returned.
///## Valid Usage (Implicit)
/// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
/// - [`p_time_domain_count`] **must**  be a valid pointer to a `uint32_t` value
/// - If the value referenced by [`p_time_domain_count`] is not `0`, and [`p_time_domains`] is not
///   `NULL`, [`p_time_domains`] **must**  be a valid pointer to an array of
///   [`p_time_domain_count`][`TimeDomainEXT`] values
///
///## Return Codes
/// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`ext_calibrated_timestamps`]
/// - [`PhysicalDevice`]
/// - [`TimeDomainEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
pub type FNGetPhysicalDeviceCalibrateableTimeDomainsExt = Option<
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        p_time_domain_count: *mut u32,
        p_time_domains: *mut TimeDomainEXT,
    ) -> VulkanResultCodes,
>;
///[vkGetCalibratedTimestampsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html) - Query calibrated timestamps
///# C Specifications
///In order to be able to correlate the time a particular operation took place
///at on timelines of different time domains (e.g. a device operation vs a host
///operation), Vulkan allows querying calibrated timestamps from multiple time
///domains.To query calibrated timestamps from a set of time domains, call:
///```c
///// Provided by VK_EXT_calibrated_timestamps
///VkResult vkGetCalibratedTimestampsEXT(
///    VkDevice                                    device,
///    uint32_t                                    timestampCount,
///    const VkCalibratedTimestampInfoEXT*         pTimestampInfos,
///    uint64_t*                                   pTimestamps,
///    uint64_t*                                   pMaxDeviation);
///```
///# Parameters
/// - [`device`] is the logical device used to perform the query.
/// - [`timestamp_count`] is the number of timestamps to query.
/// - [`p_timestamp_infos`] is a pointer to an array of
///   [`timestamp_count`][`CalibratedTimestampInfoEXT`] structures, describing the time domains the
///   calibrated timestamps should be captured from.
/// - [`p_timestamps`] is a pointer to an array of [`timestamp_count`] 64-bit unsigned integer
///   values in which the requested calibrated timestamp values are returned.
/// - [`p_max_deviation`] is a pointer to a 64-bit unsigned integer value in which the strictly
///   positive maximum deviation, in nanoseconds, of the calibrated timestamp values is returned.
///# Description
///Calibrated timestamp values  **can**  be extrapolated to estimate future
///coinciding timestamp values, however, depending on the nature of the time
///domains and other properties of the platform extrapolating values over a
///sufficiently long period of time  **may**  no longer be accurate enough to fit
///any particular purpose, so applications are expected to re-calibrate the
///timestamps on a regular basis.
///## Valid Usage (Implicit)
/// - [`device`] **must**  be a valid [`Device`] handle
/// - [`p_timestamp_infos`] **must**  be a valid pointer to an array of [`timestamp_count`] valid
///   [`CalibratedTimestampInfoEXT`] structures
/// - [`p_timestamps`] **must**  be a valid pointer to an array of [`timestamp_count`]`uint64_t`
///   values
/// - [`p_max_deviation`] **must**  be a valid pointer to a `uint64_t` value
/// - [`timestamp_count`] **must**  be greater than `0`
///
///## Return Codes
/// * - `VK_SUCCESS`
/// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
///# Related
/// - [`ext_calibrated_timestamps`]
/// - [`CalibratedTimestampInfoEXT`]
/// - [`Device`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "vkGetCalibratedTimestampsEXT")]
pub type FNGetCalibratedTimestampsExt = Option<
    for<'lt> unsafe extern "system" fn(
        device: Device,
        timestamp_count: u32,
        p_timestamp_infos: *const CalibratedTimestampInfoEXT<'lt>,
        p_timestamps: *mut u64,
        p_max_deviation: *mut u64,
    ) -> VulkanResultCodes,
>;
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
/// - [`DEVICE`] specifies the device time domain. Timestamp values in this time domain use the same
///   units and are comparable with device timestamp values captured using [`cmd_write_timestamp`] or
///   [`cmd_write_timestamp2`] and are defined to be incrementing according to the [timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod)
///   of the device.
/// - [`CLOCK_MONOTONIC`] specifies the CLOCK_MONOTONIC time domain available on POSIX platforms.
///   Timestamp values in this time domain are in units of nanoseconds and are comparable with
///   platform timestamp values captured using the POSIX clock_gettime API as computed by this
///   example:
///
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`CLOCK_MONOTONIC_RAW`] specifies the CLOCK_MONOTONIC_RAW time domain available on POSIX
///   platforms. Timestamp values in this time domain are in units of nanoseconds and are comparable
///   with platform timestamp values captured using the POSIX clock_gettime API as computed by this
///   example:
///
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC_RAW, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`QUERY_PERFORMANCE_COUNTER`] specifies the performance counter (QPC) time domain available on
///   Windows. Timestamp values in this time domain are in the same units as those provided by the
///   Windows QueryPerformanceCounter API and are comparable with platform timestamp values captured
///   using that API as computed by this example:
///
///```c
///LARGE_INTEGER counter;
///QueryPerformanceCounter(&counter);
///return counter.QuadPart;
///```
///# Related
/// - [`ext_calibrated_timestamps`]
/// - [`CalibratedTimestampInfoEXT`]
/// - [`get_physical_device_calibrateable_time_domains_ext`]
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct TimeDomainEXT(i32);
impl const Default for TimeDomainEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl TimeDomainEXT {
    ///[`DEVICE`] specifies the device time domain.
    ///Timestamp values in this time domain use the same units and are
    ///comparable with device timestamp values captured using
    ///[`cmd_write_timestamp`]
    ///or [`cmd_write_timestamp2`]
    ///and are defined to be incrementing according to the
    ///[timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.
    pub const DEVICE: Self = Self(0);
    ///[`CLOCK_MONOTONIC`] specifies the CLOCK_MONOTONIC
    ///time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    pub const CLOCK_MONOTONIC: Self = Self(1);
    ///[`CLOCK_MONOTONIC_RAW`] specifies the
    ///CLOCK_MONOTONIC_RAW time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    pub const CLOCK_MONOTONIC_RAW: Self = Self(2);
    ///[`QUERY_PERFORMANCE_COUNTER`] specifies the
    ///performance counter (QPC) time domain available on Windows.
    ///Timestamp values in this time domain are in the same units as those
    ///provided by the Windows QueryPerformanceCounter API and are comparable
    ///with platform timestamp values captured using that API as computed by
    ///this example:
    pub const QUERY_PERFORMANCE_COUNTER: Self = Self(3);
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
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe.
    ///
    ///# Safety
    ///The caller of this function must ensure that all of the bits are valid.
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
impl std::fmt::Debug for TimeDomainEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple(stringify!(TimeDomainEXT))
            .field(match *self {
                Self::DEVICE => &"DEVICE",
                Self::CLOCK_MONOTONIC => &"CLOCK_MONOTONIC",
                Self::CLOCK_MONOTONIC_RAW => &"CLOCK_MONOTONIC_RAW",
                Self::QUERY_PERFORMANCE_COUNTER => &"QUERY_PERFORMANCE_COUNTER",
                other => unreachable!(concat!("invalid value for", stringify!(TimeDomainEXT), ": {:?}"), other),
            })
            .finish()
    }
}
impl std::fmt::Display for TimeDomainEXT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match *self {
            Self::DEVICE => &"DEVICE",
            Self::CLOCK_MONOTONIC => &"CLOCK_MONOTONIC",
            Self::CLOCK_MONOTONIC_RAW => &"CLOCK_MONOTONIC_RAW",
            Self::QUERY_PERFORMANCE_COUNTER => &"QUERY_PERFORMANCE_COUNTER",
            other => unreachable!(concat!("invalid value for", stringify!(TimeDomainEXT), ": {:?}"), other),
        })
    }
}
///[VkCalibratedTimestampInfoEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkCalibratedTimestampInfoEXT.html) - Structure specifying the input parameters of a calibrated timestamp query
///# C Specifications
///The [`CalibratedTimestampInfoEXT`] structure is defined as:
///```c
///// Provided by VK_EXT_calibrated_timestamps
///typedef struct VkCalibratedTimestampInfoEXT {
///    VkStructureType    sType;
///    const void*        pNext;
///    VkTimeDomainEXT    timeDomain;
///} VkCalibratedTimestampInfoEXT;
///```
///# Members
/// - [`s_type`] is the type of this structure.
/// - [`p_next`] is `NULL` or a pointer to a structure extending this structure.
/// - [`time_domain`] is a [`TimeDomainEXT`] value specifying the time domain from which the
///   calibrated timestamp value should be returned.
///# Description
///## Valid Usage
/// - [`time_domain`] **must**  be one of the [`TimeDomainEXT`] values returned by
///   [`get_physical_device_calibrateable_time_domains_ext`]
///
///## Valid Usage (Implicit)
/// - [`s_type`] **must**  be `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
/// - [`p_next`] **must**  be `NULL`
/// - [`time_domain`] **must**  be a valid [`TimeDomainEXT`] value
///# Related
/// - [`ext_calibrated_timestamps`]
/// - [`StructureType`]
/// - [`TimeDomainEXT`]
/// - [`get_calibrated_timestamps_ext`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkCalibratedTimestampInfoEXT")]
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT<'lt> {
    ///Lifetime field
    pub _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    pub s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    pub p_next: *const BaseInStructure<'lt>,
    ///[`time_domain`] is a [`TimeDomainEXT`] value specifying the time
    ///domain from which the calibrated timestamp value should be returned.
    pub time_domain: TimeDomainEXT,
}
impl<'lt> Default for CalibratedTimestampInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: StructureType::CALIBRATED_TIMESTAMP_INFO_EXT,
            p_next: std::ptr::null(),
            time_domain: Default::default(),
        }
    }
}
impl<'lt> CalibratedTimestampInfoEXT<'lt> {
    ///Gets the raw value of [`Self::p_next`]
    pub fn p_next_raw(&self) -> *const BaseInStructure<'lt> {
        self.p_next
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next_raw(mut self, value: *const BaseInStructure<'lt>) -> Self {
        self.p_next = value;
        self
    }
    ///Gets the value of [`Self::s_type`]
    pub fn s_type(&self) -> StructureType {
        self.s_type
    }
    ///Gets the value of [`Self::p_next`]
    ///# Safety
    ///This function converts a pointer into a value which may be invalid, make sure
    ///that the pointer is valid before dereferencing.
    pub unsafe fn p_next(&self) -> &BaseInStructure<'lt> {
        &*self.p_next
    }
    ///Gets the value of [`Self::time_domain`]
    pub fn time_domain(&self) -> TimeDomainEXT {
        self.time_domain
    }
    ///Gets a mutable reference to the value of [`Self::s_type`]
    pub fn s_type_mut(&mut self) -> &mut StructureType {
        &mut self.s_type
    }
    ///Gets a mutable reference to the value of [`Self::time_domain`]
    pub fn time_domain_mut(&mut self) -> &mut TimeDomainEXT {
        &mut self.time_domain
    }
    ///Sets the value of [`Self::s_type`]
    pub fn set_s_type(mut self, value: crate::vulkan1_0::StructureType) -> Self {
        self.s_type = value;
        self
    }
    ///Sets the value of [`Self::p_next`]
    pub fn set_p_next(mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the value of [`Self::time_domain`]
    pub fn set_time_domain(mut self, value: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT) -> Self {
        self.time_domain = value;
        self
    }
}
impl PhysicalDevice {
    ///[vkGetPhysicalDeviceCalibrateableTimeDomainsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetPhysicalDeviceCalibrateableTimeDomainsEXT.html) - Query calibrateable time domains
    ///# C Specifications
    ///To query the set of time domains for which a physical device supports
    ///timestamp calibration, call:
    ///```c
    ///// Provided by VK_EXT_calibrated_timestamps
    ///VkResult vkGetPhysicalDeviceCalibrateableTimeDomainsEXT(
    ///    VkPhysicalDevice                            physicalDevice,
    ///    uint32_t*                                   pTimeDomainCount,
    ///    VkTimeDomainEXT*                            pTimeDomains);
    ///```
    ///# Parameters
    /// - [`physical_device`] is the physical device from which to query the set of calibrateable
    ///   time domains.
    /// - [`p_time_domain_count`] is a pointer to an integer related to the number of calibrateable
    ///   time domains available or queried, as described below.
    /// - [`p_time_domains`] is either `NULL` or a pointer to an array of [`TimeDomainEXT`] values,
    ///   indicating the supported calibrateable time domains.
    ///# Description
    ///If [`p_time_domains`] is `NULL`, then the number of calibrateable time
    ///domains supported for the given [`physical_device`] is returned in
    ///[`p_time_domain_count`].
    ///Otherwise, [`p_time_domain_count`] **must**  point to a variable set by the user
    ///to the number of elements in the [`p_time_domains`] array, and on return the
    ///variable is overwritten with the number of values actually written to
    ///[`p_time_domains`].
    ///If the value of [`p_time_domain_count`] is less than the number of
    ///calibrateable time domains supported, at most [`p_time_domain_count`] values
    ///will be written to [`p_time_domains`], and `VK_INCOMPLETE` will be
    ///returned instead of `VK_SUCCESS`, to indicate that not all the available
    ///time domains were returned.
    ///## Valid Usage (Implicit)
    /// - [`physical_device`] **must**  be a valid [`PhysicalDevice`] handle
    /// - [`p_time_domain_count`] **must**  be a valid pointer to a `uint32_t` value
    /// - If the value referenced by [`p_time_domain_count`] is not `0`, and [`p_time_domains`] is
    ///   not `NULL`, [`p_time_domains`] **must**  be a valid pointer to an array of
    ///   [`p_time_domain_count`][`TimeDomainEXT`] values
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`  - `VK_INCOMPLETE`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`ext_calibrated_timestamps`]
    /// - [`PhysicalDevice`]
    /// - [`TimeDomainEXT`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_physical_device_calibrateable_time_domains_ext(
        self: &Unique<PhysicalDevice>,
        p_time_domain_count: Option<usize>,
    ) -> VulkanResult<SmallVec<TimeDomainEXT>> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .instance()
            .vtable()
            .ext_calibrated_timestamps()
            .and_then(|vtable| vtable.get_physical_device_calibrateable_time_domains_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .instance()
            .vtable()
            .ext_calibrated_timestamps()
            .and_then(|vtable| vtable.get_physical_device_calibrateable_time_domains_ext())
            .unwrap_unchecked();
        let mut p_time_domain_count = match p_time_domain_count {
            Some(v) => v as _,
            None => {
                let mut v = 0;
                _function(self.as_raw(), &mut v, std::ptr::null_mut());
                v
            },
        };
        let mut p_time_domains = SmallVec::<TimeDomainEXT>::from_elem(Default::default(), p_time_domain_count as usize);
        let _return = _function(self.as_raw(), &mut p_time_domain_count, p_time_domains.as_mut_ptr());
        match _return {
            VulkanResultCodes::SUCCESS | VulkanResultCodes::INCOMPLETE => {
                VulkanResult::Success(_return, p_time_domains)
            },
            e => VulkanResult::Err(e),
        }
    }
}
impl Device {
    ///[vkGetCalibratedTimestampsEXT](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/vkGetCalibratedTimestampsEXT.html) - Query calibrated timestamps
    ///# C Specifications
    ///In order to be able to correlate the time a particular operation took place
    ///at on timelines of different time domains (e.g. a device operation vs a host
    ///operation), Vulkan allows querying calibrated timestamps from multiple time
    ///domains.To query calibrated timestamps from a set of time domains, call:
    ///```c
    ///// Provided by VK_EXT_calibrated_timestamps
    ///VkResult vkGetCalibratedTimestampsEXT(
    ///    VkDevice                                    device,
    ///    uint32_t                                    timestampCount,
    ///    const VkCalibratedTimestampInfoEXT*         pTimestampInfos,
    ///    uint64_t*                                   pTimestamps,
    ///    uint64_t*                                   pMaxDeviation);
    ///```
    ///# Parameters
    /// - [`device`] is the logical device used to perform the query.
    /// - [`timestamp_count`] is the number of timestamps to query.
    /// - [`p_timestamp_infos`] is a pointer to an array of
    ///   [`timestamp_count`][`CalibratedTimestampInfoEXT`] structures, describing the time domains
    ///   the calibrated timestamps should be captured from.
    /// - [`p_timestamps`] is a pointer to an array of [`timestamp_count`] 64-bit unsigned integer
    ///   values in which the requested calibrated timestamp values are returned.
    /// - [`p_max_deviation`] is a pointer to a 64-bit unsigned integer value in which the strictly
    ///   positive maximum deviation, in nanoseconds, of the calibrated timestamp values is
    ///   returned.
    ///# Description
    ///Calibrated timestamp values  **can**  be extrapolated to estimate future
    ///coinciding timestamp values, however, depending on the nature of the time
    ///domains and other properties of the platform extrapolating values over a
    ///sufficiently long period of time  **may**  no longer be accurate enough to fit
    ///any particular purpose, so applications are expected to re-calibrate the
    ///timestamps on a regular basis.
    ///## Valid Usage (Implicit)
    /// - [`device`] **must**  be a valid [`Device`] handle
    /// - [`p_timestamp_infos`] **must**  be a valid pointer to an array of [`timestamp_count`]
    ///   valid [`CalibratedTimestampInfoEXT`] structures
    /// - [`p_timestamps`] **must**  be a valid pointer to an array of [`timestamp_count`]`uint64_t`
    ///   values
    /// - [`p_max_deviation`] **must**  be a valid pointer to a `uint64_t` value
    /// - [`timestamp_count`] **must**  be greater than `0`
    ///
    ///## Return Codes
    /// * - `VK_SUCCESS`
    /// * - `VK_ERROR_OUT_OF_HOST_MEMORY`  - `VK_ERROR_OUT_OF_DEVICE_MEMORY`
    ///# Related
    /// - [`ext_calibrated_timestamps`]
    /// - [`CalibratedTimestampInfoEXT`]
    /// - [`Device`]
    ///
    ///# Notes and documentation
    ///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
    ///
    ///This documentation is generated from the Vulkan specification and documentation.
    ///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
    /// Commons Attribution 4.0 International*.
    ///This license explicitely allows adapting the source material as long as proper credit is
    /// given.
    #[doc(alias = "vkGetCalibratedTimestampsEXT")]
    #[track_caller]
    #[inline]
    pub unsafe fn get_calibrated_timestamps_ext<'lt>(
        self: &Unique<Device>,
        p_timestamp_infos: &[crate::extensions::ext_calibrated_timestamps::CalibratedTimestampInfoEXT<'lt>],
    ) -> VulkanResult<(SmallVec<u64>, u64)> {
        #[cfg(any(debug_assertions, feature = "assertions"))]
        let _function = self
            .vtable()
            .ext_calibrated_timestamps()
            .and_then(|vtable| vtable.get_calibrated_timestamps_ext())
            .expect("function not loaded");
        #[cfg(not(any(debug_assertions, feature = "assertions")))]
        let _function = self
            .vtable()
            .ext_calibrated_timestamps()
            .and_then(|vtable| vtable.get_calibrated_timestamps_ext())
            .unwrap_unchecked();
        let timestamp_count = (|len: usize| len)(p_timestamp_infos.len()) as _;
        let mut p_timestamps = SmallVec::<u64>::from_elem(Default::default(), timestamp_count as usize);
        let mut p_max_deviation = Default::default();
        let _return = _function(
            self.as_raw(),
            timestamp_count,
            p_timestamp_infos.as_ptr(),
            p_timestamps.as_mut_ptr(),
            &mut p_max_deviation,
        );
        match _return {
            VulkanResultCodes::SUCCESS => VulkanResult::Success(_return, (p_timestamps, p_max_deviation)),
            e => VulkanResult::Err(e),
        }
    }
}
///The V-table of [`Instance`] for functions from `VK_EXT_calibrated_timestamps`
pub struct InstanceExtCalibratedTimestampsVTable {
    ///See [`FNGetPhysicalDeviceCalibrateableTimeDomainsExt`] for more information.
    pub get_physical_device_calibrateable_time_domains_ext: FNGetPhysicalDeviceCalibrateableTimeDomainsExt,
}
impl InstanceExtCalibratedTimestampsVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Instance,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Instance,
    ) -> Self {
        Self {
            get_physical_device_calibrateable_time_domains_ext: unsafe {
                std::mem::transmute(loader_fn(
                    loader,
                    crate::cstr!("vkGetPhysicalDeviceCalibrateableTimeDomainsEXT").as_ptr(),
                ))
            },
        }
    }
    ///Gets [`Self::get_physical_device_calibrateable_time_domains_ext`]. See
    /// [`FNGetPhysicalDeviceCalibrateableTimeDomainsExt`] for more information.
    pub fn get_physical_device_calibrateable_time_domains_ext(&self) -> FNGetPhysicalDeviceCalibrateableTimeDomainsExt {
        self.get_physical_device_calibrateable_time_domains_ext
    }
}
///The V-table of [`Device`] for functions from `VK_EXT_calibrated_timestamps`
pub struct DeviceExtCalibratedTimestampsVTable {
    ///See [`FNGetCalibratedTimestampsExt`] for more information.
    pub get_calibrated_timestamps_ext: FNGetCalibratedTimestampsExt,
}
impl DeviceExtCalibratedTimestampsVTable {
    ///Loads the VTable from the owner and the names
    #[track_caller]
    pub fn load(
        loader_fn: unsafe extern "system" fn(
            Device,
            *const std::os::raw::c_char,
        ) -> Option<unsafe extern "system" fn()>,
        loader: Device,
    ) -> Self {
        Self {
            get_calibrated_timestamps_ext: unsafe {
                std::mem::transmute(loader_fn(loader, crate::cstr!("vkGetCalibratedTimestampsEXT").as_ptr()))
            },
        }
    }
    ///Gets [`Self::get_calibrated_timestamps_ext`]. See [`FNGetCalibratedTimestampsExt`] for more
    /// information.
    pub fn get_calibrated_timestamps_ext(&self) -> FNGetCalibratedTimestampsExt {
        self.get_calibrated_timestamps_ext
    }
}
