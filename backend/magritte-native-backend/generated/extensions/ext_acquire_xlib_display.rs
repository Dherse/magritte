use crate::{
    cstr,
    extensions::khr_display::DisplayKHR,
    opaque::{Display, RROutput},
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
use std::ffi::CStr;
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_acquire_xlib_display");
#[doc(alias = "vkAcquireXlibDisplayEXT")]
pub type FNAcquireXlibDisplayExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    display: DisplayKHR,
) -> VulkanResultCodes;
#[doc(alias = "vkGetRandROutputDisplayEXT")]
pub type FNGetRandROutputDisplayExt = unsafe extern "system" fn(
    physical_device: PhysicalDevice,
    dpy: *mut Display,
    rr_output: RROutput,
    p_display: *mut DisplayKHR,
) -> VulkanResultCodes;
