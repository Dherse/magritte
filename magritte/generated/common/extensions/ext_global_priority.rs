use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_SPEC_VERSION")]
pub const EXT_GLOBAL_PRIORITY_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_EXT_GLOBAL_PRIORITY_EXTENSION_NAME")]
pub const EXT_GLOBAL_PRIORITY_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_global_priority");
