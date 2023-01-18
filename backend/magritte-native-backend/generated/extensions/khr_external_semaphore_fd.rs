use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
use std::ffi::CStr;
#[doc(alias = "VkImportSemaphoreFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    fd: i32,
}
#[doc(alias = "VkSemaphoreGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_FD_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_semaphore_fd");
#[doc(alias = "vkGetSemaphoreFdKHR")]
pub type FNGetSemaphoreFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const SemaphoreGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
#[doc(alias = "vkImportSemaphoreFdKHR")]
pub type FNImportSemaphoreFdKhr = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> VulkanResultCodes;
