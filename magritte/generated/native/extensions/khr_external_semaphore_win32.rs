use crate::native::{
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, Semaphore, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalSemaphoreHandleTypeFlagBits, SemaphoreImportFlags},
};
#[doc(alias = "VkImportSemaphoreWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportSemaphoreWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    pub flags: SemaphoreImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl Default for ImportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportSemaphoreWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            handle: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportSemaphoreWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportSemaphoreWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "pAttributes")]
    pub attributes: *const SECURITY_ATTRIBUTES,
    #[doc(alias = "dwAccess")]
    pub dw_access: DWORD,
    pub name: LPCWSTR,
}
impl Default for ExportSemaphoreWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportSemaphoreWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            attributes: unsafe { std::mem::zeroed() },
            dw_access: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkD3D12FenceSubmitInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct D3d12FenceSubmitInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "waitSemaphoreValuesCount")]
    pub wait_semaphore_values_count: u32,
    #[doc(alias = "pWaitSemaphoreValues")]
    pub wait_semaphore_values: *const u64,
    #[doc(alias = "signalSemaphoreValuesCount")]
    pub signal_semaphore_values_count: u32,
    #[doc(alias = "pSignalSemaphoreValues")]
    pub signal_semaphore_values: *const u64,
}
impl Default for D3d12FenceSubmitInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::D3d12FenceSubmitInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            wait_semaphore_values_count: unsafe { std::mem::zeroed() },
            wait_semaphore_values: unsafe { std::mem::zeroed() },
            signal_semaphore_values_count: unsafe { std::mem::zeroed() },
            signal_semaphore_values: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkSemaphoreGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SemaphoreGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub semaphore: Semaphore,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalSemaphoreHandleTypeFlagBits,
}
impl Default for SemaphoreGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SemaphoreGetWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            semaphore: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_semaphore_win32::{
    KHR_EXTERNAL_SEMAPHORE_WIN32_EXTENSION_NAME, KHR_EXTERNAL_SEMAPHORE_WIN32_SPEC_VERSION,
};
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
