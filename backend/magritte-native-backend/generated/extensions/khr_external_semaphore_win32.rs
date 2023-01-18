use crate::{
    cstr,
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
use std::ffi::CStr;
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
    handle: HANDLE,
    name: LPCWSTR,
}
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pAttributes")]
    attributes: *const SECURITY_ATTRIBUTES,
    #[doc(alias = "dwAccess")]
    dw_access: DWORD,
    name: LPCWSTR,
}
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct D3d12FenceSubmitInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreValuesCount")]
    wait_semaphore_values_count: u32,
    #[doc(alias = "pWaitSemaphoreValues")]
    wait_semaphore_values: *const u64,
    #[doc(alias = "signalSemaphoreValuesCount")]
    signal_semaphore_values_count: u32,
    #[doc(alias = "pSignalSemaphoreValues")]
    signal_semaphore_values: *const u64,
}
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    semaphore: Semaphore,
    #[doc(alias = "handleType")]
    handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_semaphore_win32");
#[doc(alias = "vkGetSemaphoreWin32HandleKHR")]
pub type FNGetSemaphoreWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
#[doc(alias = "vkImportSemaphoreWin32HandleKHR")]
pub type FNImportSemaphoreWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> VulkanResultCodes;
