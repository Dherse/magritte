//!# [VK_KHR_external_fence_fd](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_external_fence_fd.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_fd/VK_KHR_external_fence_fd.md")]
use crate::{
    cstr,
    vulkan1_0::{BaseInStructure, Device, Fence, StructureType, VulkanResultCodes},
    vulkan1_1::{ExternalFenceHandleTypeFlagBits, FenceImportFlags},
};
use std::ffi::CStr;
///# [VkImportFenceFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkImportFenceFdInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_fd/VkImportFenceFdInfoKHR.md")]
#[doc(alias = "VkImportFenceFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ImportFenceFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    fence: Fence,
    flags: FenceImportFlags,
    #[doc(alias = "handleType")]
    handle_type: ExternalFenceHandleTypeFlagBits,
    fd: i32,
}
///# [VkFenceGetFdInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkFenceGetFdInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_fd/VkFenceGetFdInfoKHR.md")]
#[doc(alias = "VkFenceGetFdInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct FenceGetFdInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    fence: Fence,
    #[doc(alias = "handleType")]
    handle_type: ExternalFenceHandleTypeFlagBits,
}
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_SPEC_VERSION")]
pub const KHR_EXTERNAL_FENCE_FD_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME")]
pub const KHR_EXTERNAL_FENCE_FD_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_external_fence_fd");
///# [vkGetFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetFenceFdKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_fd/vkGetFenceFdKHR.md")]
#[doc(alias = "vkGetFenceFdKHR")]
pub type FNGetFenceFdKhr = unsafe extern "system" fn(
    device: Device,
    p_get_fd_info: *const FenceGetFdInfoKHR,
    p_fd: *mut i32,
) -> VulkanResultCodes;
///# [vkImportFenceFdKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkImportFenceFdKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_external_fence_fd/vkImportFenceFdKHR.md")]
#[doc(alias = "vkImportFenceFdKHR")]
pub type FNImportFenceFdKhr =
    unsafe extern "system" fn(device: Device, p_import_fence_fd_info: *const ImportFenceFdInfoKHR) -> VulkanResultCodes;
