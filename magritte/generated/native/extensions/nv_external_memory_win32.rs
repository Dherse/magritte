use crate::native::{
    extensions::nv_external_memory_capabilities::ExternalMemoryHandleTypeFlagsNV,
    opaque::{DWORD, HANDLE, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkImportMemoryWin32HandleInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagsNV,
    pub handle: HANDLE,
}
impl Default for ImportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryWin32HandleInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            handle: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportMemoryWin32HandleInfoNV")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoNV {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pAttributes")]
    pub attributes: *const SECURITY_ATTRIBUTES,
    #[doc(alias = "dwAccess")]
    pub dw_access: DWORD,
}
impl Default for ExportMemoryWin32HandleInfoNV {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportMemoryWin32HandleInfoNv,
            p_next: unsafe { std::mem::zeroed() },
            attributes: unsafe { std::mem::zeroed() },
            dw_access: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::nv_external_memory_win32::{
    NV_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME, NV_EXTERNAL_MEMORY_WIN32_SPEC_VERSION,
};
#[doc(alias = "vkGetMemoryWin32HandleNV")]
pub type FNGetMemoryWin32HandleNv = unsafe extern "system" fn(
    device: Device,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
