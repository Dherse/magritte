use crate::cstr;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::DeviceQueueGlobalPriorityCreateInfoKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::QueueGlobalPriorityKHR;
use std::ffi::CStr;
///See [`QueueGlobalPriorityKHR`]
#[doc(alias = "VkQueueGlobalPriorityEXT")]
pub type QueueGlobalPriorityEXT = QueueGlobalPriorityKHR;
///See [`DeviceQueueGlobalPriorityCreateInfoKHR`]
#[doc(alias = "VkDeviceQueueGlobalPriorityCreateInfoEXT")]
pub type DeviceQueueGlobalPriorityCreateInfoEXT = DeviceQueueGlobalPriorityCreateInfoKHR;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_global_priority");
