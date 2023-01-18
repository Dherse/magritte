use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Bool32, CommandBuffer, Device, Queue, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///See [`QueryPoolPerformanceQueryCreateInfoINTEL`]
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    #[doc(alias = "type")]
    type_: PerformanceValueTypeINTEL,
    data: PerformanceValueDataINTEL,
}
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
#[doc(alias = "vkInitializePerformanceApiINTEL")]
pub type FNInitializePerformanceApiIntel = unsafe extern "system" fn(
    device: Device,
    p_initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> VulkanResultCodes;
#[doc(alias = "vkUninitializePerformanceApiINTEL")]
pub type FNUninitializePerformanceApiIntel = unsafe extern "system" fn(device: Device);
#[doc(alias = "vkAcquirePerformanceConfigurationINTEL")]
pub type FNAcquirePerformanceConfigurationIntel = unsafe extern "system" fn(
    device: Device,
    p_acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    p_configuration: *mut PerformanceConfigurationINTEL,
) -> VulkanResultCodes;
#[doc(alias = "vkReleasePerformanceConfigurationINTEL")]
pub type FNReleasePerformanceConfigurationIntel =
    unsafe extern "system" fn(device: Device, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes;
#[doc(alias = "vkQueueSetPerformanceConfigurationINTEL")]
pub type FNQueueSetPerformanceConfigurationIntel =
    unsafe extern "system" fn(queue: Queue, configuration: PerformanceConfigurationINTEL) -> VulkanResultCodes;
#[doc(alias = "vkGetPerformanceParameterINTEL")]
pub type FNGetPerformanceParameterIntel = unsafe extern "system" fn(
    device: Device,
    parameter: PerformanceParameterTypeINTEL,
    p_value: *mut PerformanceValueINTEL,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSetPerformanceMarkerINTEL")]
pub type FNCmdSetPerformanceMarkerIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceMarkerInfoINTEL,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSetPerformanceStreamMarkerINTEL")]
pub type FNCmdSetPerformanceStreamMarkerIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> VulkanResultCodes;
#[doc(alias = "vkCmdSetPerformanceOverrideINTEL")]
pub type FNCmdSetPerformanceOverrideIntel = unsafe extern "system" fn(
    command_buffer: CommandBuffer,
    p_override_info: *const PerformanceOverrideInfoINTEL,
) -> VulkanResultCodes;
