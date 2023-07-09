use crate::native::{
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
#[doc(alias = "VkImportFenceWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportFenceWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub fence: Fence,
    pub flags: FenceImportFlags,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl Default for ImportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportFenceWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fence: unsafe { std::mem::zeroed() },
            flags: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            handle: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportFenceWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportFenceWin32HandleInfoKHR {
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
impl Default for ExportFenceWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportFenceWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            attributes: unsafe { std::mem::zeroed() },
            dw_access: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkFenceGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FenceGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub fence: Fence,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalFenceHandleTypeFlagBits,
}
impl Default for FenceGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::FenceGetWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            fence: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_fence_win32::{
    KHR_EXTERNAL_FENCE_WIN32_EXTENSION_NAME, KHR_EXTERNAL_FENCE_WIN32_SPEC_VERSION,
};
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
