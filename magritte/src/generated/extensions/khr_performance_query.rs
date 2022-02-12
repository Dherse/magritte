//![VK_KHR_performance_query](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VK_KHR_performance_query.html) - device extension
//!# Description
//!The [`VK_KHR_performance_query`] extension adds a mechanism to allow querying
//!of performance counters for use in applications and by profiling tools.Each queue family **may**
//! expose counters that **can** be enabled on a queue of
//!that family.
//!We extend [`QueryType`] to add a new query type for performance queries,
//!and chain a structure on [`QueryPoolCreateInfo`] to specify the
//!performance queries to enable.
//!# Revision
//!1
//!# Dependencies
//! - Requires Vulkan 1.0
//! - Requires `[`VK_KHR_get_physical_device_properties2`]`
//!# Contacts
//! - Alon Or-bach [alonorbach](https://github.com/KhronosGroup/Vulkan-Docs/issues/new?body=[VK_KHR_performance_query]
//!   @alonorbach%0A<<Here describe the issue or question you have about the
//!   VK_KHR_performance_query extension>>)
//!# New functions & commands
//! - [`AcquireProfilingLockKHR`]
//! - [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]
//! - [`GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`]
//! - [`ReleaseProfilingLockKHR`]
//!# New structures
//! - [`AcquireProfilingLockInfoKHR`]
//! - [`PerformanceCounterDescriptionKHR`]
//! - [`PerformanceCounterKHR`]
//! - Extending [`PhysicalDeviceFeatures2`], [`DeviceCreateInfo`]:
//! - [`PhysicalDevicePerformanceQueryFeaturesKHR`]
//! - Extending [`PhysicalDeviceProperties2`]:
//! - [`PhysicalDevicePerformanceQueryPropertiesKHR`]
//! - Extending [`QueryPoolCreateInfo`]:
//! - [`QueryPoolPerformanceCreateInfoKHR`]
//! - Extending [`SubmitInfo`], [`SubmitInfo2`]:
//! - [`PerformanceQuerySubmitInfoKHR`]
//!# New enums
//! - [`AcquireProfilingLockFlagBitsKHR`]
//! - [`PerformanceCounterDescriptionFlagBitsKHR`]
//! - [`PerformanceCounterScopeKHR`]
//! - [`PerformanceCounterStorageKHR`]
//! - [`PerformanceCounterUnitKHR`]
//!# New bitmasks
//! - [`AcquireProfilingLockFlagsKHR`]
//! - [`PerformanceCounterDescriptionFlagsKHR`]
//!# New constants
//! - [`KHR_PERFORMANCE_QUERY_EXTENSION_NAME`]
//! - [`KHR_PERFORMANCE_QUERY_SPEC_VERSION`]
//! - Extending [`QueryType`]:
//! - `VK_QUERY_TYPE_PERFORMANCE_QUERY_KHR`
//! - Extending [`StructureType`]:
//! - `VK_STRUCTURE_TYPE_ACQUIRE_PROFILING_LOCK_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_DESCRIPTION_KHR`
//! - `VK_STRUCTURE_TYPE_PERFORMANCE_COUNTER_KHR`
//! - `VK_STRUCTURE_TYPE_PERFORMANCE_QUERY_SUBMIT_INFO_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_FEATURES_KHR`
//! - `VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PERFORMANCE_QUERY_PROPERTIES_KHR`
//! - `VK_STRUCTURE_TYPE_QUERY_POOL_PERFORMANCE_CREATE_INFO_KHR`
//!# Known issues & F.A.Q
//!1) Should this extension include a mechanism to begin a query in command
//!buffer *A* and end the query in command buffer *B*?**RESOLVED** No - queries are tied to command
//! buffer creation and thus have to
//!be encapsulated within a single command buffer.2) Should this extension include a mechanism to
//! begin and end queries
//!globally on the queue, not using the existing command buffer commands?**RESOLVED** No - for the
//! same reasoning as the resolution of 1).3) Should this extension expose counters that require
//! multiple passes?**RESOLVED** Yes - users should re-submit a command buffer with the same
//!commands in it multiple times, specifying the pass to count as the query
//!parameter in VkPerformanceQuerySubmitInfoKHR.4) How to handle counters across parallel
//! workloads?**RESOLVED** In the spirit of Vulkan, a counter description flag
//!`VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR`
//!denotes that the accuracy of a counter result is affected by parallel
//!workloads.5) How to handle secondary command buffers?**RESOLVED** Secondary command buffers
//! inherit any counter pass index
//!specified in the parent primary command buffer.
//!Note: this is no longer an issue after change from issue 10 resolution6) What commands does the
//! profiling lock have to be held for?**RESOLVED** For any command buffer that is being queried
//! with a performance
//!query pool, the profiling lock **must** be held while that command buffer is in
//!the *recording*, *executable*, or *pending state*.7) Should we support
//! [`CmdCopyQueryPoolResults`]?**RESOLVED** Yes.8) Should we allow performance queries to interact
//! with multiview?**RESOLVED** Yes, but the performance queries must be performed once for each
//!pass per view.9) Should a `queryCount > 1` be usable for performance queries?**RESOLVED** Yes.
//!Some vendors will have costly performance counter query pool creation, and
//!would rather if a certain set of counters were to be used multiple times
//!that a `queryCount > 1` can be used to amortize the instantiation cost.10) Should we introduce
//! an indirect mechanism to set the counter pass index?**RESOLVED** Specify the counter pass index
//! at submit time instead, to avoid
//!requiring re-recording of command buffers when multiple counter passes are
//!needed.
//!# Version History
//! - Revision 1, 2019-10-08
//!# Other info
//! * 2019-10-08
//! * No known IP claims.
//!*
//! - Jesse Barker, Unity Technologies
//! - Kenneth Benzie, Codeplay
//! - Jan-Harald Fredriksen, ARM
//! - Jeff Leger, Qualcomm
//! - Jesse Hall, Google
//! - Tobias Hector, AMD
//! - Neil Henning, Codeplay
//! - Baldur Karlsson
//! - Lionel Landwerlin, Intel
//! - Peter Lohrmann, AMD
//! - Alon Or-bach, Samsung
//! - Daniel Rakos, AMD
//! - Niklas Smedberg, Unity Technologies
//! - Igor Ostrowski, Intel
//!# Related
//! - [`AcquireProfilingLockFlagBitsKHR`]
//! - [`AcquireProfilingLockFlagsKHR`]
//! - [`AcquireProfilingLockInfoKHR`]
//! - [`PerformanceCounterDescriptionFlagBitsKHR`]
//! - [`PerformanceCounterDescriptionFlagsKHR`]
//! - [`PerformanceCounterDescriptionKHR`]
//! - [`PerformanceCounterKHR`]
//! - [`PerformanceCounterResultKHR`]
//! - [`PerformanceCounterScopeKHR`]
//! - [`PerformanceCounterStorageKHR`]
//! - [`PerformanceCounterUnitKHR`]
//! - [`PerformanceQuerySubmitInfoKHR`]
//! - [`PhysicalDevicePerformanceQueryFeaturesKHR`]
//! - [`PhysicalDevicePerformanceQueryPropertiesKHR`]
//! - [`QueryPoolPerformanceCreateInfoKHR`]
//! - [`AcquireProfilingLockKHR`]
//! - [`EnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR`]
//! - [`GetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR`]
//! - [`ReleaseProfilingLockKHR`]
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
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const KHR_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 1;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_KHR_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const KHR_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_KHR_performance_query");
///[VkPerformanceCounterScopeKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterScopeKHR.html) - Supported counter scope types
///# C Specifications
///Performance counters have an associated scope.
///This scope describes the granularity of a performance counter.The performance counter scope
/// types which **may** be returned in
///[`PerformanceCounterKHR::scope`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterScopeKHR {
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0,
///    VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1,
///    VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2,
///    VK_QUERY_SCOPE_COMMAND_BUFFER_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR,
///    VK_QUERY_SCOPE_RENDER_PASS_KHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR,
///    VK_QUERY_SCOPE_COMMAND_KHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR,
///} VkPerformanceCounterScopeKHR;
///```
///# Description
/// - [`PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER`] - the performance
///counter scope is a single complete command buffer.
/// - [`PERFORMANCE_COUNTER_SCOPE_RENDER_PASS`] - the performance
///counter scope is zero or more complete render passes.
///The performance query containing the performance counter **must** begin and
///end outside a render pass instance.
/// - [`PERFORMANCE_COUNTER_SCOPE_COMMAND`] - the performance counter
///scope is zero or more commands.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterScopeKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceCounterScopeKHR(i32);
impl const Default for PerformanceCounterScopeKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceCounterScopeKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceCounterScopeKHR")
            .field(match *self {
                Self::PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER => &"PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER",
                Self::PERFORMANCE_COUNTER_SCOPE_RENDER_PASS => &"PERFORMANCE_COUNTER_SCOPE_RENDER_PASS",
                Self::PERFORMANCE_COUNTER_SCOPE_COMMAND => &"PERFORMANCE_COUNTER_SCOPE_COMMAND",
                other => unreachable!("invalid value for `PerformanceCounterScopeKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceCounterScopeKHR {
    ///[`PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER`] - the performance
    ///counter scope is a single complete command buffer.
    pub const PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER: Self = Self(0);
    ///[`PERFORMANCE_COUNTER_SCOPE_RENDER_PASS`] - the performance
    ///counter scope is zero or more complete render passes.
    ///The performance query containing the performance counter **must** begin and
    ///end outside a render pass instance.
    pub const PERFORMANCE_COUNTER_SCOPE_RENDER_PASS: Self = Self(1);
    ///[`PERFORMANCE_COUNTER_SCOPE_COMMAND`] - the performance counter
    ///scope is zero or more commands.
    pub const PERFORMANCE_COUNTER_SCOPE_COMMAND: Self = Self(2);
    ///No documentation found
    pub const QUERY_SCOPE_COMMAND_BUFFER: Self = Self::PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER;
    ///No documentation found
    pub const QUERY_SCOPE_RENDER_PASS: Self = Self::PERFORMANCE_COUNTER_SCOPE_RENDER_PASS;
    ///No documentation found
    pub const QUERY_SCOPE_COMMAND: Self = Self::PERFORMANCE_COUNTER_SCOPE_COMMAND;
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
///[VkPerformanceCounterUnitKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterUnitKHR.html) - Supported counter unit types
///# C Specifications
///Performance counters have an associated unit.
///This unit describes how to interpret the performance counter result.The performance counter unit
/// types which **may** be returned in
///[`PerformanceCounterKHR::unit`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterUnitKHR {
///    VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0,
///    VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1,
///    VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3,
///    VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4,
///    VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5,
///    VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6,
///    VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7,
///    VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8,
///    VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9,
///    VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10,
///} VkPerformanceCounterUnitKHR;
///```
///# Description
/// - [`PERFORMANCE_COUNTER_UNIT_GENERIC`] - the performance counter
///unit is a generic data point.
/// - [`PERFORMANCE_COUNTER_UNIT_PERCENTAGE`] - the performance
///counter unit is a percentage (%).
/// - [`PERFORMANCE_COUNTER_UNIT_NANOSECONDS`] - the performance
///counter unit is a value of nanoseconds (ns).
/// - [`PERFORMANCE_COUNTER_UNIT_BYTES`] - the performance counter
///unit is a value of bytes.
/// - [`PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND`] - the performance
///counter unit is a value of bytes/s.
/// - [`PERFORMANCE_COUNTER_UNIT_KELVIN`] - the performance counter
///unit is a temperature reported in Kelvin.
/// - [`PERFORMANCE_COUNTER_UNIT_WATTS`] - the performance counter
///unit is a value of watts (W).
/// - [`PERFORMANCE_COUNTER_UNIT_VOLTS`] - the performance counter
///unit is a value of volts (V).
/// - [`PERFORMANCE_COUNTER_UNIT_AMPS`] - the performance counter
///unit is a value of amps (A).
/// - [`PERFORMANCE_COUNTER_UNIT_HERTZ`] - the performance counter
///unit is a value of hertz (Hz).
/// - [`PERFORMANCE_COUNTER_UNIT_CYCLES`] - the performance counter
///unit is a value of cycles.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterUnitKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceCounterUnitKHR(i32);
impl const Default for PerformanceCounterUnitKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceCounterUnitKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceCounterUnitKHR")
            .field(match *self {
                Self::PERFORMANCE_COUNTER_UNIT_GENERIC => &"PERFORMANCE_COUNTER_UNIT_GENERIC",
                Self::PERFORMANCE_COUNTER_UNIT_PERCENTAGE => &"PERFORMANCE_COUNTER_UNIT_PERCENTAGE",
                Self::PERFORMANCE_COUNTER_UNIT_NANOSECONDS => &"PERFORMANCE_COUNTER_UNIT_NANOSECONDS",
                Self::PERFORMANCE_COUNTER_UNIT_BYTES => &"PERFORMANCE_COUNTER_UNIT_BYTES",
                Self::PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND => &"PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND",
                Self::PERFORMANCE_COUNTER_UNIT_KELVIN => &"PERFORMANCE_COUNTER_UNIT_KELVIN",
                Self::PERFORMANCE_COUNTER_UNIT_WATTS => &"PERFORMANCE_COUNTER_UNIT_WATTS",
                Self::PERFORMANCE_COUNTER_UNIT_VOLTS => &"PERFORMANCE_COUNTER_UNIT_VOLTS",
                Self::PERFORMANCE_COUNTER_UNIT_AMPS => &"PERFORMANCE_COUNTER_UNIT_AMPS",
                Self::PERFORMANCE_COUNTER_UNIT_HERTZ => &"PERFORMANCE_COUNTER_UNIT_HERTZ",
                Self::PERFORMANCE_COUNTER_UNIT_CYCLES => &"PERFORMANCE_COUNTER_UNIT_CYCLES",
                other => unreachable!("invalid value for `PerformanceCounterUnitKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceCounterUnitKHR {
    ///[`PERFORMANCE_COUNTER_UNIT_GENERIC`] - the performance counter
    ///unit is a generic data point.
    pub const PERFORMANCE_COUNTER_UNIT_GENERIC: Self = Self(0);
    ///[`PERFORMANCE_COUNTER_UNIT_PERCENTAGE`] - the performance
    ///counter unit is a percentage (%).
    pub const PERFORMANCE_COUNTER_UNIT_PERCENTAGE: Self = Self(1);
    ///[`PERFORMANCE_COUNTER_UNIT_NANOSECONDS`] - the performance
    ///counter unit is a value of nanoseconds (ns).
    pub const PERFORMANCE_COUNTER_UNIT_NANOSECONDS: Self = Self(2);
    ///[`PERFORMANCE_COUNTER_UNIT_BYTES`] - the performance counter
    ///unit is a value of bytes.
    pub const PERFORMANCE_COUNTER_UNIT_BYTES: Self = Self(3);
    ///[`PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND`] - the performance
    ///counter unit is a value of bytes/s.
    pub const PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND: Self = Self(4);
    ///[`PERFORMANCE_COUNTER_UNIT_KELVIN`] - the performance counter
    ///unit is a temperature reported in Kelvin.
    pub const PERFORMANCE_COUNTER_UNIT_KELVIN: Self = Self(5);
    ///[`PERFORMANCE_COUNTER_UNIT_WATTS`] - the performance counter
    ///unit is a value of watts (W).
    pub const PERFORMANCE_COUNTER_UNIT_WATTS: Self = Self(6);
    ///[`PERFORMANCE_COUNTER_UNIT_VOLTS`] - the performance counter
    ///unit is a value of volts (V).
    pub const PERFORMANCE_COUNTER_UNIT_VOLTS: Self = Self(7);
    ///[`PERFORMANCE_COUNTER_UNIT_AMPS`] - the performance counter
    ///unit is a value of amps (A).
    pub const PERFORMANCE_COUNTER_UNIT_AMPS: Self = Self(8);
    ///[`PERFORMANCE_COUNTER_UNIT_HERTZ`] - the performance counter
    ///unit is a value of hertz (Hz).
    pub const PERFORMANCE_COUNTER_UNIT_HERTZ: Self = Self(9);
    ///[`PERFORMANCE_COUNTER_UNIT_CYCLES`] - the performance counter
    ///unit is a value of cycles.
    pub const PERFORMANCE_COUNTER_UNIT_CYCLES: Self = Self(10);
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
///[VkPerformanceCounterStorageKHR](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceCounterStorageKHR.html) - Supported counter storage types
///# C Specifications
///Performance counters have an associated storage.
///This storage describes the payload of a counter result.The performance counter storage types
/// which **may** be returned in
///[`PerformanceCounterKHR::storage`] are:
///```c
///// Provided by VK_KHR_performance_query
///typedef enum VkPerformanceCounterStorageKHR {
///    VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0,
///    VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2,
///    VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4,
///    VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5,
///} VkPerformanceCounterStorageKHR;
///```
///# Description
/// - [`PERFORMANCE_COUNTER_STORAGE_INT32`] - the performance counter
///storage is a 32-bit signed integer.
/// - [`PERFORMANCE_COUNTER_STORAGE_INT64`] - the performance counter
///storage is a 64-bit signed integer.
/// - [`PERFORMANCE_COUNTER_STORAGE_UINT32`] - the performance
///counter storage is a 32-bit unsigned integer.
/// - [`PERFORMANCE_COUNTER_STORAGE_UINT64`] - the performance
///counter storage is a 64-bit unsigned integer.
/// - [`PERFORMANCE_COUNTER_STORAGE_FLOAT32`] - the performance
///counter storage is a 32-bit floating-point.
/// - [`PERFORMANCE_COUNTER_STORAGE_FLOAT64`] - the performance
///counter storage is a 64-bit floating-point.
///# Related
/// - [`VK_KHR_performance_query`]
/// - [`PerformanceCounterKHR`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceCounterStorageKHR")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceCounterStorageKHR(i32);
impl const Default for PerformanceCounterStorageKHR {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceCounterStorageKHR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceCounterStorageKHR")
            .field(match *self {
                Self::PERFORMANCE_COUNTER_STORAGE_INT32 => &"PERFORMANCE_COUNTER_STORAGE_INT32",
                Self::PERFORMANCE_COUNTER_STORAGE_INT64 => &"PERFORMANCE_COUNTER_STORAGE_INT64",
                Self::PERFORMANCE_COUNTER_STORAGE_UINT32 => &"PERFORMANCE_COUNTER_STORAGE_UINT32",
                Self::PERFORMANCE_COUNTER_STORAGE_UINT64 => &"PERFORMANCE_COUNTER_STORAGE_UINT64",
                Self::PERFORMANCE_COUNTER_STORAGE_FLOAT32 => &"PERFORMANCE_COUNTER_STORAGE_FLOAT32",
                Self::PERFORMANCE_COUNTER_STORAGE_FLOAT64 => &"PERFORMANCE_COUNTER_STORAGE_FLOAT64",
                other => unreachable!("invalid value for `PerformanceCounterStorageKHR`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceCounterStorageKHR {
    ///[`PERFORMANCE_COUNTER_STORAGE_INT32`] - the performance counter
    ///storage is a 32-bit signed integer.
    pub const PERFORMANCE_COUNTER_STORAGE_INT32: Self = Self(0);
    ///[`PERFORMANCE_COUNTER_STORAGE_INT64`] - the performance counter
    ///storage is a 64-bit signed integer.
    pub const PERFORMANCE_COUNTER_STORAGE_INT64: Self = Self(1);
    ///[`PERFORMANCE_COUNTER_STORAGE_UINT32`] - the performance
    ///counter storage is a 32-bit unsigned integer.
    pub const PERFORMANCE_COUNTER_STORAGE_UINT32: Self = Self(2);
    ///[`PERFORMANCE_COUNTER_STORAGE_UINT64`] - the performance
    ///counter storage is a 64-bit unsigned integer.
    pub const PERFORMANCE_COUNTER_STORAGE_UINT64: Self = Self(3);
    ///[`PERFORMANCE_COUNTER_STORAGE_FLOAT32`] - the performance
    ///counter storage is a 32-bit floating-point.
    pub const PERFORMANCE_COUNTER_STORAGE_FLOAT32: Self = Self(4);
    ///[`PERFORMANCE_COUNTER_STORAGE_FLOAT64`] - the performance
    ///counter storage is a 64-bit floating-point.
    pub const PERFORMANCE_COUNTER_STORAGE_FLOAT64: Self = Self(5);
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
