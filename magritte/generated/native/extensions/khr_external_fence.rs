use crate::native::vulkan1_1::{ExportFenceCreateInfo, FenceImportFlagBits, FenceImportFlags};
///See [`FenceImportFlags`]
#[doc(alias = "VkFenceImportFlagsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;
///See [`FenceImportFlagBits`]
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagBitsKHR = FenceImportFlagBits;
///See [`ExportFenceCreateInfo`]
#[doc(alias = "VkExportFenceCreateInfoKHR")]
pub type ExportFenceCreateInfoKHR = ExportFenceCreateInfo;
pub use crate::common::extensions::khr_external_fence::{
    KHR_EXTERNAL_FENCE_EXTENSION_NAME, KHR_EXTERNAL_FENCE_SPEC_VERSION,
};
