pub use crate::common::extensions::ext_acquire_xlib_display::{
    EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME, EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION,
};
use crate::native::{
    extensions::khr_display::DisplayKHR,
    opaque::{Display, RROutput},
    vulkan1_0::{PhysicalDevice, VulkanResultCodes},
};
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
