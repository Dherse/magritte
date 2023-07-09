#[cfg(feature = "VK_KHR_global_priority")]
use crate::native::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::native::extensions::khr_global_priority::QueueGlobalPriorityKHR;
///See [`QueueGlobalPriorityKHR`]
#[doc(alias = "VkQueueGlobalPriorityEXT")]
pub type QueueGlobalPriorityEXT = QueueGlobalPriorityKHR;
///See [`DeviceQueueGlobalPriorityCreateInfoKHR`]
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoEXT")]
pub type DeviceQueueGlobalPriorityCreateInfoEXT = DeviceQueueGlobalPriorityCreateInfoKHR;
pub use crate::common::extensions::ext_global_priority::{
    EXT_GLOBAL_PRIORITY_EXTENSION_NAME, EXT_GLOBAL_PRIORITY_SPEC_VERSION,
};
