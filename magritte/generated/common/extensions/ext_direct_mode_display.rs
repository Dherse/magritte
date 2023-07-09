use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION")]
pub const EXT_DIRECT_MODE_DISPLAY_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME")]
pub const EXT_DIRECT_MODE_DISPLAY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_direct_mode_display");
