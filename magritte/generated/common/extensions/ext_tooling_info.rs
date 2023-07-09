use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_TOOLING_INFO_SPEC_VERSION")]
pub const EXT_TOOLING_INFO_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_TOOLING_INFO_EXTENSION_NAME")]
pub const EXT_TOOLING_INFO_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_tooling_info");
