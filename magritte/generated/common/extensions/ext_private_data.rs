use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_EXT_PRIVATE_DATA_SPEC_VERSION")]
pub const EXT_PRIVATE_DATA_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_EXT_PRIVATE_DATA_EXTENSION_NAME")]
pub const EXT_PRIVATE_DATA_EXTENSION_NAME: &'static CStr = cstr!("VK_EXT_private_data");
