use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME")]
pub const EXT_ACQUIRE_XLIB_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_acquire_xlib_display");
