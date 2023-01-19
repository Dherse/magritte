//!# [VK_KHR_external_semaphore](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_semaphore.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_semaphore/VK_KHR_external_semaphore.md")]
use crate::{
    cstr,
    vulkan1_1::{ExportSemaphoreCreateInfo, SemaphoreImportFlagBits, SemaphoreImportFlags},
};
use std::ffi::CStr;
///See [`SemaphoreImportFlags`]
#[doc(alias = "VkSemaphoreImportFlagsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
///See [`SemaphoreImportFlagBits`]
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagBitsKHR = SemaphoreImportFlagBits;
///See [`ExportSemaphoreCreateInfo`]
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
pub type ExportSemaphoreCreateInfoKHR = ExportSemaphoreCreateInfo;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_semaphore");
