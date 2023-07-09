pub use crate::common::extensions::khr_external_fence::{
    KHR_EXTERNAL_FENCE_EXTENSION_NAME, KHR_EXTERNAL_FENCE_SPEC_VERSION,
};
use crate::vulkan1_1::{ExportFenceCreateInfo, FenceImportFlagBits, FenceImportFlags};
#[doc(alias = "VkFenceImportFlagsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagBitsKHR = FenceImportFlagBits;
#[doc(alias = "VkExportFenceCreateInfoKHR")]
pub type ExportFenceCreateInfoKHR = ExportFenceCreateInfo;
