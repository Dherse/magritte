use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_GOOGLE_USER_TYPE_SPEC_VERSION")]
pub const GOOGLE_USER_TYPE_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_GOOGLE_USER_TYPE_EXTENSION_NAME")]
pub const GOOGLE_USER_TYPE_EXTENSION_NAME: &'static CStr = cstr!("VK_GOOGLE_user_type");
