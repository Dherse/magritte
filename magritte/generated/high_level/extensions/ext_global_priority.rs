pub use crate::common::extensions::ext_global_priority::{
    EXT_GLOBAL_PRIORITY_EXTENSION_NAME, EXT_GLOBAL_PRIORITY_SPEC_VERSION,
};
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::QueueGlobalPriorityKHR;
#[doc(alias = "VkQueueGlobalPriorityEXT")]
pub type QueueGlobalPriorityEXT = QueueGlobalPriorityKHR;
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoEXT")]
pub type DeviceQueueGlobalPriorityCreateInfoEXT = DeviceQueueGlobalPriorityCreateInfoKHR;
