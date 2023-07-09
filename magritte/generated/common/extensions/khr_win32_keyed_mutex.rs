use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_SPEC_VERSION")]
pub const KHR_WIN32_KEYED_MUTEX_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME")]
pub const KHR_WIN32_KEYED_MUTEX_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_win32_keyed_mutex");
