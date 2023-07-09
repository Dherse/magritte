use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, Device, PhysicalDevice, StructureType, VulkanResultCodes,
    MAX_DESCRIPTION_SIZE,
};
use uuid::Uuid;
#[doc(alias = "VkPhysicalDevicePerformanceQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "performanceCounterQueryPools")]
    pub performance_counter_query_pools: Bool32,
    #[doc(alias = "performanceCounterMultipleQueryPools")]
    pub performance_counter_multiple_query_pools: Bool32,
}
impl Default for PhysicalDevicePerformanceQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePerformanceQueryFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            performance_counter_query_pools: unsafe { std::mem::zeroed() },
            performance_counter_multiple_query_pools: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDevicePerformanceQueryPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePerformanceQueryPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "allowCommandBufferQueryCopies")]
    pub allow_command_buffer_query_copies: Bool32,
}
impl Default for PhysicalDevicePerformanceQueryPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePerformanceQueryPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            allow_command_buffer_query_copies: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceCounterKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub unit: PerformanceCounterUnitKHR,
    pub scope: PerformanceCounterScopeKHR,
    pub storage: PerformanceCounterStorageKHR,
    pub uuid: Uuid,
}
impl Default for PerformanceCounterKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceCounterKhr,
            p_next: unsafe { std::mem::zeroed() },
            unit: unsafe { std::mem::zeroed() },
            scope: unsafe { std::mem::zeroed() },
            storage: unsafe { std::mem::zeroed() },
            uuid: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceCounterDescriptionKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceCounterDescriptionKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    pub flags: PerformanceCounterDescriptionFlagsKHR,
    pub name: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub category: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
    pub description: [std::ffi::c_char; MAX_DESCRIPTION_SIZE as usize],
}
impl Default for PerformanceCounterDescriptionKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceCounterDescriptionKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
            category: unsafe { std::mem::zeroed() },
            description: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueryPoolPerformanceCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueryPoolPerformanceCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "queueFamilyIndex")]
    pub queue_family_index: u32,
    #[doc(alias = "counterIndexCount")]
    pub counter_index_count: u32,
    #[doc(alias = "pCounterIndices")]
    pub counter_indices: *const u32,
}
impl Default for QueryPoolPerformanceCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueryPoolPerformanceCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            queue_family_index: unsafe { std::mem::zeroed() },
            counter_index_count: unsafe { std::mem::zeroed() },
            counter_indices: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkAcquireProfilingLockInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AcquireProfilingLockInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub flags: AcquireProfilingLockFlagsKHR,
    pub timeout: u64,
}
impl Default for AcquireProfilingLockInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::AcquireProfilingLockInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            timeout: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceQuerySubmitInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PerformanceQuerySubmitInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "counterPassIndex")]
    pub counter_pass_index: u32,
}
impl Default for PerformanceQuerySubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PerformanceQuerySubmitInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            counter_pass_index: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPerformanceCounterResultKHR")]
#[repr(C)]
#[derive(Clone, Copy)]
pub union PerformanceCounterResultKHR {
    pub int32: i32,
    pub int64: i64,
    pub uint32: u32,
    pub uint64: u64,
    pub float32: f32,
    pub float64: f64,
}
pub use crate::common::extensions::khr_performance_query::{
    AcquireProfilingLockFlagBitsKHR, AcquireProfilingLockFlagsKHR, PerformanceCounterDescriptionFlagBitsKHR,
    PerformanceCounterDescriptionFlagsKHR, PerformanceCounterScopeKHR, PerformanceCounterStorageKHR,
    PerformanceCounterUnitKHR, KHR_PERFORMANCE_QUERY_EXTENSION_NAME, KHR_PERFORMANCE_QUERY_SPEC_VERSION,
};
#[doc(alias = "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR")]
pub type FNEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKhr =
    unsafe extern "system" fn(
        physical_device: PhysicalDevice,
        queue_family_index: u32,
        p_counter_count: *mut u32,
        p_counters: *mut PerformanceCounterKHR,
        p_counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> VulkanResultCodes;
#[doc(alias = "vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR")]
pub type FNGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKhr = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    p_performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
    p_num_passes: *mut u32,
);
#[doc(alias = "vkAcquireProfilingLockKHR")]
pub type FNAcquireProfilingLockKhr =
    unsafe extern "system" fn(device: Device, p_info: *const AcquireProfilingLockInfoKHR) -> VulkanResultCodes;
#[doc(alias = "vkReleaseProfilingLockKHR")]
pub type FNReleaseProfilingLockKhr = unsafe extern "system" fn(device: Device);
