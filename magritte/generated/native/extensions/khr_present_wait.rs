use crate::native::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseOutStructure, Bool32, Device, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkPhysicalDevicePresentWaitFeaturesKHR")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhysicalDevicePresentWaitFeaturesKHR {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *mut BaseOutStructure,
    #[doc(alias = "presentWait")]
    pub present_wait: Bool32,
}
impl Default for PhysicalDevicePresentWaitFeaturesKHR {
    fn default() -> Self {
        Self {
            s_type: StructureType::PhysicalDevicePresentWaitFeaturesKhr,
            p_next: unsafe { std::mem::zeroed() },
            present_wait: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::khr_present_wait::{KHR_PRESENT_WAIT_EXTENSION_NAME, KHR_PRESENT_WAIT_SPEC_VERSION};
#[doc(alias = "vkWaitForPresentKHR")]
pub type FNWaitForPresentKhr = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> VulkanResultCodes;
