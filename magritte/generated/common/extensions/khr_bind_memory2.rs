use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_BIND_MEMORY_2_SPEC_VERSION")]
pub const KHR_BIND_MEMORY_2_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_BIND_MEMORY_2_EXTENSION_NAME")]
pub const KHR_BIND_MEMORY_2_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_bind_memory2");
