use crate::native::{
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
#[doc(alias = "VkImportMemoryWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
    pub handle: HANDLE,
    pub name: LPCWSTR,
}
impl Default for ImportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ImportMemoryWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
            handle: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkExportMemoryWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
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
impl Default for ExportMemoryWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::ExportMemoryWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            attributes: unsafe { std::mem::zeroed() },
            dw_access: unsafe { std::mem::zeroed() },
            name: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryWin32HandlePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    pub memory_type_bits: u32,
}
impl Default for MemoryWin32HandlePropertiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryWin32HandlePropertiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory_type_bits: unsafe { std::mem::zeroed() },
        }
    }
}
#[doc(alias = "VkMemoryGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    pub memory: DeviceMemory,
    #[doc(alias = "handleType")]
    pub handle_type: ExternalMemoryHandleTypeFlagBits,
}
impl Default for MemoryGetWin32HandleInfoKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::MemoryGetWin32HandleInfoKhr,
            p_next: unsafe { std::mem::zeroed() },
            memory: unsafe { std::mem::zeroed() },
            handle_type: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_external_memory_win32::{
    KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME, KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION,
};
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
pub type FNGetMemoryWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
pub type FNGetMemoryWin32HandlePropertiesKhr = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> VulkanResultCodes;
