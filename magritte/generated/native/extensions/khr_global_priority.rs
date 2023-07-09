use crate::native::vulkan1_0::{
    BaseInStructure, BaseOutStructure, Bool32, StructureType, MAX_GLOBAL_PRIORITY_SIZE_KHR,
};
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DeviceQueueGlobalPriorityCreateInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "globalPriority")]
    pub global_priority: QueueGlobalPriorityKHR,
}
impl Default for DeviceQueueGlobalPriorityCreateInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::DeviceQueueGlobalPriorityCreateInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            global_priority: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "globalPriorityQuery")]
    pub global_priority_query: Bool32,
}
impl Default for PhysicalDeviceGlobalPriorityQueryFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDeviceGlobalPriorityQueryFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            global_priority_query: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct QueueFamilyGlobalPriorityPropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "priorityCount")]
    pub priority_count: u32,
    pub priorities: [QueueGlobalPriorityKHR; MAX_GLOBAL_PRIORITY_SIZE_KHR as usize],
}
impl Default for QueueFamilyGlobalPriorityPropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::QueueFamilyGlobalPriorityPropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            priority_count: unsafe { std::mem::zeroed() },
            priorities: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_global_priority::{
    QueueGlobalPriorityKHR, KHR_GLOBAL_PRIORITY_EXTENSION_NAME, KHR_GLOBAL_PRIORITY_SPEC_VERSION,
};
