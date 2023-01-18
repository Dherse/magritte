use crate::{
    cstr,
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
use std::ffi::CStr;
#[doc(alias = "VkImportFenceWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    fence: Fence,
    flags: FenceImportFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalFenceHandleTypeFlagBits,
    handle: HANDLE,
    name: LPCWSTR,
}
#[doc(alias = "VkExportFenceWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
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
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    fence: Fence,
    #[doc(alias = "handleType")]
    handle_type: ExternalFenceHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_fence_win32");
#[doc(alias = "vkGetFenceWin32HandleKHR")]
pub type FNGetFenceWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
#[doc(alias = "vkImportFenceWin32HandleKHR")]
pub type FNImportFenceWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> VulkanResultCodes;
