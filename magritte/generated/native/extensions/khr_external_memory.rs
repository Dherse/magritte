use crate::native::vulkan1_1::{
    ExportMemoryAllocateInfo, ExternalMemoryBufferCreateInfo, ExternalMemoryImageCreateInfo,
};
///See [`ExternalMemoryImageCreateInfo`]
#[doc(alias = "VkExternalMemoryImageCreateInfoKHR")]
pub type ExternalMemoryImageCreateInfoKHR = ExternalMemoryImageCreateInfo;
///See [`ExternalMemoryBufferCreateInfo`]
#[doc(alias = "VkExternalMemoryBufferCreateInfoKHR")]
pub type ExternalMemoryBufferCreateInfoKHR = ExternalMemoryBufferCreateInfo;
///See [`ExportMemoryAllocateInfo`]
#[doc(alias = "VkExportMemoryAllocateInfoKHR")]
pub type ExportMemoryAllocateInfoKHR = ExportMemoryAllocateInfo;
pub use crate::common::extensions::khr_external_memory::{
    KHR_EXTERNAL_MEMORY_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_SPEC_VERSION, QUEUE_FAMILY_EXTERNAL_KHR,
};
