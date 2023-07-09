pub use crate::common::extensions::ext_global_priority_query::{
    EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME, EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION, MAX_GLOBAL_PRIORITY_SIZE_EXT,
};
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
pub type QueueFamilyGlobalPriorityPropertiesEXT = QueueFamilyGlobalPriorityPropertiesKHR;
