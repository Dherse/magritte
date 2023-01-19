//!# [VK_KHR_8bit_storage](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_8bit_storage.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_8bit_storage/VK_KHR_8bit_storage.md")]
use crate::{cstr, vulkan1_2::PhysicalDevice8BitStorageFeatures};
use std::ffi::CStr;
///See [`PhysicalDevice8BitStorageFeatures`]
#[doc(alias = "VkPhysicalDevice8BitStorageFeaturesKHR")]
pub type PhysicalDevice8BitStorageFeaturesKHR = PhysicalDevice8BitStorageFeatures;
#[doc(alias = "VK_KHR_8BIT_STORAGE_SPEC_VERSION")]
pub const KHR_8BIT_STORAGE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_8BIT_STORAGE_EXTENSION_NAME")]
pub const KHR_8BIT_STORAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_8bit_storage");
