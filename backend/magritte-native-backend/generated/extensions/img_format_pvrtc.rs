use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_IMG_FORMAT_PVRTC_SPEC_VERSION")]
pub const IMG_FORMAT_PVRTC_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_IMG_FORMAT_PVRTC_EXTENSION_NAME")]
pub const IMG_FORMAT_PVRTC_EXTENSION_NAME: &'static CStr = cstr!("VK_IMG_format_pvrtc");
