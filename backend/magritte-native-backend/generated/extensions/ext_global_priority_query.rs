#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
#[cfg(feature = "VK_KHR_global_priority")]
use crate::extensions::khr_global_priority::QueueFamilyGlobalPriorityPropertiesKHR;
use crate::{cstr, vulkan1_0::MAX_GLOBAL_PRIORITY_SIZE_KHR};
use std::ffi::CStr;
///See [`PhysicalDeviceGlobalPriorityQueryFeaturesKHR`]
#[doc(alias = "VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT")]
pub type PhysicalDeviceGlobalPriorityQueryFeaturesEXT = PhysicalDeviceGlobalPriorityQueryFeaturesKHR;
///See [`QueueFamilyGlobalPriorityPropertiesKHR`]
#[doc(alias = "VkQueueFamilyGlobalPriorityPropertiesEXT")]
pub type QueueFamilyGlobalPriorityPropertiesEXT = QueueFamilyGlobalPriorityPropertiesKHR;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_QUERY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_QUERY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_global_priority_query");
///See [`MAX_GLOBAL_PRIORITY_SIZE_KHR`]
#[doc(alias = "VK_MAX_GLOBAL_PRIORITY_SIZE_EXT")]
pub const MAX_GLOBAL_PRIORITY_SIZE_EXT: u32 = MAX_GLOBAL_PRIORITY_SIZE_KHR;
