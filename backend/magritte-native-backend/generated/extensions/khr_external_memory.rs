//!# [VK_KHR_external_memory](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_memory/VK_KHR_external_memory.md")]
use crate::{
    cstr,
    vulkan1_0::QUEUE_FAMILY_EXTERNAL,
    vulkan1_1::{ExportMemoryAllocateInfo, ExternalMemoryBufferCreateInfo, ExternalMemoryImageCreateInfo},
};
use std::ffi::CStr;
///See [`ExternalMemoryImageCreateInfo`]
#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
pub type ExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfo;
///See [`ExternalMemoryBufferCreateInfo`]
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
pub type ExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfo;
///See [`ExportMemoryAllocateInfo`]
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
pub type ExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfo;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_memory");
///See [`QUEUE_FAMILY_EXTERNAL`]
#[doc(alias = "VK_QUEUE_FAMILY_EXTERNAL_KHR")]
pub const QUEUE_FAMILY_EXTERNAL_KHR: u32 = QUEUE_FAMILY_EXTERNAL;
