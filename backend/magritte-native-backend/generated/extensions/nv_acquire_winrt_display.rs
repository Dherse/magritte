//!# [VK_NV_acquire_winrt_display](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_NV_acquire_winrt_display.html)
# ! [doc = include_str ! ("../../../../doc/extensions/nv_acquire_winrt_display/VK_NV_acquire_winrt_display.md")]
use crate::{
    cstr,
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION")]
pub const NV_ACQUIRE_WINRT_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME")]
pub const NV_ACQUIRE_WINRT_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_acquire_winrt_display");
///# [vkAcquireWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkAcquireWinrtDisplayNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_acquire_winrt_display/vkAcquireWinrtDisplayNV.md")]
#[doc(alias = "vkAcquireWinrtDisplayNV")]
pub type FNAcquireWinrtDisplayNv =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes;
///# [vkGetWinrtDisplayNV](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetWinrtDisplayNV.html)
# [doc = include_str ! ("../../../../doc/extensions/nv_acquire_winrt_display/vkGetWinrtDisplayNV.md")]
#[doc(alias = "vkGetWinrtDisplayNV")]
pub type FNGetWinrtDisplayNv = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    device_relative_id: u32,
    p_display: *mut DisplayKHR,
) -> VulkanResultCodes;
