use crate::{
    cstr,
    vulkan1_1::{ExportFenceCreateInfo, FenceImportFlagBits, FenceImportFlags},
};
use std::ffi::CStr;
///See [`FenceImportFlags`]
#[doc(alias = "VkFenceImportFlagsKHR")]
pub type FenceImportFlagsKHR = FenceImportFlags;
///See [`FenceImportFlagBits`]
#[doc(alias = "VkFenceImportFlagBitsKHR")]
pub type FenceImportFlagBitsKHR = FenceImportFlagBits;
///See [`ExportFenceCreateInfo`]
#[doc(alias = "VkExportFenceCreateInfoKHR")]
pub type ExportFenceCreateInfoKHR = ExportFenceCreateInfo;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_fence");
