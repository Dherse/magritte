//!# [VK_KHR_vulkan_memory_model](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_vulkan_memory_model.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_vulkan_memory_model/VK_KHR_vulkan_memory_model.md")]
use crate::{cstr, vulkan1_2::PhysicalDeviceVulkanMemoryModelFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceVulkanMemoryModelFeatures`]
#[doc(alias = "VkPhysicalDeviceVulkanMemoryModelFeaturesKHR")]
pub type PhysicalDeviceVulkanMemoryModelFeaturesKHR = PhysicalDeviceVulkanMemoryModelFeatures;
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION")]
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME")]
pub const KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_vulkan_memory_model");
