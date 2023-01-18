use crate::{
    cstr,
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    opaque::{DWORD, HANDLE, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    handle: HANDLE,
}
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "pAttributes")]
    attributes: *const SECURITY_ATTRIBUTES,
    #[doc(alias = "dwAccess")]
    dw_access: DWORD,
}
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
pub const NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
pub const NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_external_memory_win32");
#[doc(alias = "vkGetMemoryWin32HandleNV")]
pub type FNGetMemoryWin32HandleNv = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
