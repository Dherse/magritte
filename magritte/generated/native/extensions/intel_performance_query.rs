use crate::native::vulkan1_0::{
    BaseInStructure, Bool32, CommandBuffer, Device, Queue, StructureType, VulkanResultCodes,
};
use std::ffi::c_char;
///See [`QueryPoolPerformanceQueryCreateInfoINTEL`]
#[doc(alias = "VkQueryPoolCreateInfoINTEL")]
pub type QueryPoolCreateInfoINTEL = QueryPoolPerformanceQueryCreateInfoINTEL;
#[doc(alias = "VkPerformanceValueINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceValueINTEL {
    #[doc(alias = "type")]
    pub type_: PerformanceValueTypeINTEL,
    pub data: PerformanceValueDataINTEL,
}
impl Default for PerformanceValueINTEL {
    fn default() -> Self {
        Self {
            type_: unsafe { std::mem::zeroed() },
            data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkInitializePerformanceApiInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct InitializePerformanceApiInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pUserData")]
    pub user_data: *mut std::ffi::c_void,
}
impl Default for InitializePerformanceApiInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::InitializePerformanceApiInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            user_data: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueryPoolPerformanceQueryCreateInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceQueryCreateInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "performanceCountersSampling")]
    pub performance_counters_sampling: QueryPoolSamplingModeINTEL,
}
impl Default for QueryPoolPerformanceQueryCreateInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueryPoolPerformanceQueryCreateInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            performance_counters_sampling: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceMarkerInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceMarkerInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub marker: u64,
}
impl Default for PerformanceMarkerInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceMarkerInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            marker: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceStreamMarkerInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceStreamMarkerInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub marker: u32,
}
impl Default for PerformanceStreamMarkerInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceStreamMarkerInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            marker: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceOverrideInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceOverrideInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: PerformanceOverrideTypeINTEL,
    pub enable: Bool32,
    pub parameter: u64,
}
impl Default for PerformanceOverrideInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceOverrideInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
            enable: unsafe { std::mem::zeroed() },
            parameter: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceConfigurationAcquireInfoINTEL")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceConfigurationAcquireInfoINTEL {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "type")]
    pub type_: PerformanceConfigurationTypeINTEL,
}
impl Default for PerformanceConfigurationAcquireInfoINTEL {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceConfigurationAcquireInfoIntel,
            p_next: unsafe { std::mem::zeroed() },
            type_: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceValueDataINTEL")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceValueDataINTEL {
    pub value32: u32,
    pub value64: u64,
    #[doc(alias = "valueFloat")]
    pub value_float: f32,
    #[doc(alias = "valueBool")]
    pub value_bool: Bool32,
    #[doc(alias = "valueString")]
    pub value_string: *const c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[doc(alias = "VkPerformanceConfigurationINTEL")]
#[repr(transparent)]
pub struct PerformanceConfigurationINTEL(u64);
impl PerformanceConfigurationINTEL {
    pub const fn null() -> Self {
        Self(0)
    }
    pub const fn raw(&self) -> u64 {
        self.0
    }
}
impl Default for PerformanceConfigurationINTEL {
    fn default() -> Self {
        Self::null()
    }
}
pub use crate::common::extensions::intel_performance_query::{
    PerformanceConfigurationTypeINTEL, PerformanceOverrideTypeINTEL, PerformanceParameterTypeINTEL,
    PerformanceValueTypeINTEL, QueryPoolSamplingModeINTEL, INTEL_PERFORMANCE_QUERY_EXTENSION_NAME,
    INTEL_PERFORMANCE_QUERY_SPEC_VERSION,
};
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
