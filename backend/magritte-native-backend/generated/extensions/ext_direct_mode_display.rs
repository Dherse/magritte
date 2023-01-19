//!# [VK_EXT_direct_mode_display](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VK_EXT_direct_mode_display.html)
# ! [doc = include_str ! ("../../../../doc/extensions/ext_direct_mode_display/VK_EXT_direct_mode_display.md")]
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
///# [vkReleaseDisplayEXT](https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkReleaseDisplayEXT.html)
# [doc = include_str ! ("../../../../doc/extensions/ext_direct_mode_display/vkReleaseDisplayEXT.md")]
#[doc(alias = "vkReleaseDisplayEXT")]
pub type FNReleaseDisplayExt =
    unsafe extern "system" fn(physical_device: PhysicalDevice, display: DisplayKHR) -> VulkanResultCodes;
