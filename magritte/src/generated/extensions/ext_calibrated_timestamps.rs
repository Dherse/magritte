use crate::vulkan1_0::{BaseInStructure, StructureType};
#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
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
/// - [`TimeDomainDeviceExt`] specifies the device time domain. Timestamp values in this time domain
///   use the same units and are comparable with device timestamp values captured using [`CmdWriteTimestamp`]
///   or [`CmdWriteTimestamp2`] and are defined to be incrementing according to the [timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod)
///   of the device.
/// - [`TimeDomainClockMonotonicExt`] specifies the CLOCK_MONOTONIC time domain available on POSIX
///   platforms. Timestamp values in this time domain are in units of nanoseconds and are comparable
///   with platform timestamp values captured using the POSIX clock_gettime API as computed by this
///   example:
///
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`TimeDomainClockMonotonicRawExt`] specifies the CLOCK_MONOTONIC_RAW time domain available on
///   POSIX platforms. Timestamp values in this time domain are in units of nanoseconds and are
///   comparable with platform timestamp values captured using the POSIX clock_gettime API as
///   computed by this example:
///
///```c
///struct timespec tv;
///clock_gettime(CLOCK_MONOTONIC_RAW, &tv);
///return tv.tv_nsec + tv.tv_sec*1000000000ull;
///```
///
/// - [`TimeDomainQueryPerformanceCounterExt`] specifies the performance counter (QPC) time domain
///   available on Windows. Timestamp values in this time domain are in the same units as those
///   provided by the Windows QueryPerformanceCounter API and are comparable with platform timestamp
///   values captured using that API as computed by this example:
///
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum TimeDomainEXT {
    ///[`TimeDomainDeviceExt`] specifies the device time domain.
    ///Timestamp values in this time domain use the same units and are
    ///comparable with device timestamp values captured using
    ///[`CmdWriteTimestamp`]
    ///or [`CmdWriteTimestamp2`]
    ///and are defined to be incrementing according to the
    ///[timestampPeriod](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html#limits-timestampPeriod) of the device.
    TimeDomainDeviceExt = 0,
    ///[`TimeDomainClockMonotonicExt`] specifies the CLOCK_MONOTONIC
    ///time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    TimeDomainClockMonotonicExt = 1,
    ///[`TimeDomainClockMonotonicRawExt`] specifies the
    ///CLOCK_MONOTONIC_RAW time domain available on POSIX platforms.
    ///Timestamp values in this time domain are in units of nanoseconds and are
    ///comparable with platform timestamp values captured using the POSIX
    ///clock_gettime API as computed by this example:
    TimeDomainClockMonotonicRawExt = 2,
    ///[`TimeDomainQueryPerformanceCounterExt`] specifies the
    ///performance counter (QPC) time domain available on Windows.
    ///Timestamp values in this time domain are in the same units as those
    ///provided by the Windows QueryPerformanceCounter API and are comparable
    ///with platform timestamp values captured using that API as computed by
    ///this example:
    TimeDomainQueryPerformanceCounterExt = 3,
}
impl const Default for TimeDomainEXT {
    fn default() -> Self {
        TimeDomainDeviceExt
    }
}
impl TimeDomainEXT {
    ///Default empty value
    #[inline]
    pub const fn empty() -> Self {
        Self::default()
    }
    ///Gets the raw underlying value
    #[inline]
    pub const fn bits(&self) -> i32 {
        self as i32
    }
    ///Gets a value from a raw underlying value, unchecked and therefore unsafe
    #[inline]
    pub const unsafe fn from_bits(bits: i32) -> i32 {
        std::mem::transmute(bits)
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
///Valid Usage
/// - [`time_domain`]**must** be one of the [`TimeDomainEXT`] values returned by
///   [`GetPhysicalDeviceCalibrateableTimeDomainsEXT`]
///Valid Usage (Implicit)
/// - [`s_type`]**must** be `VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_EXT`
/// - [`p_next`]**must** be `NULL`
/// - [`time_domain`]**must** be a valid [`TimeDomainEXT`] value
///# Related
/// - [`VK_EXT_calibrated_timestamps`]
/// - [`StructureType`]
/// - [`TimeDomainEXT`]
/// - [`GetCalibratedTimestampsEXT`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[repr(C)]
pub struct CalibratedTimestampInfoEXT<'lt> {
    _lifetime: PhantomData<&'lt ()>,
    ///[`s_type`] is the type of this structure.
    s_type: StructureType,
    ///[`p_next`] is `NULL` or a pointer to a structure extending this
    ///structure.
    p_next: *const BaseInStructure<'lt>,
    ///[`time_domain`] is a [`TimeDomainEXT`] value specifying the time
    ///domain from which the calibrated timestamp value should be returned.
    time_domain: TimeDomainEXT,
}
impl<'lt> Default for CalibratedTimestampInfoEXT<'lt> {
    fn default() -> Self {
        Self {
            _lifetime: PhantomData,
            s_type: Default::default(),
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
    pub fn set_p_next_raw(&mut self, value: *const BaseInStructure<'lt>) -> &mut Self {
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
    ///Sets the raw value of [`Self::s_type`]
    pub fn set_s_type(&mut self, value: crate::vulkan1_0::StructureType) -> &mut Self {
        self.s_type = value;
        self
    }
    ///Sets the raw value of [`Self::p_next`]
    pub fn set_p_next(&mut self, value: &'lt crate::vulkan1_0::BaseInStructure<'lt>) -> &mut Self {
        self.p_next = value as *const _;
        self
    }
    ///Sets the raw value of [`Self::time_domain`]
    pub fn set_time_domain(&mut self, value: crate::extensions::ext_calibrated_timestamps::TimeDomainEXT) -> &mut Self {
        self.time_domain = value;
        self
    }
}
