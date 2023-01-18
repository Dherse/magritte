use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Bool32, Device, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *mut BaseOutStructure,
    #[doc(alias = "presentWait")]
    present_wait: Bool32,
}
#[doc(alias = "VK_KHR_PRESENT_WAIT_SPEC_VERSION")]
pub const KHR_PRESENT_WAIT_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_PRESENT_WAIT_EXTENSION_NAME")]
pub const KHR_PRESENT_WAIT_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_present_wait");
#[doc(alias = "vkWaitForPresentKHR")]
pub type FNWaitForPresentKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> VulkanResultCodes;
