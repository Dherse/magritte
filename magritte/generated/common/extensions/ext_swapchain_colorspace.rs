use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_SPEC_VERSION: u32 = 4;
#[doc(alias = "VK_EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME")]
pub const EXT_SWAPCHAIN_COLOR_SPACE_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_swapchain_colorspace");
