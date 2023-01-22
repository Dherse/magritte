//!# [VK_KHR_external_memory_fd](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_memory_fd.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/VK_KHR_external_memory_fd.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, BaseOutStructure, Device, DeviceMemory, StructureType, VulkanResultCodes},
    vulkan1_1::ExternalMemoryHandleTypeFlagBits,
};
use std::ffi::CStr;
///# [VkImportMemoryFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportMemoryFdInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/VkImportMemoryFdInfoKHR.md")]
#[doc(alias = "VkImportMemoryFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportMemoryFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
    fd: i32,
}
///# [VkMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryFdPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/VkMemoryFdPropertiesKHR.md")]
#[doc(alias = "VkMemoryFdPropertiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryFdPropertiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "memoryTypeBits")]
    memory_type_bits: u32,
}
///# [VkMemoryGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkMemoryGetFdInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/VkMemoryGetFdInfoKHR.md")]
#[doc(alias = "VkMemoryGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MemoryGetFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    memory: DeviceMemory,
    #[doc(alias = "handleType")]
    handle_type: ExternalMemoryHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_MEMORY_FD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_MEMORY_FD_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_memory_fd");
///# [vkGetMemoryFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/vkGetMemoryFdKHR.md")]
#[doc(alias = "vkGetMemoryFdKHR")]
pub type FNGetMemoryFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const MemoryGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
///# [vkGetMemoryFdPropertiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetMemoryFdPropertiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_memory_fd/vkGetMemoryFdPropertiesKHR.md")]
#[doc(alias = "vkGetMemoryFdPropertiesKHR")]
pub type FNGetMemoryFdPropertiesKhr = unsafe extern "system" fn(
    device: Device,
    handle_type: ExternalMemoryHandleTypeFlagBits,
    fd: i32,
    p_memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> VulkanResultCodes;