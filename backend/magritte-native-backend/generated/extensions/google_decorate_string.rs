use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_SPEC_VERSION")]
pub const GOOGLE_DECORATE_STRING_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_DECORATE_STRING_EXTENSION_NAME")]
pub const GOOGLE_DECORATE_STRING_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_decorate_string");
