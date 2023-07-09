pub use crate::common::extensions::khr_external_memory::{
    KHR_EXTERNAL_MEMORY_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_SPEC_VERSION, QUEUE_FAMILY_EXTERNAL_KHR,
};
use crate::vulkan1_1::{ExportMemoryAllocateInfo, ExternalMemoryBufferCreateInfo, ExternalMemoryImageCreateInfo};
#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
pub type ExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfo;
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
pub type ExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfo;
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
pub type ExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfo;
