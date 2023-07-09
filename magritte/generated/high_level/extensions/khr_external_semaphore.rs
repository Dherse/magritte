pub use crate::common::extensions::khr_external_semaphore::{
    KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION,
};
use crate::vulkan1_1::{ExportSemaphoreCreateInfo, SemaphoreImportFlagBits, SemaphoreImportFlags};
#[doc(alias = "VkSemaphoreImportFlagsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagBitsKHR = SemaphoreImportFlagBits;
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
pub type ExportSemaphoreCreateInfoKHR = ExportSemaphoreCreateInfo;
