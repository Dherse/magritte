use crate::native::vulkan1_1::{ExportSemaphoreCreateInfo, SemaphoreImportFlagBits, SemaphoreImportFlags};
///See [`SemaphoreImportFlags`]
#[doc(alias = "VkSemaphoreImportFlagsKHR")]
pub type SemaphoreImportFlagsKHR = SemaphoreImportFlags;
///See [`SemaphoreImportFlagBits`]
#[doc(alias = "VkSemaphoreImportFlagBitsKHR")]
pub type SemaphoreImportFlagBitsKHR = SemaphoreImportFlagBits;
///See [`ExportSemaphoreCreateInfo`]
#[doc(alias = "VkExportSemaphoreCreateInfoKHR")]
pub type ExportSemaphoreCreateInfoKHR = ExportSemaphoreCreateInfo;
pub use crate::common::extensions::khr_external_semaphore::{
    KHR_EXTERNAL_SEMAPHORE_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_SPEC_VERSION,
};
