//!# [VK_KHR_shared_presentable_image](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_KHR_shared_presentable_image.html)
# ! [doc = include_str ! ("../../../../doc/extensions/khr_shared_presentable_image/VK_KHR_shared_presentable_image.md")]
use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Device, ImageUsageFlags, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
///# [VkSharedPresentSurfaceCapabilitiesKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkSharedPresentSurfaceCapabilitiesKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_shared_presentable_image/VkSharedPresentSurfaceCapabilitiesKHR.md")]
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "sharedPresentSupportedUsageFlags")]
    shared_present_supported_usage_flags: ImageUsageFlags,
}
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME")]
pub const KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shared_presentable_image");
///# [vkGetSwapchainStatusKHR](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetSwapchainStatusKHR.html)
# [doc = include_str ! ("../../../../doc/extensions/khr_shared_presentable_image/vkGetSwapchainStatusKHR.md")]
#[doc(alias = "vkGetSwapchainStatusKHR")]
pub type FNGetSwapchainStatusKhr =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
