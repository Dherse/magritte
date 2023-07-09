use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const NV_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 2;
#[doc(alias = "VK_NV_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const NV_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static CStr = cstr!("VK_NV_win32_keyed_mutex");
