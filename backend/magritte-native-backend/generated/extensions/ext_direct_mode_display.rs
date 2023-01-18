use crate::{
    cstr,
    extensions::khr_display::DisplayKHR,
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_direct_mode_display");
#[doc(alias = "vkReleaseDisplayEXT")]
pub type FNReleaseDisplayExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes;
