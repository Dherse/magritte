use crate::{
    cstr,
    extensions::khr_swapchain::SwapchainKHR,
    vulkan1_0::{BaseInStructure, Device, StructureType, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VkRefreshCycleDurationGOOGLE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RefreshCycleDurationGOOGLE {
    #[doc(alias = "refreshDuration")]
    refresh_duration: u64,
}
#[doc(alias = "VkPastPresentationTimingGOOGLE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PastPresentationTimingGOOGLE {
    #[doc(alias = "presentID")]
    present_id: u32,
    #[doc(alias = "desiredPresentTime")]
    desired_present_time: u64,
    #[doc(alias = "actualPresentTime")]
    actual_present_time: u64,
    #[doc(alias = "earliestPresentTime")]
    earliest_present_time: u64,
    #[doc(alias = "presentMargin")]
    present_margin: u64,
}
#[doc(alias = "VkPresentTimesInfoGOOGLE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentTimesInfoGOOGLE {
    #[doc(alias = "sType")]
    s_type: StructureType,
    #[doc(alias = "pNext")]
    p_next: *const BaseInStructure,
    #[doc(alias = "swapchainCount")]
    swapchain_count: u32,
    #[doc(alias = "pTimes")]
    times: *const PresentTimeGOOGLE,
}
#[doc(alias = "VkPresentTimeGOOGLE")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PresentTimeGOOGLE {
    #[doc(alias = "presentID")]
    present_id: u32,
    #[doc(alias = "desiredPresentTime")]
    desired_present_time: u64,
}
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_SPEC_VERSION")]
pub const GOOGLE_DISPLAY_TIMING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_DISPLAY_TIMING_EXTENSION_NAME")]
pub const GOOGLE_DISPLAY_TIMING_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_display_timing");
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
