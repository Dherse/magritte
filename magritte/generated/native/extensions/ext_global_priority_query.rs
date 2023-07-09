#[cfg(feature = "VK_KHR_global_priority")]
use crate::native::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::native::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
///See [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
///See [`QueueFamilyGlobalPriorityPropertiesKHR`]
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
pub type QueueFamilyGlobalPriorityPropertiesEXT = QueueFamilyGlobalPriorityPropertiesKHR;
pub use crate::common::extensions::ext_global_priority_query::{
    EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME, EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION, MAX_GLOBAL_PRIORITY_SIZE_EXT,
};
