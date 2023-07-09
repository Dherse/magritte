use crate::cstr;
use std::ffi::CStr;
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_SPEC_VERSION")]
pub const KHR_SHADER_ATOMIC_INT64_SPEC_VERSION: u32 = 1;
#[doc(alias = "VK_KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME")]
pub const KHR_SHADER_ATOMIC_INT64_EXTENSION_NAME: &'static CStr = cstr!("VK_KHR_shader_atomic_int64");
