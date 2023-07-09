pub use crate::common::extensions::nv_acquire_winrt_display::{
    NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME, NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION,
};
use crate::native::{
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
#[doc(alias = "vkAcquireWinrtDisplayNV")]
pub type FNAcquireWinrtDisplayNv =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes;
#[doc(alias = "vkGetWinrtDisplayNV")]
pub type FNGetWinrtDisplayNv = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut DisplayKHR,
) -> VulkanResultCodes;
