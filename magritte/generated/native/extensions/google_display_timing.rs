pub use crate::common::extensions::google_display_timing::{
    PastPresentationTimingGOOGLE, PresentTimeGOOGLE, RefreshCycleDurationGOOGLE,
};
use crate::native::{
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType, VulkanResultCodes},
};
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    #[doc(alias = "sType")]
    pub s_type: StructureType,
    #[doc(alias = "pNext")]
    pub p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    pub swapchain_count: u32,
    #[doc(alias = "pTimes")]
    pub times: *const PresentTimeGOOGLE,
}
impl Default for PresentTimesInfoGOOGLE {
    fn default() -> Self {
        Self {
            s_type: StructureType::PresentTimesInfoGoogle,
            p_next: unsafe { std::mem::zeroed() },
            swapchain_count: unsafe { std::mem::zeroed() },
            times: unsafe { std::mem::zeroed() },
        }
    }
}
pub use crate::common::extensions::google_display_timing::{
    GOOGLE_DISPLAY_TIMING_EXTENSION_NAME, GOOGLE_DISPLAY_TIMING_SPEC_VERSION,
};
#[doc(alias = "vkGetRefreshCycleDurationGOOGLE")]
pub type FNGetRefreshCycleDurationGoogle = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> VulkanResultCodes;
#[doc(alias = "vkGetPastPresentationTimingGOOGLE")]
pub type FNGetPastPresentationTimingGoogle = unsafe extern "system" fn(
    device: Device,
    swapchain: SwapchainKHR,
    p_presentation_timing_count: *mut u32,
    p_presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> VulkanResultCodes;
