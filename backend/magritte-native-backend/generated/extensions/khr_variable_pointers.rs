//!# [VK_KHR_variable_pointers](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_variable_pointers.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_variable_pointers/VK_KHR_variable_pointers.md")]
use crate::{cstr, vulkan1_1::PhysicalDeviceVariablePointersFeatures};
use std::ffi::CStr;
///See [`PhysicalDeviceVariablePointersFeatures`]
#[doc(alias = "VkPhysicalDeviceVariablePointersFeaturesKHR")]
pub type PhysicalDeviceVariablePointersFeaturesKHR = PhysicalDeviceVariablePointersFeatures;
///See [`PhysicalDeviceVariablePointersFeatures`]
#[doc(alias = "VkPhysicalDeviceVariablePointerFeaturesKHR")]
pub type PhysicalDeviceVariablePointerFeaturesKHR = PhysicalDeviceVariablePointersFeatures;
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_SPEC_VERSION")]
pub const KHR_VARIABLE_POINTERS_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_VARIABLE_POINTERS_EXTENSION_NAME")]
pub const KHR_VARIABLE_POINTERS_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_variable_pointers");
