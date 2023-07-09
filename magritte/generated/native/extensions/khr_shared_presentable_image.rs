use crate::native::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Device, ImageUsageFlags, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkSharedPresentSurfaceCapabilitiesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SharedPresentSurfaceCapabilitiesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "sharedPresentSupportedUsageFlags")]
    pub shared_present_supported_usage_flags: ImageUsageFlags,
}
impl Default for SharedPresentSurfaceCapabilitiesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::SharedPresentSurfaceCapabilitiesKhr,
            p_next: unsafe { std::mem::zeroed() },
            shared_present_supported_usage_flags: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_shared_presentable_image::{
    KHR_SHARED_PRESENTABLE_IMAGE_EXTENSION_NAME, KHR_SHARED_PRESENTABLE_IMAGE_SPEC_VERSION,
};
#[doc(alias = "vkGetSwapchainStatusKHR")]
pub type FNGetSwapchainStatusKhr =
    unsafe extern "system" fn(device: Device, swapchain: SwapchainKHR) -> VulkanResultCodes;
