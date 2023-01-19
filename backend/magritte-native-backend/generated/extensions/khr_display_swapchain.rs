//!# [VK_KHR_display_swapchain](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_display_swapchain.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_display_swapchain/VK_KHR_display_swapchain.md")]
use crate::{
    cstr,
    extensions::khr_swapchain::{SwapchainCreateInfoKHR, SwapchainKHR},
    vulkan1_0::{AllocationCallbacks, BaseInStructure, Bool32, Device, Rect2D, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkDisplayPresentInfoKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkDisplayPresentInfoKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display_swapchain/VkDisplayPresentInfoKHR.md")]
#[doc(alias = "VkDisplayPresentInfoKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DisplayPresentInfoKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "srcRect")]
    src_rect: Rect2D,
    #[doc(alias = "dstRect")]
    dst_rect: Rect2D,
    persistent: Bool32,
}
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION")]
pub const KHR_DISPLAY_SWAPCHAIN_SPEC_VERSION: u32 = 10;
#[doc(alias = "VK_KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME")]
pub const KHR_DISPLAY_SWAPCHAIN_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_display_swapchain");
///# [vkCreateSharedSwapchainsKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateSharedSwapchainsKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_display_swapchain/vkCreateSharedSwapchainsKHR.md")]
#[doc(alias = "vkCreateSharedSwapchainsKHR")]
pub type FNCreateSharedSwapchainsKhr = unsafe extern "system" fn(
    device: Device,
    swapchain_count: u32,
    p_create_infos: *const SwapchainCreateInfoKHR,
    p_allocator: *const AllocationCallbacks,
    p_swapchains: *mut SwapchainKHR,
) -> VulkanResultCodes;
