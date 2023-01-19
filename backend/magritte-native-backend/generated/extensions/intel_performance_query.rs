//!# [VK_INTEL_performance_query](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_INTEL_performance_query.html)
# ! [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VK_INTEL_performance_query.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, CommandBuffer, Device, Queue, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///See [`QueryPoolPerformanceQueryCreateInfoINTEL`]
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
///# [VkPerformanceValueINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceValueINTEL.md")]
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    #[doc(alias = "type")]
    type_: PerformanceValueTypeINTEL,
    data: PerformanceValueDataINTEL,
}
///# [VkInitializePerformanceApiInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkInitializePerformanceApiInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkInitializePerformanceApiInfoINTEL.md")]
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pUserData")]
    user_data: *mut std::ffi::c_void,
}
///# [VkQueryPoolPerformanceQueryCreateInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolPerformanceQueryCreateInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkQueryPoolPerformanceQueryCreateInfoINTEL.md")]
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "performanceCountersSampling")]
    performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
///# [VkPerformanceMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceMarkerInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceMarkerInfoINTEL.md")]
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    marker: u64,
}
///# [VkPerformanceStreamMarkerInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceStreamMarkerInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceStreamMarkerInfoINTEL.md")]
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    marker: u32,
}
///# [VkPerformanceOverrideInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceOverrideInfoINTEL.md")]
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: PerformanceOverrideTypeINTEL,
    enable: Bool32,
    parameter: u64,
}
///# [VkPerformanceConfigurationAcquireInfoINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationAcquireInfoINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceConfigurationAcquireInfoINTEL.md")]
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    type_: PerformanceConfigurationTypeINTEL,
}
///# [VkPerformanceValueDataINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueDataINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceValueDataINTEL.md")]
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceValueDataINTEL {
    value32: u32,
    value64: u64,
    #[doc(alias = "valueFloat")]
    value_float: f32,
    #[doc(alias = "valueBool")]
    value_bool: Bool32,
    #[doc(alias = "valueString")]
    value_string: *const CStr,
}
///# [VkPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceConfigurationINTEL.md")]
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);
impl PerformanceConfigurationINTEL {
    pub const fn null() -> Self {
        Self(0)
    }
}
impl const Default for PerformanceConfigurationINTEL {
    fn default() -> Self {
        Self::null()
    }
}
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_SPEC_VERSION")]
pub const INTEL_PERFORMANCE_QUERY_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_INTEL_PERFORMANCE_QUERY_EXTENSION_NAME")]
pub const INTEL_PERFORMANCE_QUERY_EXTENSION_NAME: &'static CStr = cstr!("VK_INTEL_performance_query");
///# [VkPerformanceConfigurationTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceConfigurationTypeINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceConfigurationTypeINTEL.md")]
#[doc(alias = "VkPerformanceConfigurationTypeINTEL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceConfigurationTypeINTEL(i32);
impl PerformanceConfigurationTypeINTEL {
    #[doc(alias = "VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL")]
    pub const COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED: Self = Self(0);
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
            x if x == Self::COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkQueryPoolSamplingModeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkQueryPoolSamplingModeINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkQueryPoolSamplingModeINTEL.md")]
#[doc(alias = "VkQueryPoolSamplingModeINTEL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct QueryPoolSamplingModeINTEL(i32);
impl QueryPoolSamplingModeINTEL {
    #[doc(alias = "VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL")]
    pub const MANUAL: Self = Self(0);
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
            x if x == Self::MANUAL.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPerformanceOverrideTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceOverrideTypeINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceOverrideTypeINTEL.md")]
#[doc(alias = "VkPerformanceOverrideTypeINTEL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceOverrideTypeINTEL(i32);
impl PerformanceOverrideTypeINTEL {
    #[doc(alias = "VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL")]
    pub const NULL_HARDWARE: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL")]
    pub const FLUSH_GPU_CACHES: Self = Self(1);
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
            x if x == Self::NULL_HARDWARE.bits() => Some(Self(x)),
            x if x == Self::FLUSH_GPU_CACHES.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPerformanceParameterTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceParameterTypeINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceParameterTypeINTEL.md")]
#[doc(alias = "VkPerformanceParameterTypeINTEL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceParameterTypeINTEL(i32);
impl PerformanceParameterTypeINTEL {
    #[doc(alias = "VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL")]
    pub const HW_COUNTERS_SUPPORTED: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL")]
    pub const STREAM_MARKER_VALID_BITS: Self = Self(1);
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
            x if x == Self::HW_COUNTERS_SUPPORTED.bits() => Some(Self(x)),
            x if x == Self::STREAM_MARKER_VALID_BITS.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [VkPerformanceValueTypeINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPerformanceValueTypeINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/VkPerformanceValueTypeINTEL.md")]
#[doc(alias = "VkPerformanceValueTypeINTEL")]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct PerformanceValueTypeINTEL(i32);
impl PerformanceValueTypeINTEL {
    #[doc(alias = "VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL")]
    pub const UINT32: Self = Self(0);
    #[doc(alias = "VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL")]
    pub const UINT64: Self = Self(1);
    #[doc(alias = "VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL")]
    pub const FLOAT: Self = Self(2);
    #[doc(alias = "VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL")]
    pub const BOOL: Self = Self(3);
    #[doc(alias = "VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL")]
    pub const STRING: Self = Self(4);
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
            x if x == Self::UINT32.bits() => Some(Self(x)),
            x if x == Self::UINT64.bits() => Some(Self(x)),
            x if x == Self::FLOAT.bits() => Some(Self(x)),
            x if x == Self::BOOL.bits() => Some(Self(x)),
            x if x == Self::STRING.bits() => Some(Self(x)),
            _ => None,
        }
    }
    ///Builds a bitmask from the bits of this variant without validating it
    #[inline]
    pub const unsafe fn from_bits_unchecked(bits: i32) -> Self {
        Self(bits)
    }
}
///# [vkInitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkInitializePerformanceApiINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkInitializePerformanceApiINTEL.md")]
#[doc(alias = "vkInitializePerformanceApiINTEL")]
pub type FNInitializePerformanceApiIntel = unsafe extern "system" fn(
    device: Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> VulkanResultCodes;
///# [vkUninitializePerformanceApiINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkUninitializePerformanceApiINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkUninitializePerformanceApiINTEL.md")]
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
pub type FNUninitializePerformanceApiIntel = unsafe extern "system" fn(device: Device);
///# [vkAcquirePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquirePerformanceConfigurationINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkAcquirePerformanceConfigurationINTEL.md")]
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
pub type FNAcquirePerformanceConfigurationIntel = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> VulkanResultCodes;
///# [vkReleasePerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleasePerformanceConfigurationINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkReleasePerformanceConfigurationINTEL.md")]
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
pub type FNReleasePerformanceConfigurationIntel =
    unsafe extern "system" fn(device: Device, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes;
///# [vkQueueSetPerformanceConfigurationINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkQueueSetPerformanceConfigurationINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkQueueSetPerformanceConfigurationINTEL.md")]
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
pub type FNQueueSetPerformanceConfigurationIntel =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes;
///# [vkGetPerformanceParameterINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetPerformanceParameterINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkGetPerformanceParameterINTEL.md")]
#[doc(alias = "vkGetPerformanceParameterINTEL")]
pub type FNGetPerformanceParameterIntel = unsafe extern "system" fn(
    device: Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL,
) -> VulkanResultCodes;
///# [vkCmdSetPerformanceMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceMarkerINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkCmdSetPerformanceMarkerINTEL.md")]
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
pub type FNCmdSetPerformanceMarkerIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL,
) -> VulkanResultCodes;
///# [vkCmdSetPerformanceStreamMarkerINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceStreamMarkerINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkCmdSetPerformanceStreamMarkerINTEL.md")]
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
pub type FNCmdSetPerformanceStreamMarkerIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> VulkanResultCodes;
///# [vkCmdSetPerformanceOverrideINTEL](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCmdSetPerformanceOverrideINTEL.html)
# [doc = include_str ! ("../../../../doc/extensions/intel_performance_query/vkCmdSetPerformanceOverrideINTEL.md")]
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
pub type FNCmdSetPerformanceOverrideIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL,
) -> VulkanResultCodes;
