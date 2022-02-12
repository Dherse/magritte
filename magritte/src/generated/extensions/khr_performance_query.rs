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
