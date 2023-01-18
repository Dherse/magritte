use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_IMG_FILTER_CUBIC_SPEC_VERSION")]
pub const IMG_FILTER_CUBIC_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_IMG_FILTER_CUBIC_EXTENSION_NAME")]
pub const IMG_FILTER_CUBIC_EXTENSION_NAME: &'static CStr = cstr!("VK_IMG_filter_cubic");
