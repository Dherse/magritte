use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION")]
pub const KHR_VULKAN_MEMORY_MODEL_SPEC_VERSION: u32 = 3;
#[doc(alias = "VK_KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME")]
pub const KHR_VULKAN_MEMORY_MODEL_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_vulkan_memory_model");
