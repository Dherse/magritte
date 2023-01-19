//!# [VK_KHR_external_memory_win32](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_win32.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/VK_KHR_external_memory_win32.md")]
use crate::{
    cstr,
    opaque::{DWORD, HANDLE, LPCWSTR, SECURITY_ATTRIBUTES},
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::ffi::CStr;
///# [VkImportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryWin32HandleInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/VkImportMemoryWin32HandleInfoKHR.md")]
#[doc(alias = "VkImportMemoryWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    name: LPCWSTR,
}
///# [VkExportMemoryWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkExportMemoryWin32HandleInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/VkExportMemoryWin32HandleInfoKHR.md")]
#[doc(alias = "VkExportMemoryWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExportMemoryWin32HandleInfoKHR {
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
///# [VkMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryWin32HandlePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/VkMemoryWin32HandlePropertiesKHR.md")]
#[doc(alias = "VkMemoryWin32HandlePropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryWin32HandlePropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
}
///# [VkMemoryGetWin32HandleInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetWin32HandleInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/VkMemoryGetWin32HandleInfoKHR.md")]
#[doc(alias = "VkMemoryGetWin32HandleInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetWin32HandleInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    memory: DeviceMemory,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_WIN32_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_WIN32_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_memory_win32");
///# [vkGetMemoryWin32HandleKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandleKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/vkGetMemoryWin32HandleKHR.md")]
#[doc(alias = "vkGetMemoryWin32HandleKHR")]
pub type FNGetMemoryWin32HandleKhr = unsafe extern "system" fn(
    device: Device,
    p_get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    p_handle: *mut HANDLE,
) -> VulkanResultCodes;
///# [vkGetMemoryWin32HandlePropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryWin32HandlePropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_win32/vkGetMemoryWin32HandlePropertiesKHR.md")]
#[doc(alias = "vkGetMemoryWin32HandlePropertiesKHR")]
pub type FNGetMemoryWin32HandlePropertiesKhr = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    handle: HANDLE,
    p_memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> VulkanResultCodes;
