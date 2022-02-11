#[cfg(feature = "bytemuck")]
use bytemuck::{Pod, Zeroable};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::CStr;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
///This element is not documented in the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html).
///See the module level documentation where a description may be given.
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = crate::cstr!("VK_INTEL_performance_query");
///[VkPerformanceConfigurationTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html) - Type of performance configuration
///# C Specifications
///Possible values of
///[`PerformanceConfigurationAcquireInfoINTEL::type_`], specifying
///performance configuration types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceConfigurationTypeINTEL {
///    VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL = 0,
///} VkPerformanceConfigurationTypeINTEL;
///```
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceConfigurationAcquireInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl const Default for PerformanceConfigurationTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceConfigurationTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceConfigurationTypeINTEL")
            .field(match *self {
                Self::PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED => {
                    &"PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED"
                },
                other => unreachable!("invalid value for `PerformanceConfigurationTypeINTEL`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceConfigurationTypeINTEL {
    ///No documentation found
    pub const PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
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
///[VkQueryPoolSamplingModeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html) - Enum specifying how performance queries should be captured
///# C Specifications
///Possible values of
///[`QueryPoolPerformanceQueryCreateInfoINTEL::performance_counters_sampling`]
///are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkQueryPoolSamplingModeINTEL {
///    VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0,
///} VkQueryPoolSamplingModeINTEL;
///```
///# Description
/// - [`QUERY_POOL_SAMPLING_MODE_MANUAL`] is the default mode in
///which the application calls [`CmdBeginQuery`] and
///[`CmdEndQuery`] to record performance data.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`QueryPoolPerformanceQueryCreateInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct QueryPoolSamplingModeINTEL(i32);
impl const Default for QueryPoolSamplingModeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for QueryPoolSamplingModeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("QueryPoolSamplingModeINTEL")
            .field(match *self {
                Self::QUERY_POOL_SAMPLING_MODE_MANUAL => &"QUERY_POOL_SAMPLING_MODE_MANUAL",
                other => unreachable!("invalid value for `QueryPoolSamplingModeINTEL`: {:?}", other),
            })
            .finish()
    }
}
impl QueryPoolSamplingModeINTEL {
    ///[`QUERY_POOL_SAMPLING_MODE_MANUAL`] is the default mode in
    ///which the application calls [`CmdBeginQuery`] and
    ///[`CmdEndQuery`] to record performance data.
    pub const QUERY_POOL_SAMPLING_MODE_MANUAL: Self = Self(0);
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
///[VkPerformanceOverrideTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html) - Performance override type
///# C Specifications
///Possible values of [`PerformanceOverrideInfoINTEL::type_`],
///specifying performance override types, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceOverrideTypeINTEL {
///    VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0,
///    VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1,
///} VkPerformanceOverrideTypeINTEL;
///```
///# Description
/// - [`PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE`] turns all
///rendering operations into noop.
/// - [`PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES`] stalls the
///stream of commands until all previously emitted commands have completed
///and all caches been flushed and invalidated.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceOverrideInfoINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceOverrideTypeINTEL(i32);
impl const Default for PerformanceOverrideTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceOverrideTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceOverrideTypeINTEL")
            .field(match *self {
                Self::PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE => &"PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE",
                Self::PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES => &"PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES",
                other => unreachable!("invalid value for `PerformanceOverrideTypeINTEL`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceOverrideTypeINTEL {
    ///[`PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE`] turns all
    ///rendering operations into noop.
    pub const PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE: Self = Self(0);
    ///[`PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES`] stalls the
    ///stream of commands until all previously emitted commands have completed
    ///and all caches been flushed and invalidated.
    pub const PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES: Self = Self(1);
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
///[VkPerformanceParameterTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html) - Parameters that can be queried
///# C Specifications
///Possible values of [`GetPerformanceParameterINTEL`]`::parameter`,
///specifying a performance query feature, are:
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceParameterTypeINTEL {
///    VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0,
///    VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1,
///} VkPerformanceParameterTypeINTEL;
///```
///# Description
/// - [`PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED`] has a
///boolean result which tells whether hardware counters can be captured.
/// - [`PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS`] has a
///32 bits integer result which tells how many bits can be written into the
///[`PerformanceValueINTEL`] value.
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`GetPerformanceParameterINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceParameterTypeINTEL(i32);
impl const Default for PerformanceParameterTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceParameterTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceParameterTypeINTEL")
            .field(match *self {
                Self::PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED => {
                    &"PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED"
                },
                Self::PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS => {
                    &"PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS"
                },
                other => unreachable!("invalid value for `PerformanceParameterTypeINTEL`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceParameterTypeINTEL {
    ///[`PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED`] has a
    ///boolean result which tells whether hardware counters can be captured.
    pub const PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED: Self = Self(0);
    ///[`PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS`] has a
    ///32 bits integer result which tells how many bits can be written into the
    ///[`PerformanceValueINTEL`] value.
    pub const PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS: Self = Self(1);
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
///[VkPerformanceValueTypeINTEL](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html) - Type of the parameters that can be queried
///# C Specifications
///Possible values of [`PerformanceValueINTEL::type_`], specifying the
///type of the data returned in [`PerformanceValueINTEL::data`], are:
/// - [`PERFORMANCE_VALUE_TYPE_UINT32`] specifies that unsigned
///32-bit integer data is returned in `data.value32`.
/// - [`PERFORMANCE_VALUE_TYPE_UINT64`] specifies that unsigned
///64-bit integer data is returned in `data.value64`.
/// - [`PERFORMANCE_VALUE_TYPE_FLOAT`] specifies that
///floating-point data is returned in `data.valueFloat`.
/// - [`PERFORMANCE_VALUE_TYPE_BOOL`] specifies that
///[`Bool32`] data is returned in `data.valueBool`.
/// - [`PERFORMANCE_VALUE_TYPE_STRING`] specifies that a pointer to
///a null-terminated UTF-8 string is returned in `data.valueString`.
///The pointer is valid for the lifetime of the `device` parameter
///passed to [`GetPerformanceParameterINTEL`].
///```c
///// Provided by VK_INTEL_performance_query
///typedef enum VkPerformanceValueTypeINTEL {
///    VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0,
///    VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1,
///    VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2,
///    VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3,
///    VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4,
///} VkPerformanceValueTypeINTEL;
///```
///# Related
/// - [`VK_INTEL_performance_query`]
/// - [`PerformanceValueINTEL`]
///
///# Notes and documentation
///For more information, see the [Vulkan specification](https://www.khronos.org/registry/vulkan/specs/1.3-extensions/html/vkspec.html)
///
///This documentation is generated from the Vulkan specification and documentation.
///The documentation is copyrighted by *The Khronos Group Inc.* and is licensed under *Creative
/// Commons Attribution 4.0 International*.
///This license explicitely allows adapting the source material as long as proper credit is given.
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "bytemuck", derive(Pod, Zeroable))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct PerformanceValueTypeINTEL(i32);
impl const Default for PerformanceValueTypeINTEL {
    fn default() -> Self {
        Self(0)
    }
}
impl std::fmt::Debug for PerformanceValueTypeINTEL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("PerformanceValueTypeINTEL")
            .field(match *self {
                Self::PERFORMANCE_VALUE_TYPE_UINT32 => &"PERFORMANCE_VALUE_TYPE_UINT32",
                Self::PERFORMANCE_VALUE_TYPE_UINT64 => &"PERFORMANCE_VALUE_TYPE_UINT64",
                Self::PERFORMANCE_VALUE_TYPE_FLOAT => &"PERFORMANCE_VALUE_TYPE_FLOAT",
                Self::PERFORMANCE_VALUE_TYPE_BOOL => &"PERFORMANCE_VALUE_TYPE_BOOL",
                Self::PERFORMANCE_VALUE_TYPE_STRING => &"PERFORMANCE_VALUE_TYPE_STRING",
                other => unreachable!("invalid value for `PerformanceValueTypeINTEL`: {:?}", other),
            })
            .finish()
    }
}
impl PerformanceValueTypeINTEL {
    ///No documentation found
    pub const PERFORMANCE_VALUE_TYPE_UINT32: Self = Self(0);
    ///No documentation found
    pub const PERFORMANCE_VALUE_TYPE_UINT64: Self = Self(1);
    ///No documentation found
    pub const PERFORMANCE_VALUE_TYPE_FLOAT: Self = Self(2);
    ///No documentation found
    pub const PERFORMANCE_VALUE_TYPE_BOOL: Self = Self(3);
    ///No documentation found
    pub const PERFORMANCE_VALUE_TYPE_STRING: Self = Self(4);
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
